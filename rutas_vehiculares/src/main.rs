mod rutas_vehiculares;

use crate::rutas_vehiculares::*;

fn rutas_vehiculares(
    n_hormigas: usize,
    rho: f64,
    max_iteraciones: usize,
    importancia_distancia: f64,
    importancia_feromona: f64,
    filepath: String,
) {
    // Esqueleto de alto nivel para el algoritmo
    let datos = leer_matriz(filepath);

    // Inicializacion de conjunto de aristas, feromonas y distancias
    let ca = create_conjunto_aristas(datos.dimension as usize);
    let mut cf = algoritmo_inicializacion(&ca);
    let cd = calcular_matriz_distancias(&datos);

    for i in 1..=max_iteraciones {
        let mut hormigas = inicializacion_hormigas(n_hormigas, datos.deposito_id);

        construccion_rutas(
            &ca,
            &cf,
            &cd,
            &mut hormigas,
            datos.capacidad,
            importancia_feromona,
            importancia_distancia,
            datos.deposito_id,
            &datos.demanda,
            datos.n_vehiculos,
        );

        let evaluacion_rutas =
            evaluacion_soluciones(&hormigas, &cd, datos.capacidad, datos.n_vehiculos);

        // TODO: Mejores rutas de la iteraccion actual

        evapozacion_feromona(&ca, &mut cf, rho);

        actualizacion_feromona(&hormigas, &mut cf, &cd);
    }
}

fn main() {}
