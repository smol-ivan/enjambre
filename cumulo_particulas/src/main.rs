mod cumulo_particulas;

use crate::cumulo_particulas::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 8 {
        eprintln!(
            "Uso: {} <MAX_ITER> <DEBUG> <POBLACION> <FUNCION_OBJETIVO> <MODELO_VELOCIDAD> <c1> <c2>",
            args[0]
        );
        return;
    }

    let config = Configuracion {
        max_iter: args[1].parse().expect("MAX_ITER debe ser un número válido"),
        debug: args[2]
            .parse()
            .expect("DEBUG debe ser un booleano (true/false)"),
        poblacion: args[3]
            .parse()
            .expect("POBLACION debe ser un número válido"),

        c1: args[6].parse().expect("c1 debe ser un número válido"),
        c2: args[7].parse().expect("c2 debe ser un número válido"),
    };

    let funcion: Box<dyn FuncionObjetivo> = match args[4].as_str() {
        "1" => Box::new(FuncionEsfera),
        "2" => Box::new(FuncionRastrigin),
        "3" => Box::new(FuncionSchwefel),
        "4" => Box::new(FuncionAckley),
        _ => {
            eprintln!("Función objetivo no válida. Use '1' ~ '4'.");
            return;
        }
    };

    let modelo_velocidad: Box<dyn ModeloVelocidad> = match args[5].as_str() {
        "1" => Box::new(ModeloInercia),
        "2" => Box::new(ModeloConstriccion),
        _ => {
            eprintln!("Modelo de velocidad no válido. Use '1' o '2'.");
            return;
        }
    };

    let (mejor_posicion, mejor_valor) = pso(config, funcion, modelo_velocidad);

    println!("Mejor posición: {:?}", mejor_posicion);
    println!("Mejor valor: {:.6}", mejor_valor);
}
