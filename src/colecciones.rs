
fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let tercero: &i32 = &v[2];
    println!("El tercer elemento es {tercero}");

    let tercero = v.get(2);
    match tercero {
        None => println!("No existe el tercer elemento."),
        Some(tercero) => println!("El tercer elemento es {tercero}")
    }
    let v = vec![1, 2, 3, 4, 5];
    //let no_existe = &v[100];
    let no_existe = v.get(100);

    let v = vec![100, 32, 57];
    for i in &v{
        println!("{i}")
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    enum HojaDeCalculo {
        Entero(i32),
        Flotante(f64),
        Texto(String),
    }

    let fila = vec![
        HojaDeCalculo::Entero(3),
        HojaDeCalculo::Texto(String::from("azul")),
        HojaDeCalculo::Flotante(10.12),
    ];

    let mut s = String::new();
    s = "Hola".to_string();
    s.push_str("Mundo!");
    s.push('!');

    println!("{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    println!("{s3}");

    let hola = "hola";
    let h = &hola[0..1];
    println!("{h}");

    for c in "नमस्ते".chars() {
        println!("{c}");
    }

    for b in "3д".bytes() {
        println!("{b}");
    }

    use std::collections::HashMap;

    let mut puntajes = HashMap::new();

    puntajes.insert(String::from("Azul"), 10);
    puntajes.insert(String::from("Amarillo"), 50);

    let nombre_del_equipo = String::from("Azul");
    let puntaje = puntajes.get(&nombre_del_equipo).copied().unwrap_or(0);

    for (llave, valor) in &puntajes {
        println!("{llave}: {valor}");
    }

    let nombre_campo = String::from("Color favorito");
    let valor_campo = String::from("Azul");

    let mut map = HashMap::new();

    map.insert(nombre_campo, valor_campo);

    // Entry y or_insert
    // devuelve una referencia al valor mutable si existe la clave
    // si no inserta el nuevo valor y devuelve  una referencia mutable a el
    puntajes.entry(String::from("Amarillo")).or_insert(60);
    puntajes.entry(String::from("Morado")).or_insert(60);


    for (llave, valor) in &puntajes {
        println!("{llave}: {valor}");
    }

    let texto = "hola mundo maravilloso mundo";
    let mut map = HashMap::new();

    // cuenta las palabras de un &str
    // cada vuelta comprueba si  la llave es nueva
    // si es nueva la crea y le aumenta en 1
    // si no devuelve la referencia mut al valor y le aumenta 1
    for palabra in texto.split_whitespace() {
        let cont = map.entry(palabra).or_insert(0);
        *cont += 1;
    }

    println!("{:?}", map);
}

