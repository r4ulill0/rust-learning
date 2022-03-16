use std::error::Error;
use std::fs;
use std::env;

pub struct Configuracion {
    archivo: String,
    busqueda: String,
    mayus_sensible: bool
}

impl Configuracion {
    pub fn new(mut args: env::Args) -> Result<Configuracion, &'static str> {
        let busqueda = match args.next() {
            Some(b) => b,
            None => return Err("No se ha especificado el término de búsqueda"),
        };

        let archivo = match args.next() {
            Some(f) => f,
            None => return Err("No se ha especificado el archivo a analizar"),
        };

        let mayus_sensible = env::var("MAYUS_INSENSIBLE").is_err();
        Ok(Configuracion { busqueda, archivo, mayus_sensible })
    }
}

pub fn run(config: Configuracion) -> Result<(), Box<dyn Error>> {
    let contenido = fs::read_to_string(config.archivo)?;

    let resultado: Vec<&str> = if config.mayus_sensible {
        busca(&config.busqueda, &contenido)
    } else {
        busca_insensible(&config.busqueda, &contenido)
    };

    for linea in resultado {
        println!("{}", linea);
    }

    Ok(())
}

pub fn busca<'a>(busqueda: &str, contenido: &'a str) -> Vec<&'a str> {

    contenido
        .lines()
        .filter(|linea| linea.contains(busqueda))
        .collect()
}

pub fn busca_insensible<'a>(busqueda: &str, contenido: &'a str) -> Vec<&'a str> {
    let busqueda = busqueda.to_lowercase();

    contenido
        .lines()
        .filter(|linea| linea.to_lowercase().contains(&busqueda))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn un_resultado() {
        let busqueda = "duct";
        let contenido = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], busca(busqueda, contenido));
    }

    #[test]
    fn insensible_mayus() {
        let busqueda = "rUst";
        let contenido = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:","Trust me."],
            busca_insensible(busqueda, contenido));
    }
}
