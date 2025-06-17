use rand::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub type Nodo = u32;
pub type Peso = f64;
pub type Distancia = Vec<u32>;
pub type Base = Vec<Vec<Nodo>>;
pub type Feromonas = Vec<HashMap<Nodo, Peso>>;
pub type Distancias = Vec<Distancia>;
pub type Camino = Vec<Nodo>;
pub type Hormigas = Vec<Hormiga>;
pub type EvaluacionCaminos = Distancia;
pub type Rho = f64;

pub struct MatrizFromFile {
    pub matriz: Distancias,
    pub dimension: u32,
}

impl MatrizFromFile {
    fn new(matriz: Distancias, dimension: u32) -> MatrizFromFile {
        MatrizFromFile { matriz, dimension }
    }
}

#[derive(Debug)]
pub struct Hormiga {
    pub camino: Camino,
}

impl Hormiga {
    fn new(inicio: Nodo) -> Hormiga {
        Hormiga {
            camino: vec![inicio],
        }
    }
}

pub fn get_inicio(dimension: &u32) -> u32 {
    rand::random_range(0..*dimension)
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

pub fn inicializacion_hormigas(cantidad_hormigas: usize, inicio: u32) -> Hormigas {
    // Inicializacion de hormigas en punto inicial
    let mut hormigas: Hormigas = Vec::with_capacity(cantidad_hormigas);
    for _ in 0..cantidad_hormigas {
        hormigas.push(Hormiga::new(inicio));
    }
    hormigas
}

pub fn seleccion_ruleta(
    origen: Nodo,
    vertices_factibles: &mut Camino,
    feromonas: &Feromonas,
    distancias: &Distancias,
) -> Option<Nodo> {
    let mut rng = rand::rng();
    // Generar umbral aleatorio
    let umbral: f64 = rng.random_range(0.0..=1.0);
    let mut proporcion = 0.0;
    // Suma total de feromonas en los vertices factibles
    let total_feromonas: Peso = vertices_factibles
        .iter()
        .map(|nodo| feromonas[origen as usize].get(&nodo).copied().unwrap())
        .sum();

    if total_feromonas == 0.0 {
        return None;
    }

    let total_distancias: u32 = vertices_factibles
        .iter()
        .map(|n| distancias[origen as usize][*n as usize])
        .sum::<u32>();

    let total = total_feromonas / total_distancias as f64;

    loop {
        if vertices_factibles.len() == 1 {
            return vertices_factibles.last().copied();
        }

        // Elejir vertice aleatoriamente
        let indice_nodo = rng.random_range(0..=vertices_factibles.len() - 1);
        // Remover j de las posibles selecciones
        let j: Nodo = vertices_factibles.remove(indice_nodo);

        proporcion += feromonas[origen as usize].get(&j).copied().unwrap()
            / (distancias[origen as usize][j as usize] as Peso * total);

        if proporcion >= umbral {
            return Some(j);
        }
    }
}

pub fn construccion_caminos(
    conjunto_aristas: &Base,
    feromonas: &Feromonas,
    hormigas: &mut Hormigas,
    distancias: &Distancias,
) {
    let total_nodos = conjunto_aristas.len();
    for hormiga in hormigas.iter_mut() {
        while hormiga.camino.len() < total_nodos {
            let origen = *hormiga.camino.last().unwrap();

            // Generar los nodos aun no visitados
            let visitados = &hormiga.camino;
            let vecinos: &Camino = &conjunto_aristas[origen as usize];
            let vertices_factibles: Camino = vecinos
                .iter()
                .copied()
                .filter(|n| !visitados.contains(n))
                .collect();

            if let Some(siguiente) = seleccion_ruleta(
                origen,
                &mut vertices_factibles.clone(),
                &feromonas,
                &distancias,
            ) {
                hormiga.camino.push(siguiente);
            } else {
                break; // No hay opciones, termina la construccion
            }
        }
    }
}

pub fn evaluacion_caminos(
    hormigas: &Hormigas,
    n_hormigas: usize,
    distancias: &Distancias,
) -> EvaluacionCaminos {
    let mut evaluacion_caminos = Vec::with_capacity(n_hormigas);
    for hormiga in hormigas.iter() {
        let camino = &hormiga.camino;
        let mut distancia_total = 0;
        for i in 0..camino.len() - 1 {
            let a = camino[i];
            let b = camino[i + 1];
            distancia_total += distancias[a as usize][b as usize];
        }
        let Some(inicio) = camino.first() else {
            panic!("Error: Algo salio mal :p")
        };
        let Some(ultimo) = camino.last() else {
            panic!("Error: Algo salio mal :p")
        };
        distancia_total += distancias[*ultimo as usize][*inicio as usize];
        evaluacion_caminos.push(distancia_total);
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
            if let Some(peso) = feromonas[origen].get_mut(&(*vecino)) {
                *peso *= 1.0 - p;
            }
        }
    }
}

pub fn leer_matriz(path: &str) -> Result<MatrizFromFile, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut dimension = 0;
    let mut leyendo_matriz = false;
    let mut valores = Vec::new();

    for linea in reader.lines() {
        let linea = linea?;

        if linea.starts_with("DIMENSION:") {
            dimension = linea["DIMENSION:".len()..].trim().parse()?;
        } else if linea.trim() == "EDGE_WEIGHT_SECTION" {
            leyendo_matriz = true;
        } else if leyendo_matriz {
            if linea.trim() == "EOF" {
                break;
            }

            let numeros: Distancia = linea
                .split_whitespace()
                .map(|s| s.parse::<u32>())
                .collect::<Result<_, _>>()?;

            valores.extend(numeros);
        }
    }

    if dimension == 0 {
        return Err("No se encontro la dimension".into());
    }

    if valores.len() != dimension * dimension {
        return Err(format!(
            "Se esperaban {} valores pero se encontraron {}",
            dimension * dimension,
            valores.len()
        )
        .into());
    }
    let matriz = valores
        .chunks(dimension)
        .map(|fila| fila.to_vec())
        .collect();

    let object = MatrizFromFile::new(matriz, dimension as u32);

    Ok(object)
}

pub fn get_conjunto_aristas_from_distancia(file: &MatrizFromFile) -> Base {
    let mut conjunto_aristas: Base = Vec::with_capacity(file.dimension as usize);
    for i in 0..file.dimension {
        let mut fila: Vec<Nodo> = Vec::with_capacity(file.dimension as usize);
        for j in 0..file.dimension {
            if i != j {
                fila.push(j as Nodo);
            }
        }
        conjunto_aristas.push(fila);
    }
    conjunto_aristas
}
