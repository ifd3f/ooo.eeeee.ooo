use clap::Parser;

mod cli;
mod precompress;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    match cli::Args::parse() {
        cli::Args::Precompress(a) => {
            
        },
        cli::Args::Server(_) => todo!(),
    }
}
