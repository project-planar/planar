use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use console::{style, Emoji};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use miette::{Context, miette};
use planar_pkg::config::PlanarContext;
use planar_pkg::model::planardl::PackageManifest;
use planar_pkg::packaging::resolver::{ResolverProgress, WorkspaceResolver};
use planar_pkg::parser::ctx::ParseContext;
use planar_pkg::parser::parsable::KdlParsable;
use planarc::compiler::Compiler;
use planarc::artifact::builder::create_bundle;
use planarc::artifact::writer::write_bundle;
use planarc::module_loader::FsModuleLoader;

static TICK: Emoji<'_, '_> = Emoji("✔ ", "");
static HAMMER: Emoji<'_, '_> = Emoji("🔨 ", "");

pub fn run(path: PathBuf, is_tracing: bool) -> miette::Result<()> {
    
    let start_time = Instant::now();

    let ctx = PlanarContext::new();
    let progress = CliProgress::new(is_tracing);
    
    let mut resolver = WorkspaceResolver::new(ctx, &progress);
    resolver.resolve(path.clone()).map_err(|e| miette!(e))?;
    
    let root_manifest = load_manifest(&path)?;
    let package_name = root_manifest.package.name.clone();
    let pkg_count = resolver.packages.len();
    
    progress.main_pb.finish_with_message(format!(
        "Dependencies resolved for {}", 
        style(&package_name).bold().green()
    ));

    
    let roots = resolver.get_roots_for_compiler();
    let compiler = Compiler::new(FsModuleLoader);

    println!("{} {}Compiling...", style("planar").bold().cyan(), HAMMER);

    let result = compiler.compile(roots)
        .with_context(|| format!("Compilation failed for {}", package_name))?;

    if result.has_errors() {
        
        for error in &result.errors.0 { 
            eprintln!("{:?}", error);
        }

        let error_count = result.errors.0.len();

        eprintln!(
            "\n{} with {} {}",
            style("Build failed"),
            style(error_count).red().bold(),
            if error_count == 1 { "error" } else { "errors" }
        );

        std::process::exit(1);
    }

    let program = create_bundle(result);

    let target_dir = path.join("target");
    if !target_dir.exists() {
        fs::create_dir_all(&target_dir).map_err(|e| miette!(e))?;
    }
    
    let output_path = target_dir.join(format!("{}.pdla", package_name));
    let mut buffer = Vec::new();
    write_bundle(&program, &mut buffer, None).map_err(|e| miette!(e))?;
    fs::write(&output_path, &buffer).map_err(|e| miette!(e))?;

    let duration = start_time.elapsed();

    let packages = format!("{} package{}", pkg_count, if pkg_count == 1 { "" } else { "s" });

    println!(
        "{} {} {} {} in {:?}\n  {}  Artifact: {}",
        style("planar").bold().cyan(),
        TICK,
        style("Finished").green().bold(),
        packages,
        duration,
        style("➜").dim(),
        style(output_path.display()).bold()
    );


    Ok(())
}


fn load_manifest(path: &PathBuf) -> miette::Result<PackageManifest> {
    let file_path = path.join("planar.kdl");
    let content = fs::read_to_string(&file_path)
        .map_err(|_| miette!("Missing planar.kdl in {:?}", path))?;
    let doc = content.parse::<kdl::KdlDocument>()?;
    let ctx = ParseContext::new(doc, &file_path.to_string_lossy());
    PackageManifest::parse_node(&ctx, &()).map_err(|e| miette!("Invalid manifest: {:?}", e))
}

struct CliProgress {
    multi: MultiProgress,
    main_pb: ProgressBar,
}

impl CliProgress {
    fn new(is_verbose: bool) -> Self {
        let multi = MultiProgress::new();
        let main_pb = multi.add(ProgressBar::new_spinner());
        main_pb.set_style(ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap());

        if is_verbose {
            main_pb.set_draw_target(indicatif::ProgressDrawTarget::hidden());
        } else {
            main_pb.set_style(ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap());
            main_pb.enable_steady_tick(std::time::Duration::from_millis(100));
        }


        Self { multi, main_pb }
    }
}

impl ResolverProgress for CliProgress {
    fn on_start_resolve(&self, name: &str) {
        self.main_pb.set_message(format!("Resolving {}...", style(name).bold()));
    }
    fn on_fetch_start(&self, name: &str, ver: &str) {
        self.multi.println(format!("  {} Fetching {} ({})", style("➜").blue(), name, ver)).unwrap();
    }
    fn on_fetch_done(&self, _name: &str) {}
    fn on_error(&self, msg: &str) {
        self.multi.println(format!("  {} {}", style("✘").red(), msg)).unwrap();
    }
}