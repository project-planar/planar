import * as path from 'path';
import { workspace, ExtensionContext } from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    Executable
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: ExtensionContext) {
    
    const serverPath = context.asAbsolutePath(path.join('bin', 'planar-lsp'));
    
    const run: Executable = {
        command: serverPath,
        options: {
            env: {
                ...process.env,
                "RUST_LOG": "debug"
            }
        }
    };

    const serverOptions: ServerOptions = {
        run: run,
        debug: run
    };

    const clientOptions: LanguageClientOptions = {
        
        documentSelector: [{ scheme: 'file', language: 'pdl' }],
        synchronize: {
            fileEvents: workspace.createFileSystemWatcher('**/*.pdl')
        }
    };

    client = new LanguageClient(
        'pdlLanguageServer',
        'Planardl Language Server',
        serverOptions,
        clientOptions
    );

    client.start();
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}