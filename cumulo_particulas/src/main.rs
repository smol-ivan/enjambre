mod cumulo_particulas;

use crate::cumulo_particulas::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 7 {
        eprintln!(
            "Uso: {} <MAX_ITER> <DEBUG> <POBLACION> <FUNCION> <c1> <c2>",
            args[0]
        );
        return;
    }

    let config = Configuracion {
        max_iter: args[1].parse().expect("MAX_ITER debe ser un número válido"),
        debug: args[2].parse().expect("DEBUG debe ser un booleano (true/false)"),
        poblacion: args[3].parse().expect("POBLACION debe ser un número válido"),

        c1: args[5].parse().expect("c1 debe ser un número válido"),
        c2: args[6].parse().expect("c2 debe ser un número válido"),
    };

    let funcion = match args[4].as_str() {
        "1" => FuncionEsfera,
        "2" => FuncionRosenbrock,
        "3" => FuncionRastrigin,
        "4" => FuncionSchwefel,
        "5" => FuncionAckley,
        _ => {
            eprintln!("Función objetivo no válida. Use '1' ~ '5'.");
            return;
        }
    };

    let (mejor_posicion, mejor_valor) = pso(config, funcion);

    println!("Mejor posición: {:?}", mejor_posicion);
    println!("Mejor valor: {:.6}", mejor_valor);
}