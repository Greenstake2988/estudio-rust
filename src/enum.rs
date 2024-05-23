#[derive(Debug)]
enum Mensaje {
    Salir,
    Mover { x: i32, y: i32 },
    Escribir(String),
    CambiarColor(i32, i32, i32),
}

impl Mensaje {
    fn llamar(&self) {
        println!("Llamando y escribiendo: {:?}", self);
    }
}

#[derive(Debug)]
enum TipoDirIp {
    V4(String),
    V6(String),
}

fn main() {
    let home = TipoDirIp::V4(String::from("127.0.0.1"));
    let loopback = TipoDirIp::V6(String::from("::1"));

    let m = Mensaje::Escribir(String::from("Hola Mundo"));
    m.llamar();
    
    let m2 = Mensaje::Escribir;
}

fn ruta(tipo_ip: TipoDirIp) {}




