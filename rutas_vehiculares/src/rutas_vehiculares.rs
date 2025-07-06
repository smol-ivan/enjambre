use rand::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub type NodoId = u32;
pub type Feromona = f64;
pub type Distancia = f32;
pub type ConjuntoAristas = Vec<Vec<NodoId>>;
pub type ConjuntoFeromonas = Vec<Vec<Feromona>>;
pub type ConjuntoDistancias = Vec<Vec<Distancia>>;
pub type Camino = Vec<NodoId>;
pub type Hormigas = Vec<Hormiga>;
pub type Rho = f64;
pub type Evaluaciones = Vec<EvaluacionSolucion>;
pub type ImportanciaDistancia = f64;
pub type ImportanciaFeromona = f64;

pub struct EvaluacionSolucion {
    pub costo_total: u32,
    pub vehiculos_usados: u32,
    pub es_factible: bool,
}

pub struct Nodo {
    pub id: u32,
    pub x: u32,
    pub y: u32,
}

impl Nodo {
    fn new(id: u32, x: u32, y: u32) -> Nodo {
        Nodo { id, x, y }
    }
}

pub struct Cliente {
    pub id: u32,
    pub demanda: u32,
}

pub struct DatosVRP {
    pub nodos: Vec<Nodo>,
    pub dimension: u32,
    pub capacidad: u32,
    pub deposito_id: u32,
    pub clientes: Vec<Cliente>,
    pub n_vehiculos: u32,
}

pub struct Hormiga {
    pub rutas: Vec<Camino>,
}

impl Hormiga {
    fn new(deposito: NodoId) -> Hormiga {
        Hormiga { rutas: Vec::new() }
    }
}

pub fn algoritmo_inicializacion(ca: &ConjuntoAristas) -> ConjuntoFeromonas {
    let mut rng = rand::rng();

    // Agregar valores aleatorias de feromonas en las aristas
    let mut cf: ConjuntoFeromonas = vec![vec![]];
    for i in 0..ca.len() {
        for j in 0..ca.len() {
            if i != j && ca[i][j] == 1 {
                let mut feromona: Feromona = rng.random_range(0.01..=0.1);
                feromona = (feromona * 1_000.0).round() / 1_000.0;

                // Asignacion de feromona a arista
                cf[i][j] = feromona;
                cf[j][i] = feromona;
            }
        }
    }
    cf
}

pub fn inicializacion_hormigas(n_hormigas: usize, deposito: NodoId) -> Hormigas {
    let mut h: Hormigas = Vec::with_capacity(n_hormigas);
    for _ in 0..n_hormigas {
        h.push(Hormiga::new(deposito));
    }
    h
}

pub fn construccion_rutas(
    _ca: &ConjuntoAristas,
    cf: &ConjuntoFeromonas,
    cd: &ConjuntoDistancias,
    h: &mut Hormigas,
    capacidad_maxima: u32,
    i_f: ImportanciaFeromona,
    i_d: ImportanciaDistancia,
    deposito: NodoId,
    clientes: &Vec<Cliente>,
) {
    // Para cada hormiga
    for hormiga in h.iter_mut() {
        // Limpiar rutas previas
        hormiga.rutas.clear();

        // Crear conjunto de clientes sin visitar (excluir depósito)
        let mut clientes_sin_visitar: Vec<NodoId> = clientes
            .iter()
            .filter(|d| d.id != deposito)
            .map(|d| d.id)
            .collect();

        // Construir rutas hasta visitar todos los clientes
        while !clientes_sin_visitar.is_empty() {
            // Crear nueva ruta empezando desde el depósito
            let mut ruta_actual = vec![deposito];
            let mut capacidad_restante = capacidad_maxima;
            let mut nodo_actual = deposito;

            // Construir ruta para el vehículo actual
            loop {
                // Filtrar vértices factibles por capacidad
                let vertices_factibles: Vec<NodoId> = clientes_sin_visitar
                    .iter()
                    .filter(|&&cliente| {
                        // Encontrar la demanda del cliente
                        let demanda = clientes
                            .iter()
                            .find(|d| d.id == cliente)
                            .map(|d| d.demanda)
                            .unwrap_or(0);

                        // Verificar que la demanda no exceda la capacidad restante
                        demanda <= capacidad_restante
                    })
                    .copied()
                    .collect();

                // Si no hay vértices factibles, terminar la ruta del vehículo actual
                if vertices_factibles.is_empty() {
                    break;
                }

                // Seleccionar siguiente nodo usando ruleta
                if let Some(siguiente_nodo) =
                    seleccion_ruleta(nodo_actual, &vertices_factibles, cf, cd, i_d, i_f)
                {
                    // Agregar el nodo a la ruta
                    ruta_actual.push(siguiente_nodo);

                    // Actualizar capacidad restante
                    let demanda_cliente = clientes
                        .iter()
                        .find(|d| d.id == siguiente_nodo)
                        .map(|d| d.demanda)
                        .unwrap_or(0);

                    capacidad_restante = capacidad_restante.saturating_sub(demanda_cliente);

                    // Remover cliente de la lista de sin visitar
                    clientes_sin_visitar.retain(|&x| x != siguiente_nodo);

                    // Actualizar nodo actual
                    nodo_actual = siguiente_nodo;
                } else {
                    // Si no se puede seleccionar ningún nodo, terminar la ruta
                    break;
                }
            }

            // Regresar al depósito
            ruta_actual.push(deposito);

            // Agregar la ruta completada a la hormiga
            hormiga.rutas.push(ruta_actual);
        }

        // Nota: La hormiga ahora tiene una solución completa (visita todos los clientes)
        // pero puede usar más o menos vehículos que n_vehiculos.
        // La factibilidad se evaluará en la función de evaluación.
    }
}

fn seleccion_ruleta(
    origen: NodoId,
    vertices_factibles: &[NodoId],
    cf: &ConjuntoFeromonas,
    cd: &ConjuntoDistancias,
    i_d: ImportanciaDistancia,
    i_f: ImportanciaFeromona,
) -> Option<NodoId> {
    if vertices_factibles.is_empty() {
        return None;
    }

    if vertices_factibles.len() == 1 {
        return vertices_factibles.first().copied();
    }

    // Calcular probabilidades usando la fórmula del ACO
    // p_ij = (τ_ij^α * η_ij^β) / Σ(τ_ik^α * η_ik^β)
    // donde τ_ij es la feromona, η_ij es la heurística (1/distancia)

    let mut probabilidades: Vec<f64> = Vec::new();
    let mut suma_total = 0.0;

    for &destino in vertices_factibles {
        let feromona_ij = cf[origen as usize][destino as usize];
        let distancia_ij = cd[origen as usize][destino as usize];

        if distancia_ij <= 0.0 {
            // Si la distancia es 0, este nodo tiene probabilidad máxima
            return Some(destino);
        }

        // Heurística = 1/distancia (mayor preferencia a distancias cortas)
        let heuristica_ij = 1.0 / distancia_ij as f64;

        // Aplicar las importancias (exponentes)
        let valor = feromona_ij.powf(i_f) * heuristica_ij.powf(i_d);

        probabilidades.push(valor);
        suma_total += valor;
    }

    // Normalizar probabilidades
    for prob in &mut probabilidades {
        *prob /= suma_total;
    }

    // Selección por ruleta
    let mut rng = rand::rng();
    let umbral: f64 = rng.random_range(0.0..=1.0);
    let mut acumulado = 0.0;

    for (i, &prob) in probabilidades.iter().enumerate() {
        acumulado += prob;
        if acumulado >= umbral {
            return Some(vertices_factibles[i]);
        }
    }

    // En caso de error de redondeo, devolver el último elemento
    vertices_factibles.last().copied()
}

pub fn evapozacion_feromona(ca: &ConjuntoAristas, cf: &mut ConjuntoFeromonas, p: Rho) {
    todo!("Implementar evaporación de feromona");
}

pub fn actualizacion_feromona(h: &Hormigas, cf: &mut ConjuntoFeromonas, ed: &ConjuntoDistancias) {
    todo!("Implementar actualización de feromona");
}

pub fn evaluacion_soluciones(
    h: &Hormigas,
    cd: &ConjuntoDistancias,
    capacidad_maxima: u32,
    n_vehiculos: u32,
) -> Evaluaciones {
    todo!("Implementar evaluación de soluciones");
}

pub fn calcular_matriz_distancias(datos: &DatosVRP) -> ConjuntoDistancias {
    // Calcular la distancia euclidiana entre cada par de nodos
    let n = datos.dimension as usize;
    let mut matriz_distancias = vec![vec![0.0; n]; n];

    for i in 0..n {
        for j in 0..n {
            if i == j {
                matriz_distancias[i][j] = 0.0;
            } else {
                // Obtener coordenadas de los nodos
                let nodo_i = &datos.nodos[i];
                let nodo_j = &datos.nodos[j];

                // Calcular distancia euclidiana: sqrt((x2-x1)² + (y2-y1)²)
                let dx = (nodo_j.x as f64 - nodo_i.x as f64).powi(2);
                let dy = (nodo_j.y as f64 - nodo_i.y as f64).powi(2);
                let distancia = (dx + dy).sqrt();

                matriz_distancias[i][j] = distancia as f32;
            }
        }
    }

    matriz_distancias
}

pub fn create_conjunto_aristas(dimension: usize, nodos: &Vec<Nodo>) -> ConjuntoAristas {
    // Crear la matriz de adyacencia para las aristas
    todo!("Implementar obtención de conjunto de aristas");
}

pub fn leer_matriz(path: String) -> DatosVRP {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let file = File::open(path).expect("No se pudo abrir el archivo");
    let reader = BufReader::new(file);

    let mut dimension = 0u32;
    let mut capacidad = 0u32;
    let mut nodos: Vec<Nodo> = Vec::new();
    let mut clientes_con_demanda: Vec<Cliente> = Vec::new();
    let mut deposito_id: u32 = 0;
    let mut n_vehiculos: u32 = 0;

    let mut seccion = "";

    for linea in reader.lines() {
        let linea = linea.expect("Error leyendo línea");
        let ltrim = linea.trim();

        if ltrim.starts_with("COMMENT :") {
            // Ejemplo: COMMENT : (Augerat et al, No of trucks: 5, Optimal value: 784)
            let comment = ltrim["COMMENT :".len()..].trim();
            if let Some(idx) = comment.find("No of trucks:") {
                let after = &comment[idx + "No of trucks:".len()..];
                if let Some(end) = after.find(',') {
                    let trucks_str = after[..end].trim();
                    if let Ok(val) = trucks_str.parse::<u32>() {
                        n_vehiculos = val;
                    }
                } else {
                    // Si no hay coma, intentar parsear hasta el final
                    let trucks_str = after.trim();
                    if let Ok(val) = trucks_str.parse::<u32>() {
                        n_vehiculos = val;
                    }
                }
            }
        }
        if ltrim.starts_with("DIMENSION :") {
            dimension = ltrim["DIMENSION :".len()..]
                .trim()
                .parse()
                .expect("Error parseando DIMENSION");
        } else if ltrim.starts_with("CAPACITY :") {
            capacidad = ltrim["CAPACITY :".len()..]
                .trim()
                .parse()
                .expect("Error parseando CAPACITY");
        } else if ltrim.starts_with("NODE_COORD_SECTION") {
            seccion = "NODE_COORD_SECTION";
            continue;
        } else if ltrim.starts_with("DEMAND_SECTION") {
            seccion = "DEMAND_SECTION";
            continue;
        } else if ltrim.starts_with("DEPOT_SECTION") {
            seccion = "DEPOT_SECTION";
            continue;
        } else if ltrim == "-1" || ltrim == "EOF" || ltrim.is_empty() {
            continue;
        }

        match seccion {
            "NODE_COORD_SECTION" => {
                // Formato: Id x y
                let fila: Vec<u32> = ltrim
                    .split_whitespace()
                    .map(|c| c.parse::<u32>().expect("Error parseando nodo"))
                    .collect();
                if fila.len() == 3 {
                    let nodo = Nodo::new(fila[0], fila[1], fila[2]);
                    nodos.push(nodo);
                }
            }
            "DEMAND_SECTION" => {
                // Formato: Id demanda
                let fila: Vec<u32> = ltrim
                    .split_whitespace()
                    .map(|c| c.parse::<u32>().expect("Error parseando demanda"))
                    .collect();
                if fila.len() == 2 {
                    let cliente = Cliente {
                        id: fila[0],
                        demanda: fila[1],
                    };
                    clientes_con_demanda.push(cliente);
                }
            }
            "DEPOT_SECTION" => {
                // Solo un número, el id del depósito
                deposito_id = ltrim.parse::<u32>().expect("Error parseando deposito");
            }
            _ => {}
        }
    }

    DatosVRP {
        nodos,
        dimension,
        capacidad,
        deposito_id,
        clientes: clientes_con_demanda,
        n_vehiculos,
    }
}
