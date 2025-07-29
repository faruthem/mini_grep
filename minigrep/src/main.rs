use mini_greg::Config;
use std::env; //Libreria para manejo de argumentos

fn main() {
    //Atrapa los argumentos y los convierte en una colección de strings
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args); //Crea una instancia de Config

    //Imprimo el contenido del archivo
    println!("Palabra a buscar: {}", config.query);
    println!("Buscar: {}", config.filename);
//
    mini_greg::run(config);
}
