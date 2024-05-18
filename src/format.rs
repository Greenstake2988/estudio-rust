fn main() {
    // el {} se remplaza por el argumento
    println!("{} days", 31);

    // Se pueden usar argumentos poscicionales para especificar el argumento
    // Empieza en 0 como el primero
    println!("{0}, esto es {1}. {1}, this is {0}", "Alice", "Bob");
    
    // Tambien se pueden usar nombres para los argumentos
    println!("{sujeto} {verbo} {sustantivo}",
            sustantivo="el perro flojo",
            sujeto="el zorro rapido cafe",
            verbo="salta encima de");
    
    // Se pueden invocar diferentes formatos usando : 
    println!("Base 10:               {}",   69420);
    println!("Base 2 (binario):      {:b}", 69420);
    println!("Base 8 (octal):        {:o}", 69420);
    println!("Base 16 (hexadecimal): {:x}", 69420);
    
    // Puedes formatear el texto con un ancho especifico
    // este le pondra 4 espacios vacios mas el valor 1
    // con un total de 5
    println!("{numero:>5}", numero=1);
}
