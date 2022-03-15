use std::env;
use std::process;

use minigrep::Configuracion;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Configuracion::new(&args).unwrap_or_else(|err| {
        println!("Problema al parsear los argumentos: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Error de minigrep: {}", e);

        process::exit(1);
    }
}
