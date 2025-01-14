use std::fs; //Libreria para manejo de archivos
pub struct Config {
    pub filename: String,
    pub query: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let filename = args[1].clone();
        let query = args[2].clone();

        Config { filename, query }
    }
}

pub fn run(config: Config) {
    //Llama a la función que lee el archivo
    let content = fs::read_to_string(config.filename)
        .expect("Algo salio mal al leer el archivo ¿está en la misma carpeta?");
    let found = search(&config.query, &content);

    for line in found {
        println!("{}", line);
    }
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}
