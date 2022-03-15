use std::error::Error;
use std::fs;
use std::env;

pub struct Configuracion {
    archivo: String,
    busqueda: String,
    mayus_sensible: bool
}

impl Configuracion {
    pub fn new(args: &[String]) -> Result<Configuracion, &'static str> {
        if args.len() < 3 {
            return Err("Argumentos insuficientes");
        }
        let busqueda = args[1].clone();
        let archivo = args[2].clone();
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
    let mut resultado = vec![];

    for linea in contenido.lines() {
        if linea.contains(busqueda) {
            resultado.push(linea);
        }
    }

    return resultado;
}

pub fn busca_insensible<'a>(busqueda: &str, contenido: &'a str) -> Vec<&'a str> {
    let mut resultado = vec![];
    let busqueda = busqueda.to_lowercase();

    for linea in contenido.lines() {
        if linea.to_lowercase().contains(&busqueda.to_lowercase()) {
            resultado.push(linea);
        }
    }

    return resultado;
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
