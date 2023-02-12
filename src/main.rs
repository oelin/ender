use clap::{Parser, Subcommand};
mod commands;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {

        #[command(subcommand)]
        command: Commands,
}


#[derive(Subcommand, Debug)]
enum Commands {

        /// Decrypt a message using an Esther key.
        Decrypt {
                /// Path of the key.
                #[arg(short, long, value_name = "PATH")]
                key: String,

                /// Path of the message.
                #[arg(short, long, value_name = "PATH")]
                message: String,
        },

        /// Encrypt a message using an Esther key.
        Encrypt {
                /// Path of the key.
                #[arg(short, long, value_name = "PATH")]
                key: String,

                /// Path of the message.
                #[arg(short, long, value_name = "PATH")]
                message: String,
        },

        /// Create a new Esther key.
        New {
                /// Path of the key.
                path: String,

                /// Size of the key in kB.
                #[arg(short, long, default_value_t = 1000)]
                size: u32,
        },
}


fn main() {
        let cli = Cli::parse();

        let command = match &cli.command {
                Commands::Decrypt {..} => commands::decrypt,
                Commands::Encrypt {..} => commands::encrypt,
                Commands::New {..} => commands::new,
        };

        command()//(&cli.command);
}
