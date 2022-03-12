use std::io;
mod fibonacci;
mod temperatura;
mod cancion;
fn main() {
    let mut bienvenida = "Bienvenido al programa de repaso de la leccion 2\n".to_owned();
    bienvenida.push_str("Esta lección repasa el marco común del lenguaje (Rust) con otros\n");
    bienvenida.push_str(" los temas tratados son los siguientes:\n");
    bienvenida.push_str("Variables y su mutabilidad\nTipos de datos\n Funciones\nControl de flujo");
    bienvenida.push_str("\n\nPara repasar estos conceptos hay 3 programas de práctica\n");
    bienvenida.push_str("Selecciona el programa que quieras probar:\n" );
    bienvenida.push_str("1. Conversor celsius-farenheit\n" );
    bienvenida.push_str("2. Calculo de fibonacci\n" );
    bienvenida.push_str("3. Cancion de Twelve days of Christmas\n" );
    bienvenida.push_str("4. Salir\n" );
    bienvenida.push_str("Introduce el número de la opción para seleccionarla:\n");

    loop {
    println!("{}", bienvenida);

    let mut seleccion = String::new();
    io::stdin()
        . read_line(&mut seleccion)
        .expect("No estaba contemplada esa entrada");

    let seleccion: u8 = seleccion.trim().parse().expect("Solo números");
    match seleccion {
        1 => temperatura::ejecutar(), // Llamar a la clase de temperaturas
        2 => fibonacci::ejecutar(), // Llamar a la clase fibonacci
        3 => cancion::ejecutar(), // Llamar a la clase cancion
        4 => println!("opcion 4"),
        _ => println!("opcion imposible")
    }

    }
}
