use clap::Parser;


/// Perfect secrecy for everyone!
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
   /// Encrypt a message.
   #[arg(short, long, default_value_t = true)]
   encrypt: bool,

   /// Decrypt a message.
   #[arg(short, long, default_value_t = false)]
   decrypt: bool,

   /// Generate a key.
   #[arg(short, long, default_value_t = false)]
   generate: bool,

   /// Key file.
   #[arg(short, long, default_value_t = String::from("key"))]
   key: String,

   /// Message file.
   #[arg(short, long, default_value_t = String::from("message"))]
   message: String,

   /// Message/key size in megabytes.
   #[arg(short, long, default_value_t = 5)]
   size: u32,
}


fn main() {
   Arguments::parse();
}
