mod accounts;
mod core;

use accounts::controller::{handle_add, handle_list};
use clap::{Parser, Subcommand};
use core::db::{build_db_path, init_db};
use rusqlite::Connection;

#[derive(Parser)]
#[command(name = "money-manager")]
#[command(about = "Controle simples de contas no terminal", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Cadastra uma conta com nome e valor
    Add {
        #[arg(short, long)]
        name: String,
        #[arg(short, long, help = "Valor com duas casas, ex: 123.45")]
        value: String,
    },
    /// Lista todas as contas cadastradas
    List,
}

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
    }
}
