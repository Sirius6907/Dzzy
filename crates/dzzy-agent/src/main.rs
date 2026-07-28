fn main() {
    if let Err(e) = dzzy_agent::run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
