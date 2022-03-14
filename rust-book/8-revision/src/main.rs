use std::collections::HashMap;
use std::io;
fn main() {
    vector_exercise();
    strings_exercise();
    hashmap_exercise();
}

fn vector_exercise() {
    let mut some_integers = vec![1, 4, 5, 6, 3, 4, 6, 9, 8, 3, 6, 0, 7, 5, 6, 4, 2, 5, 7];
    some_integers[..].sort();
    let moda = some_integers[some_integers.len() / 2 - 1];

    let mut total = 0;
    for num in &some_integers {
        total += num;
    }
    let media = total / some_integers.len();

    println!("Moda: {} Media: {}", moda, media);
}

fn strings_exercise() {
    let input = vec!["first", "apple"];
    for palabra in input {
        let inicial = palabra.chars().next();
        let resultado = match inicial {
            None => format!("{}", &palabra[..]),
            Some('a') | Some('e') | Some('i') | Some('o') | Some('u') => {
                format!("{}-hay", &palabra[..])
            }
            Some(sufix) => format!("{}-{}ay", &palabra[1..], sufix),
        };
        println!("{}", resultado);
    }
}

fn hashmap_exercise() {
    let mut catalog_empleados = HashMap::new();
    loop {
        println!("\n\n1.Añdadir empleado");
        println!("2. Listado de empleados por departamento");
        println!("Seleccione una opción: \n");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Hubo un error al procesar la entrada");

        println!("Input de seleccion USADO:{}",&input);
        let seleccion: i32 = input
            .trim()
            .parse()
            .expect("No se selecciono una opcion valida");
        match seleccion {
            1 => guardar_empleado(&mut catalog_empleados),
            2 => {
                println!("Introduzca departamento");

                let mut inputo = String::new();
                io::stdin()
                    .read_line(&mut inputo)
                    .expect("Hubo un error al procesar la entrada");
                let clave = inputo.trim();
                let clave = String::from(clave);
                let lista_empleados = catalog_empleados.get_mut(&clave);
                println!("Lista desordenada de empleados {:?}",lista_empleados);
                match lista_empleados {
                    Some(lista) => {
                        lista.sort();
                        for nombre in lista {
                            println!("{}", nombre);
                        }
                    }
                    None => (),
                };
            }
            _ => (),
        }
    }
}

fn guardar_empleado(catalog_empleados: &mut HashMap<String,Vec<String>>) {
    println!("Introduzca {{empleado}}' '{{departamento}}");
    
    let mut empleado_input = String::new();
    io::stdin()
        .read_line(&mut empleado_input)
        .expect("Hubo un error al procesar la entrada");
    
    let entrada_tratada: Vec<&str> = empleado_input.split_whitespace().collect();
    let empleado;
    let departamento;
    if entrada_tratada.len() > 1 {
        empleado = entrada_tratada[0];
        departamento = entrada_tratada[1].trim();
    } else {
        empleado = "Error";
        departamento = "Error"
    }
    println!("Entrada tratada: {:?}", entrada_tratada);
    let empleado = empleado.to_owned();
    let departamento = departamento.to_owned();
    println!("Departamento: {} empleado: {}", &departamento, &empleado);
    let vec_empleados = catalog_empleados
        .entry(departamento)
        .or_insert(Vec::new());
    vec_empleados.push(empleado);
}
