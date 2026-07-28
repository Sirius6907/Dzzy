#[tokio::main]
async fn main() {
    std::process::exit(dzzy_cli::run_from_args(std::env::args()).await);
}
