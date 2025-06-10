use rand::prelude::*;
use std::collections::HashMap;

pub type Nodo = i32;
pub type Peso = f64;
pub type Base = Vec<Vec<Nodo>>;
pub type Feromonas = Vec<HashMap<Nodo, Peso>>;
pub type Camino = Vec<Nodo>;
pub type Hormigas = Vec<Hormiga>;
pub type EvaluacionCaminos = Vec<usize>;
pub type Rho = f64;

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

pub fn algoritmo_inicializacion(conjunto_aristas: &Base) -> Feromonas {
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

    feromonas
}

pub fn inicializacion_hormigas(cantidad_hormigas: usize, inicio: Nodo) -> Hormigas {
    // Inicializacion de hormigas en punto inicial
    let mut hormigas: Hormigas = Vec::with_capacity(cantidad_hormigas);
    for _ in 0..cantidad_hormigas {
        hormigas.push(Hormiga::new(inicio));
    }
    hormigas
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
            let mut vertices_factibles: Camino = vecinos
                .iter()
                .copied()
                .filter(|nodo| !visitados.contains(nodo))
                .collect();

            // La hormiga esta atrapada en un nodo sin salidas
            if vertices_factibles.is_empty() && hormiga.camino.len() > 1 {
                // Retroceder antes de toparse con el camino muerto
                let anterior = hormiga.camino[hormiga.camino.len() - 2];
                vertices_factibles.push(anterior);
            }

            let siguiente = seleccion_ruleta(*origen, vertices_factibles, &feromonas.to_vec());
            hormiga.camino.push(siguiente.unwrap());
        }
    }
}

pub fn evaluacion_caminos(hormigas: &Hormigas) -> EvaluacionCaminos {
    let mut evaluacion_caminos = Vec::with_capacity(hormigas.len());
    for hormiga in hormigas.iter() {
        evaluacion_caminos.push(hormiga.camino.len());
    }
    evaluacion_caminos
}

pub fn actualizacion_feromona(
    hormigas: &Hormigas,
    feromonas: &mut Feromonas,
    evaluacion_caminos: &EvaluacionCaminos,
) {
    for (index, hormiga) in hormigas.iter().enumerate() {
        // Aportacion segun la inversa de la evaluacion del camino
        let aportacion: f64 = 1.0 / evaluacion_caminos[index] as f64;
        let camino = &hormiga.camino;
        // Para cada arista de nuestro camino
        for ventana in camino.windows(2) {
            let origen = ventana[0] as usize;
            let destino = ventana[1];

            if let Some(peso) = feromonas[origen].get_mut(&destino) {
                *peso += aportacion;
            }
        }
    }
}

pub fn evapozacion_feromona(conjunto_aristas: &Base, feromonas: &mut Feromonas, p: Rho) {
    for (origen, vecinos) in conjunto_aristas.iter().enumerate() {
        for vecino in vecinos.iter() {
            if let Some(peso) = feromonas[origen].get_mut(&(*vecino as i32)) {
                *peso *= 1.0 - p;
            }
        }
    }
}
