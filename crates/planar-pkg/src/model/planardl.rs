use planar_config_macro::{planar_node, NodeSchema, Parser};

#[planar_node]
#[derive(Parser, Clone, Debug, NodeSchema)]
#[node(root)]
pub struct PackageManifest {
    #[node(child)]
    pub package: PackageInfo,

    #[node(child)]
    pub dependencies: Option<DependenciesDef>,
}

#[planar_node]
#[derive(Parser, Clone, Debug, NodeSchema)]
pub struct PackageInfo {
    #[node(child)]
    pub name: String,
    #[node(child)]
    pub version: String,
}

#[planar_node]
#[derive(Parser, Clone, Debug, NodeSchema, Default)]
#[node(name = "dependencies")]
pub struct DependenciesDef {
    #[node(dynamic_child)]
    pub items: Vec<DependencyItemDef>,
}

#[planar_node]
#[derive(Parser, Clone, Debug, NodeSchema)]
pub struct DependencyItemDef {
    #[node(node_name)]
    pub name: String,
    
    #[node(prop)]
    pub path: Option<String>,
    
    #[node(prop)]
    pub github: Option<String>,
    
    #[node(prop)]
    pub branch: Option<String>,
    
    #[node(prop)]
    pub tag: Option<String>,
}
