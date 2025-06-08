use rand::prelude::*;
use std::collections::HashMap;

pub type Nodo = i32;
pub type Peso = f64;
pub type Base = Vec<Vec<Nodo>>;
pub type Feromonas = Vec<HashMap<Nodo, Peso>>;
pub type Camino = Vec<Nodo>;
pub type Hormigas = Vec<Hormiga>;

#[derive(Debug)]
pub struct Hormiga {
    camino: Camino,
}

impl Hormiga {
    fn new(inicio: Nodo) -> Hormiga {
        Hormiga {
            camino: vec![inicio],
        }
    }
}

pub fn algoritmo_inicializacion(
    conjunto_aristas: &Base,
    cantidad_hormigas: usize,
    inicio: Nodo,
) -> (Feromonas, Hormigas) {
    // Agregar valores aletatorios de feromonas en las aristas
    let mut feromonas: Feromonas = vec![HashMap::new(); conjunto_aristas.len()];
    for (index, conexiones_vecinos) in conjunto_aristas.iter().enumerate() {
        for conexion in conexiones_vecinos.iter() {
            // Generacion valor aleatorio y redondeo de decimales
            let mut rng = rand::rng();
            let mut feromona: Peso = rng.random_range(0.01..=0.1);
            feromona = (feromona * 1_000.0).round() / 1_000.0;
            // Asignacion de feromona a arista
            feromonas[index].insert(*conexion, feromona);
        }
    }
    // Inicializacion de hormigas en punto inicial
    let mut hormigas: Hormigas = Vec::with_capacity(cantidad_hormigas);
    for _ in 0..cantidad_hormigas {
        hormigas.push(Hormiga::new(inicio));
    }
    (feromonas, hormigas)
}

pub fn seleccion_ruleta(
    origen: Nodo,
    mut vertices_factibles: Camino,
    feromonas: &Feromonas,
) -> Option<Nodo> {
    let mut rng = rand::rng();
    // Generar umbral aleatorio
    let umbral: f64 = rng.random_range(0.0..=1.0);
    let mut proporcion_feromona = 0.0;
    // Suma total de feromonas en los vertices factibles
    let total_feromonas: Peso = vertices_factibles
        .iter()
        .map(|nodo| feromonas[origen as usize].get(&nodo).copied().unwrap())
        .sum();

    if total_feromonas == 0.0 {
        return None;
    }

    loop {
        if vertices_factibles.len() == 1 {
            return vertices_factibles.last().copied();
        }

        // Elejir vertice aleatoriamente
        let indice_nodo = rng.random_range(0..=vertices_factibles.len() - 1);
        // Remover j de las posibles selecciones
        let j: Nodo = vertices_factibles.remove(indice_nodo);

        proporcion_feromona =
            proporcion_feromona + feromonas[origen as usize].get(&j).copied().unwrap();

        if proporcion_feromona >= umbral {
            return Some(j);
        }
    }
}

pub fn construccion_caminos(
    conjunto_aristas: &Base,
    feromonas: &Feromonas,
    hormigas: &mut Hormigas,
    destino: Nodo,
) {
    for hormiga in hormigas.iter_mut() {
        loop {
            let origen = hormiga.camino.last().unwrap();

            if *origen == destino {
                break;
            }

            // Evitar volver a los mismo nodos si ya fueron visitados
            // NOTE: Esto evita que existan ciclos en el camino que forma la hormiga
            let visitados = &hormiga.camino;
            let vecinos: &Camino = &conjunto_aristas[*origen as usize];
            let vertices_factibles: Camino = vecinos
                .iter()
                .copied()
                .filter(|nodo| !visitados.contains(nodo))
                .collect();

            if vertices_factibles.is_empty() {
                break;
            }

            let siguiente = seleccion_ruleta(*origen, vertices_factibles, &feromonas.to_vec());
            hormiga.camino.push(siguiente.unwrap());
        }
    }
}
