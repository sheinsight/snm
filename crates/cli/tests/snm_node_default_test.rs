use clap::Parser;
use cli::{manage_command::ManageCommands, snm_command::SnmCommands, SnmCli};

#[tokio::test]
async fn test_node_default_v18() {
    let cli = SnmCli::parse_from(["snm", "node", "default", "v18.12.2"]);
    match cli.command {
        SnmCommands::Node { command } => match command {
            ManageCommands::Default { version } => {
                assert_eq!(version, "v18.12.2");
            }
            _ => panic!("expect  ManageCommands::Default"),
        },
        _ => panic!("expect SnmCommands::Node"),
    }
}

#[tokio::test]
async fn test_node_default_18() {
    let cli = SnmCli::parse_from(["snm", "node", "default", "18.12.2"]);
    match cli.command {
        SnmCommands::Node { command } => match command {
            ManageCommands::Default { version } => {
                assert_eq!(version, "18.12.2");
            }
            _ => panic!("expect  ManageCommands::Default"),
        },
        _ => panic!("expect SnmCommands::Node"),
    }
}

#[tokio::test]
async fn test_node_default_error_version() {
    let cli = SnmCli::parse_from(["snm", "node", "default", "error-version"]);
    match cli.command {
        SnmCommands::Node { command } => match command {
            ManageCommands::Default { version } => {
                assert_eq!(version, "error-version");
            }
            _ => panic!("expect  ManageCommands::Default"),
        },
        _ => panic!("expect SnmCommands::Node"),
    }
}
