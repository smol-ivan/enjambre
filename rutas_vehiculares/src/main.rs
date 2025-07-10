mod rutas_vehiculares;

use crate::rutas_vehiculares::*;

use std::env;

fn rutas_vehiculares(
    n_hormigas: usize,
    rho: f64,
    max_iteraciones: usize,
    importancia_distancia: f64,
    importancia_feromona: f64,
    filepath: String,
) {
    let datos = leer_matriz(filepath);

    // Inicializacion de conjunto de aristas, feromonas y distancias
    let ca = create_conjunto_aristas(datos.dimension as usize);
    let cd = calcular_conjunto_distancias(&datos);
    let mut cf = algoritmo_inicializacion(&ca);

    let mut mejor_solucion = Solucion {
        rutas: vec![vec![]; n_hormigas],
        costo_total: u32::MAX,
        vehiculos_usados: 0,
    };

    for i in 1..=max_iteraciones {
        print!("\rProgreso: {}/{}", i, max_iteraciones);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut hormigas = inicializacion_hormigas(n_hormigas);

        construccion_rutas(
            &ca,
            &cf,
            &cd,
            &mut hormigas,
            datos.capacidad,
            importancia_feromona,
            importancia_distancia,
            datos.deposito_id,
            &datos.clientes,
        );

        let evaluacion_rutas = evaluacion_soluciones(
            &hormigas,
            &cd,
            datos.capacidad,
            datos.n_vehiculos,
            &datos.clientes,
        );

        // TODO: Mejor ruta de la iteraccion actual
        for i in 0..evaluacion_rutas.len() {
            if evaluacion_rutas[i].costo_total < mejor_solucion.costo_total
                && evaluacion_rutas[i].es_factible
            {
                mejor_solucion.rutas = hormigas[i].rutas.clone();
                mejor_solucion.costo_total = evaluacion_rutas[i].costo_total;
                mejor_solucion.vehiculos_usados = evaluacion_rutas[i].vehiculos_usados;
            }
        }

        evapozacion_feromona(&ca, &mut cf, rho);

        actualizacion_feromona(&hormigas, &mut cf, &evaluacion_rutas);
    }
    println!();
    println!("Mejor solución encontrada:");
    println!("Costo total: {}", mejor_solucion.costo_total);
    println!("Vehículos usados: {}", mejor_solucion.vehiculos_usados);
    for (i, ruta) in mejor_solucion.rutas.iter().enumerate() {
        println!("Ruta {}: {:?}", i + 1, ruta);
    }
    println!();
    println!("Valor optimo encontrado: {}", mejor_solucion.costo_total);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 6 {
        eprintln!("Uso: {} <n_hormigas> <rho> <max_iteraciones> <importancia_distancia> <importancia_feromona> <filepath>", args[0]);
        return;
    }
    let n_hormigas: usize = args[1]
        .parse()
        .expect("Error al parsear el numero de hormigas");
    let rho: f64 = args[2].parse().expect("Error al parsear rho");
    let max_iteraciones: usize = args[3]
        .parse()
        .expect("Error al parsear el numero de iteraciones");
    let importancia_distancia: f64 = args[4]
        .parse()
        .expect("Error al parsear la importancia de la distancia");
    let importancia_feromona: f64 = args[5]
        .parse()
        .expect("Error al parsear la importancia de la feromona");
    let filepath = args[6].clone();

    println!("Ejecutando algoritmo con los siguientes parámetros:");
    println!("Número de hormigas: {}", n_hormigas);
    println!("Rho (factor de evaporación): {}", rho);
    println!("Máximo de iteraciones: {}", max_iteraciones);
    println!("Importancia de la distancia: {}", importancia_distancia);
    println!("Importancia de la feromona: {}", importancia_feromona);
    println!("Archivo de datos: {}", filepath);
    println!();

    rutas_vehiculares(
        n_hormigas,
        rho,
        max_iteraciones,
        importancia_distancia,
        importancia_feromona,
        filepath.clone(),
    );

    if let Some(valor_optimo) = leer_valor_optimo(&filepath) {
        println!("Valor óptimo conocido: {}", valor_optimo);
    }
}
