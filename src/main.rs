mod accounts;
mod core;
mod transactions;
use accounts::controller::{handle_add, handle_list};
use clap::{Parser, Subcommand};
use core::db::{build_db_path, init_db};
use core::utils::parse_account_id;
use rusqlite::Connection;
use transactions::controller::handle_add_transaction;

// CLI
#[derive(Parser)]
#[command(name = "money-manager")]
#[command(about = "Controle simples de contas no terminal", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// Commands
#[derive(Subcommand)]
enum Commands {
    #[command(alias = "acc")]
    Account {
        #[command(subcommand)]
        cmd: AccountCommands,
    },
    Tx {
        #[command(subcommand)]
        cmd: TxCommands,
    },
}

#[derive(Subcommand)]
enum AccountCommands {
    Add {
        name: String,
        #[arg(help = "Valor com duas casas, ex: 123.45")]
        value: String,
    },
    #[command(alias = "ls")]
    List,
}

#[derive(Subcommand)]
enum TxCommands {
    #[command(name = "in")]
    In {
        #[arg(help = "ID da conta ou nome da conta")]
        account: String,
        #[arg(help = "Valor com duas casas, ex: 123.45")]
        value: String,
    },
    #[command(name = "out")]
    Out {
        #[arg(help = "ID da conta ou nome da conta")]
        account: String,
        #[arg(help = "Valor com duas casas, ex: 123.45")]
        value: String,
        #[arg(short, long, help = "Descrição da transação")]
        description: Option<String>,
    },
}

// Entrypoint
fn main() {
    let cli = Cli::parse();
    let db_path = match build_db_path() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Erro ao preparar o caminho do banco: {err}");
            std::process::exit(1);
        }
    };
    let mut conn = match Connection::open(db_path) {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Erro ao abrir o banco: {err}");
            std::process::exit(1);
        }
    };
    if let Err(err) = init_db(&mut conn) {
        eprintln!("Erro ao inicializar o banco: {err}");
        std::process::exit(1);
    }

    match cli.command {
        Commands::Account { cmd } => match cmd {
            AccountCommands::Add { name, value } => match handle_add(&conn, &name, &value) {
                Ok(output) => println!("{output}"),
                Err(err) => {
                    eprintln!("{err}");
                    std::process::exit(1);
                }
            },
            AccountCommands::List => match handle_list(&conn) {
                Ok(output) => println!("{output}"),
                Err(err) => {
                    eprintln!("{err}");
                    std::process::exit(1);
                }
            },
        },
        Commands::Tx { cmd } => match cmd {
            TxCommands::In { account, value } => {
                match parse_account_id(&account) {
                    Ok(account_id) => {
                        match handle_add_transaction(&conn, account_id, "add", &value, None) {
                            Ok(output) => println!("{output}"),
                            Err(err) => {
                                eprintln!("{err}");
                                std::process::exit(1);
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("{err}");
                        std::process::exit(1);
                    }
                }
            }
            TxCommands::Out {
                account,
                value,
                description,
            } => {
                match parse_account_id(&account) {
                    Ok(account_id) => {
                        match handle_add_transaction(&conn, account_id, "subtract", &value, description) {
                            Ok(output) => println!("{output}"),
                            Err(err) => {
                                eprintln!("{err}");
                                std::process::exit(1);
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("{err}");
                        std::process::exit(1);
                    }
                }
            }
        },
    }
}
