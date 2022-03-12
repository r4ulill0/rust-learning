use std::io;

pub fn ejecutar(){
    println!("Bienvenido a la calculadora de fibonacci\n");
    println!("Introduce cuánto de larga quieres que sea la sucesión de fibonacci:\n");

    let mut iteraciones = String::new();
    io::stdin().read_line(&mut iteraciones)
        .expect("No se pudo leer las iteraciones correctamente");

    let iteraciones: u64 = iteraciones.trim().parse()
        .expect("Fallo al convertir entrada a número");

    let resultado = fibonacci(iteraciones);
    println!("El resultado es {}", resultado);
}

fn fibonacci(numero: u64) -> u64 {

    if numero == 0 { 0 }
    else if numero == 1 { 1 }
    else {
        fibonacci(numero-1) + fibonacci(numero - 2)
    }
}
