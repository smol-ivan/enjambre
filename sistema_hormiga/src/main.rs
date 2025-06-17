mod sistema_hormigas;

use crate::sistema_hormigas::*;
use std::env;

fn sistema_hormigas(
    n_hormigas: usize,
    _importancia_distancia: Rho,
    _importancia_feromona: Rho,
    max_iteraciones: usize,
    filepath: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = leer_matriz(filepath.as_str())?;
    let conjunto_aristas = get_conjunto_aristas_from_distancia(&file);
    let distancias = file.matriz;

    let p: Rho = 0.6;

    let mut feromonas = algoritmo_inicializacion(&conjunto_aristas);

    let mut camino_minimo: Camino = Vec::new();
    let mut mejor_costo = u32::MAX;

    let ciudad_inicio = get_inicio(&file.dimension);

    for _ in 1..=max_iteraciones {
        let mut hormigas = inicializacion_hormigas(n_hormigas, ciudad_inicio);

        construccion_caminos(&conjunto_aristas, &feromonas, &mut hormigas, &distancias);

        let evaluacion_caminos = evaluacion_caminos(&hormigas, hormigas.len(), &distancias);

        // Mostrar el camino de cada hormiga
        for (i, hormiga) in hormigas.iter().enumerate() {
            // println!("Hormiga {}: Camino: {:?}", i + 1, hormiga.camino);
        }

        // Buscar el mejor camino de la iteracion actual
        let (pos_mejor, costo_actual) = evaluacion_caminos
            .iter()
            .enumerate()
            .min_by_key(|(_, &val)| val)
            .unwrap();

        if *costo_actual < mejor_costo {
            mejor_costo = *costo_actual;
            camino_minimo = hormigas[pos_mejor].camino.clone();
        }

        evapozacion_feromona(&conjunto_aristas, &mut feromonas, p);

        actualizacion_feromona(&hormigas, &mut feromonas, &evaluacion_caminos);
    }
    println!("Camino minimo encontrado: {:?}", camino_minimo);
    println!("Costo del camino minimo: {}", mejor_costo);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        eprintln!(
            "Uso: $ ./{} <n_hormigas> <nodo_inicio> <nodo_destino> <iteraciones_maximas> <filepath>",
            args[0]
        );
        return;
    }
    let n_hormigas: usize = args[1].parse().expect("Numero de hormigas invalido");
    if n_hormigas < 1 {
        eprintln!("El numero de hormigas debe ser al menos 1");
        return;
    }
    let importancia_feromona: Rho = args[2].parse().expect("Numero de nodo invalido");
    if importancia_feromona < 0.0 {
        eprintln!("Verificar el ejemplo y sus nodos disponibles");
        return;
    }
    let importancia_distancia: Rho = args[3].parse().expect("Numero de nodo invalido");
    if importancia_distancia < 0.0 {
        eprintln!("Verificar el ejemplo y sus nodos disponibles");
        return;
    }
    let iteraciones: usize = args[4].parse().expect("Numero de iteraciones invalido!");
    if iteraciones < 1 {
        eprintln!("Verificar el ejemplo y el numero de itereaciones > 0");
    }
    let filepath: String = args[5].parse().expect("String invalido!");

    println!("Total_hormigas: {} hormigas.", n_hormigas);
    println!("Matriz: {}", filepath);

    let _ = sistema_hormigas(
        n_hormigas,
        importancia_distancia,
        importancia_feromona,
        iteraciones,
        filepath,
    );
}
