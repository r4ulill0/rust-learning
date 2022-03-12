pub fn ejecutar() {
    println!("Bienvenido al reproductor de canciones");


    let posiciones = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];
    let numeros = [
        "a", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Eleven",
        "Twelve",
    ];
    let regalos = [
        "partridge in a pear tree",
        "turtle doves",
        "calling birds",
        "golden rings",
        "geese-a-laying",
        "swans-a-swimming",
        "maids-a-milking",
        "ladies dancing",
        "lords-a-leaping",
        "drummers drumming",
        "pipers piping",
        "French hens",
    ];

    let mut day = 1;
    while day <= 12 {
        println!("On the {} day of Christmas\nMy true love gave to me", posiciones[day-1]);
        let mut cuenta_atras = day;
        while cuenta_atras > 0 {
            cuenta_atras -= 1;
            if cuenta_atras == 0 && day != 1 {
                println!("and {} {}", numeros[cuenta_atras], regalos[cuenta_atras]);
            } else {
                println!("{} {}", numeros[cuenta_atras], regalos[cuenta_atras]);
            }
        }
        println!("\n");
        day += 1;
    }
}
