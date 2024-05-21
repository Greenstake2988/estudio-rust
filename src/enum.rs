#[derive(Debug)]
enum TipoDirIp {
    V4,
    V6,
}

fn main() {
    let cuatro = TipoDirIp::V4;
    let seis = TipoDirIp::V6;
}

fn ruta(tipo_ip: TipoDirIp) {}
