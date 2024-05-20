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
}
