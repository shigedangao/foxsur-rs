# Foxsur RS 🔋 + 🦀

An implementation of foxsur in Rust to demonstrate how it'd looks like (sort of as it's quick and dirty rendering for now...)

## Launching in vscode

Add the following configurations in the `launch.json` file. Please use the `docker-compose.yaml` located in the `pkg/foxsur`

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "debug foxsur",
            "program": "${workspaceFolder}/target/debug/${workspaceFolderBasename}",
            "preLaunchTask": "rust: cargo build",
            "console": "integratedTerminal",
            "cwd": "${workspaceFolder}",
            "env": {
                "database_host": "localhost",
                "database_username": "exchanges-metadata-api",
                "database_password": "password",
                "database_database": "metadata",
                "slack_bot_token": "foo",
                "slack_channel_id": "foo",
                "max_con": "10",
                "auto_map": "true",
                "RUST_LOG": "info"
            }
        }
    ]
}
```
