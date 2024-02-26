# Foxsur RS 🔋 + 🦀 (Sync edition)

An implementation of a service which collect information about centralized crypto exchanges in Rust

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
                "DATABASE_USERNAME": "root",
                "DATABASE_PASSWORD": "password",
                "DATABASE_NAME": "postgres",
                "SOURCE": "deribit",
                "MAX_CON": "10",
                "AUTO_MAP": "true",
                "RUST_LOG": "info"
            }
        }
    ]
}
```

## Running locally

If you do not use vscode you may use these environment variables below:

```sh
export DATABASE_HOST="localhost"
export DATABASE_USERNAME="root"
export DATABASE_PASSWORD="password"
export DATABASE_NAME="postgres"
export SLACK_BOT_TOKEN="foo"
export SLACK_CHANNEL_ID="foo"
export SOURCE="deribit"
export MAX_CON="10"
export AUTO_MAP="true"
export RUST_LOG="info"
```

After exporting these environment variables you can run foxsur-rs with these commands

```sh
cargo run
```
