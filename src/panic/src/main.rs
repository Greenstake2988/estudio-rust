use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    let archivo_saludo_resultante = File::open("hola.txt");
    let archivo_saludo = match archivo_saludo_resultante {
        Ok(archivo) => archivo,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hola.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problema al crear el archivo: {:?}", e),
            },
            otro_error=> {
                panic!("Problema al abrir el archivo: {:?}", otro_error);
            }
        }
    };

    // let archivo_saludo2 = File::open("hola2.txt").unwrap();
    //let archivo_saludo2 = File::open("hola2.txt").expect("hola2.txt no encontrado");

    let usuario = leer_el_usuario_desde_yaloquesea();

    println!("{:?}", usuario);
    println!("{:?}", ultimo_caracter_de_la_primera_linea(""));
}

fn leer_el_usuario_desde_el_archivo() -> Result<String, io::Error> {
    let archivo_usuario_resultante = File::open("hola.txt");

    let mut archivo_usuario = match archivo_usuario_resultante {
        Ok(archivo) => archivo,
        Err(e) => return Err(e),
    };

    let mut usuario = String::new();

    match archivo_usuario.read_to_string(&mut usuario) {
        Ok(_) => Ok(usuario),
        Err(e) => Err(e),
    }
}

fn leer_el_usuario_desde_el_archivo_corto() -> Result<String, io::Error> {
    let mut archivo_usuario = File::open("hola.txt")?;
    let mut usuario = String::new();
    archivo_usuario.read_to_string(&mut usuario)?;
    Ok(usuario)
}
fn leer_el_usuario_desde_el_archivo_aun_mas_corto() -> Result<String, io::Error> {
    let mut  usuario = String::new();

    File::open("hola.txt")?.read_to_string(&mut usuario)?;

    Ok(usuario)
}

fn leer_el_usuario_desde_yaloquesea() -> Result<String, io::Error> {
    fs::read_to_string("hola.txt")
}
fn ultimo_caracter_de_la_primera_linea(texto: &str) -> Option<char> {
    texto.lines().next()?.chars().last()
}