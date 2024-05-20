fn main() {
    let numero = 6;

    if numero % 4 == 0 {
        println!("El numero es divisible entre 4");
    } else if numero % 3 == 0 {
        println!("El numero es divisible entre 3");
    } else if numero % 2 == 0 {
        println!("El numero es divisible entre 2");
    } else {
        println!("el numero no es divisible ni por 4, 3, o 2");
    }
    
    let condicion = true;
    let numero = if condicion { 5 } else { 6 };
    
    // Esto no compila por que los valores que podria devolver el if 
    // no son el mismo uno es un i32 y el otro es un string
    // let numero = if condicion { 3 } else { "seis" };

    println!("El valor de numero es: {numero}");
    
    let mut contador = 0;

    let resultado = loop {
        contador += 1;

        if contador == 10 {
            break contador * 2 
        }
    };

    println!("El resultado de la iteracion es {resultado}");

    let mut contador = 0;
   
    // Puedes ponerle una etiqueta a tu bucle
    // para que pueda posteriormente usar break o continue
    // con la etiqueta para poder decidir cual de lso bucles andidados
    // deseas romper
    'contador_arriba: loop {
        println!("contador = {contador}");
        let mut remanente = 10;
        loop {
            println!("remanente = {remanente}");
            if remanente == 9 {
                break;
            }
            if contador == 2 {
                break 'contador_arriba;
            }
            remanente -= 1;
        }

    contador += 1;
    }
    println!("Final del contador = {contador}");

    let mut numero = 3;
    
    while numero != 0 {
        println!("{numero}!");

        numero -= 1;
    }
    println!("LISTO!!");

    let a = [10, 20, 30, 40, 50];

    for elemento in a {
        println!("El valor es: {elemento}");
    }

    for numero in (1..4).rev() {
        println!("{numero}!");
    }
    println!("LIFTOFF!!!");
}

