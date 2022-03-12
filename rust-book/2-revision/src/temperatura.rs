use std::io;
pub fn ejecutar() {
    let mut intro = "Bienvenido al conversor de temperaturas\n".to_owned();
    intro.push_str("Selecciona la conversión que desees:\n");
    intro.push_str("\n1. Celsius to farenheit\n");
    intro.push_str("2. farenheit a Celsius\n");
    intro.push_str("3. Volver al menu principal\n");
    println!("{}", intro);

    let mut seleccion = String::new();
    io::stdin().read_line(&mut seleccion).expect("Algo salió mal");

    println!("\n\n Introduce la cantidad a convertir:\n");

    let mut grados = String::new();
    io::stdin().read_line(&mut grados).expect("Algo salió mal");

    let grados: f64 = grados.trim()
        .parse()
        .expect("Fallo al convertir la entrada de grados");

    let seleccion: u8 = seleccion.trim()
        .parse()
        .expect("Fallo al convertir seleccion a número");

    let resultado = match seleccion {
        1 => (celsius_a_farenheit(grados), 'F'),
        2 => (farenheit_a_celsius(grados), 'C'),
        _ => (0.0, '?')
    };

    println!("La conversión son {} º{}", resultado.0, resultado.1);
}

fn farenheit_a_celsius(grados:f64) -> f64 {
    (grados - 32.0) * 5.0/9.0
}

fn celsius_a_farenheit(grados:f64) -> f64 {
    grados * 9.0 / 5.0 + 32.0
}
