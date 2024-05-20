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
            break 100;
        }
    };

    println!("El resultado de la iteracion es {resultado}");
}
