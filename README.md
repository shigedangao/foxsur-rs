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
                "DATABASE_HOST": "localhost",
                "DATABASE_USERNAME": "exchanges-metadata-api",
                "DATABASE_PASSWORD": "password",
                "DATABASE_NAME": "metadata",
                "SLACK_BOT_TOKEN": "foo",
                "SLACK_CHANNEL_ID": "foo",
                "SOURCE": "paxos",
                "MAX_CON": "10",
                "AUTO_MAP": "true",
                "RUST_LOG": "info ./main"
            }
        }
    ]
}
```
