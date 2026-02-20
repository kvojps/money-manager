mod accounts;
mod core;
mod transactions;
use accounts::controller::{handle_add, handle_list};
use clap::{Parser, Subcommand};
use core::db::{build_db_path, init_db};
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
    Add {
        #[arg(short, long)]
        name: String,
        #[arg(short, long, help = "Valor com duas casas, ex: 123.45")]
        value: String,
    },
    List,
    Tx {
        #[command(subcommand)]
        command: TxCommands,
    },
}
#[derive(Subcommand)]
enum TxCommands {
    Add {
        #[arg(short, long, help = "ID da conta")]
        account_id: i64,
        #[arg(short, long, help = "Tipo de transação: 'add' ou 'subtract'")]
        tx_type: String,
        #[arg(short, long, help = "Valor com duas casas, ex: 123.45")]
        value: String,
        #[arg(short, long, help = "Descrição da transação (opcional)")]
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
        Commands::Add { name, value } => match handle_add(&conn, &name, &value) {
            Ok(output) => println!("{output}"),
            Err(err) => {
                eprintln!("{err}");
                std::process::exit(1);
            }
        },
        Commands::List => match handle_list(&conn) {
            Ok(output) => println!("{output}"),
            Err(err) => {
                eprintln!("{err}");
                std::process::exit(1);
            }
        },
        Commands::Tx { command } => match command {
            TxCommands::Add {
                account_id,
                tx_type,
                value,
                description,
            } => match handle_add_transaction(&conn, account_id, &tx_type, &value, description) {
                Ok(output) => println!("{output}"),
                Err(err) => {
                    eprintln!("{err}");
                    std::process::exit(1);
                }
            },
        },
    }
}
