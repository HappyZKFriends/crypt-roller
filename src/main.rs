use crypt_roller::cli::run_cli;

fn main() {
    if let Err(error) = run_cli() {
        println!("ERROR: {:#?}", error);
    }
}
