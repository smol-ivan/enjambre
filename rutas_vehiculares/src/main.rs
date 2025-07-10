mod rutas_vehiculares;

use crate::rutas_vehiculares::*;

use csv::Writer;
use serde::Serialize;
use std::env;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Serialize)]
struct Fila {
    ejec: usize,
    mas_importancia_distancia: usize,
    solo_distancia: usize,
    solo_feromona: usize,
    mas_importancia_feromona: usize,
    importancia_ambas: usize,
}

fn rutas_vehiculares(
    n_hormigas: usize,
    rho: f64,
    max_iteraciones: usize,
    importancia_distancia: f64,
    importancia_feromona: f64,
    filepath: String,
    test: bool,
) -> Solucion {
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
        if !test {
            print!("\rProgreso: {}/{}", i, max_iteraciones);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }

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

    if !test {
        println!();
        println!("Mejor solución encontrada:");
        println!("Costo total: {}", mejor_solucion.costo_total);
        println!("Vehículos usados: {}", mejor_solucion.vehiculos_usados);
        for (i, ruta) in mejor_solucion.rutas.iter().enumerate() {
            println!("Ruta {}: {:?}", i + 1, ruta);
        }
        println!();
    }
    mejor_solucion
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 7 {
        eprintln!("Uso: {} <n_hormigas> <rho> <max_iteraciones> <importancia_distancia> <importancia_feromona> <filepath> <test>", args[0]);
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
    let test = args.get(7).map_or(false, |s| s == "test");

    println!("Ejecutando algoritmo con los siguientes parámetros:");
    println!("Número de hormigas: {}", n_hormigas);
    println!("Rho (factor de evaporación): {}", rho);
    println!("Máximo de iteraciones: {}", max_iteraciones);
    println!("Importancia de la distancia: {}", importancia_distancia);
    println!("Importancia de la feromona: {}", importancia_feromona);
    println!("Archivo de datos: {}", filepath);
    println!();

    if test {
        let dir = "resultados";
        create_dir_all(dir).expect("No se pudo crear la carpeta de resultados");

        let file_stem = Path::new(&filepath)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");

        let csv_path = format!("{}/{}.csv", dir, file_stem);
        println!("Guardando resultados en: {}", csv_path);

        let mut writer = Writer::from_path(&csv_path).expect("No se pudo crear el archivo CSV");

        let mut mejores: Vec<Option<(usize, Vec<Vec<u32>>)>> = vec![None; 5];

        for i in 0..30 {
            let mut resultados = vec![0; 5];
            for j in 0..=4 {
                print!(
                    "\rEjecucion {}: {}/30, Prueba de parametros: {}/5",
                    &filepath,
                    i + 1,
                    j + 1
                );
                std::io::Write::flush(&mut std::io::stdout()).unwrap();

                let mut i_d = 0.0;
                let mut i_f = 0.0;
                if j == 0 {
                    i_d = 4.0;
                    i_f = 1.0;
                } else if j == 1 {
                    i_d = 1.0;
                    i_f = 0.0;
                } else if j == 2 {
                    i_d = 0.0;
                    i_f = 1.0;
                } else if j == 3 {
                    i_d = 1.0;
                    i_f = 4.0;
                } else if j == 4 {
                    i_d = 1.0;
                    i_f = 1.0;
                }
                // Devolver costo y solución completa
                let sol = rutas_vehiculares(
                    n_hormigas,
                    rho,
                    max_iteraciones,
                    i_d,
                    i_f,
                    filepath.clone(),
                    test,
                );
                resultados[j] = sol.costo_total;
                // Actualizar mejor solución si es primera vez o encontramos menor costo
                let rutas = sol.rutas;
                if mejores[j]
                    .as_ref()
                    .map_or(true, |m| sol.costo_total < m.0 as u32)
                {
                    mejores[j] = Some((sol.costo_total as usize, rutas.clone()));
                }
            }
            let fila = Fila {
                ejec: i + 1,
                mas_importancia_distancia: resultados[0] as usize,
                solo_distancia: resultados[1] as usize,
                solo_feromona: resultados[2] as usize,
                mas_importancia_feromona: resultados[3] as usize,
                importancia_ambas: resultados[4] as usize,
            };
            writer.serialize(fila).expect("No se pudo escribir la fila");
        }
        println!();
        writer.flush().expect("No se pudo guardar el archivo CSV");
        // Mejores soluciones a un archivo de texto
        let txt_path = format!("{}/solutions_{}.txt", dir, file_stem);
        let mut f = File::create(&txt_path).expect("No se pudo crear el archivo de soluciones");
        for (idx, opt) in mejores.into_iter().enumerate() {
            if let Some((cost, rutas)) = opt {
                writeln!(f, "=== Configuración {} ===", idx + 1).unwrap();
                writeln!(f, "Costo: {}", cost).unwrap();
                // Escribir cada ruta en línea separada
                for (ri, ruta) in rutas.iter().enumerate() {
                    let seq: Vec<String> = ruta.iter().map(|node| node.to_string()).collect();
                    writeln!(f, "Route #{}: {}", ri + 1, seq.join(" ")).unwrap();
                }
                writeln!(f).unwrap();
            }
        }
        println!("Soluciones guardadas en {}", txt_path);
    } else {
        let sol = rutas_vehiculares(
            n_hormigas,
            rho,
            max_iteraciones,
            importancia_distancia,
            importancia_feromona,
            filepath.clone(),
            test,
        );
        if let Some(valor_optimo) = leer_valor_optimo(&filepath) {
            println!("Valor óptimo conocido: {}", valor_optimo);
        }
        println!("Costo de la mejor solución: {}", sol.costo_total);
    }
}
