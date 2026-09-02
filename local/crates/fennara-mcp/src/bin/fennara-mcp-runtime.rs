#[path = "../runtime/mod.rs"]
mod runtime;

fn main() {
    match runtime::RuntimeConfig::from_process() {
        Ok(config) => runtime::run_stdio(config),
        Err(error) => {
            eprintln!("fennara-mcp-runtime failed: {error}");
            std::process::exit(2);
        }
    }
}
