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
        eprintln!("FUNCION_OBJETIVO: 1=Esfera, 2=Rastrigin, 3=Schwefel, 4=Ackley");
        eprintln!("MODELO_VELOCIDAD: 1=Inercia, 2=Constricción, 3=Barebones");
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
        "esfera" => Box::new(FuncionEsfera),
        "rastrigin" => Box::new(FuncionRastrigin),
        "schwefel" => Box::new(FuncionSchwefel),
        "ackley" => Box::new(FuncionAckley),
        _ => {
            eprintln!(
                "Función objetivo no válida. Use 'esfera', 'rastrigin', 'schwefel', 'ackley'."
            );
            return;
        }
    };

    let modelo_velocidad: Box<dyn ModeloVelocidad> = match args[5].as_str() {
        "inercia" => Box::new(ModeloInercia),
        "constriccion" => Box::new(ModeloConstriccion),
        "barebones" => Box::new(ModeloBarebones),
        _ => {
            eprintln!(
                "Modelo de velocidad no válido. Use 'Inercia)', 'constriccion' o 'barebones'."
            );
            return;
        }
    };

    let (mejor_posicion, mejor_valor) = pso(config, funcion, modelo_velocidad);

    println!("Mejor posición: {:?}", mejor_posicion);
    println!("Mejor valor: {:.6}", mejor_valor);
}
