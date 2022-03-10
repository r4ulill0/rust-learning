use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("\n\n¡¡Adivina el número!!\n\n");
    let numero_secreto = rand::thread_rng().gen_range(1..1000);

    loop {
        println!("Por favor introduce el número que crees que será:\n");


        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Fallo al leer la línea");
        println!("Has propuesto: {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        match guess.cmp(&numero_secreto) {
            Ordering::Less => println!("Demasiado pequeño!"),
            Ordering::Greater => println!("Demasiado GRANDE!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
    println!("El número secreto era: {}", numero_secreto);
}
