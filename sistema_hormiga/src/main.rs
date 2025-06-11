mod sistema_hormigas;

use crate::sistema_hormigas::*;
use std::env;

fn sistema_hormigas(n_hormigas: usize, inicio: Nodo, destino: Nodo, max_iteraciones: usize) {
    // EJEMPLO
    let conjunto_aristas: Vec<Vec<Nodo>> = vec![
        vec![],
        vec![2, 10, 5],    // 1
        vec![1, 6, 8, 3],  // 2
        vec![2, 7, 9, 4],  // 3
        vec![3, 8, 10, 5], // 4
        vec![4, 9, 6, 1],  // 5
        vec![2, 5, 10],    // 6
        vec![3, 8],        // 7
        vec![2, 7, 9, 4],  // 8
        vec![3, 8, 10, 5], // 9
        vec![4, 9, 6, 1],  //10
    ];
    let p: Rho = 0.6;

    let mut i = 0;

    let mut feromonas = algoritmo_inicializacion(&conjunto_aristas);

    let mut camino_minimo: Camino = Vec::new();

    while i <= max_iteraciones {
        let mut hormigas = inicializacion_hormigas(n_hormigas, inicio);

        construccion_caminos(&conjunto_aristas, &feromonas, &mut hormigas, destino);

        let evaluacion_caminos = evaluacion_caminos(&hormigas);

        // Buscar el mejor camino de la iteracion actual
        let (pos_mejor, valor_minimo) = evaluacion_caminos
            .iter()
            .enumerate()
            .min_by_key(|(_, &val)| val)
            .unwrap();

        if camino_minimo.is_empty() || *valor_minimo < camino_minimo.len() {
            camino_minimo = hormigas[pos_mejor].camino.clone();
        }

        evapozacion_feromona(&conjunto_aristas, &mut feromonas, p);

        actualizacion_feromona(&hormigas, &mut feromonas, &evaluacion_caminos);

        i += 1;
    }
    // Mostrar camino minimo
    println!("Camino minimo encontrado: {:?}", camino_minimo);
    println!("Tamano del camino minimo: {}", camino_minimo.len());
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!(
            "Uso: $ ./{} <n_hormigas> <nodo_inicio> <nodo_destino> <iteraciones_maximas>",
            args[0]
        );
        return;
    }
    let n_hormigas: usize = args[1].parse().expect("Numero de hormigas invalido");
    if n_hormigas < 1 {
        eprintln!("El numero de hormigas debe ser al menos 1");
        return;
    }
    let inicio: Nodo = args[2].parse().expect("Numero de nodo invalido");
    if inicio < 1 {
        eprintln!("Verificar el ejemplo y sus nodos disponibles");
        return;
    }
    let destino: Nodo = args[3].parse().expect("Numero de nodo invalido");
    if destino < 1 {
        eprintln!("Verificar el ejemplo y sus nodos disponibles");
        return;
    }
    let iteraciones: usize = args[4].parse().expect("Numero de iteraciones invalido!");
    if iteraciones < 1 {
        eprintln!("Verificar el ejemplu y el numero de itereaciones > 0");
    }

    println!("Total_hormigas: {} hormigas.", n_hormigas);

    sistema_hormigas(n_hormigas, inicio, destino, iteraciones);
}
