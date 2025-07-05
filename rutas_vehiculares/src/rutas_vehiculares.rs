use std::fs::File;
use std::io::{BufRead, BufReader};

pub type NodoId = u32;
pub type Feromona = f64;
pub type ConjuntoAristas = Vec<Vec<NodoId>>;
pub type ConjuntoFeromonas = Vec<Vec<Feromona>>;
pub type ConjuntoDistancias = Vec<Vec<u32>>;
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
    id: u32,
    x: u32,
    y: u32,
}

impl Nodo {
    fn new(id: u32, x: u32, y: u32) -> Nodo {
        Nodo { id, x, y }
    }
}

pub struct Demanda {
    id_cliente: u32,
    demanda: u32,
}

pub struct DatosVRP {
    pub clientes: Vec<Nodo>,
    pub dimension: u32,
    pub capacidad: u32,
    pub deposito_id: u32,
    pub demanda: Vec<Demanda>,
    pub n_vehiculos: u32,
}

pub struct Hormiga {
    pub rutas: Vec<Camino>,
}

pub fn algoritmo_inicializacion(ca: &ConjuntoAristas) -> ConjuntoFeromonas {
    todo!("Implementar algoritmo de inicialización de feromonas");
}

pub fn inicializacion_hormigas(n_hormigas: usize, deposito: NodoId) -> Hormigas {
    todo!("Implementar inicialización de hormigas");
}

pub fn construccion_rutas(
    ca: &ConjuntoAristas,
    cf: &ConjuntoFeromonas,
    cd: &ConjuntoDistancias,
    h: &mut Hormigas,
    capacidad_maxima: u32,
    i_f: ImportanciaFeromona,
    i_d: ImportanciaDistancia,
    deposito: NodoId,
    demandas: &Vec<Demanda>,
    n_vehiculos: u32,
) {
    todo!("Implementar construcción de rutas");
}

fn seleccion_ruleta(
    origen: NodoId,
    vertices_factibles: &mut Camino,
    cf: &ConjuntoFeromonas,
    cd: &ConjuntoDistancias,
    i_d: ImportanciaDistancia,
    i_f: ImportanciaFeromona,
    capacidad_restante: u32,
    demandas: &Vec<Demanda>,
) -> Option<NodoId> {
    todo!("Implementar selección por ruleta");
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
    // Calcular la distancia eucladiana entre cada par de nodos
    todo!("Implementar cálculo de matriz de distancias");
}

pub fn create_conjunto_aristas(dimension: usize) -> ConjuntoAristas {
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
    let mut clientes: Vec<Nodo> = Vec::new();
    let mut demandas: Vec<Demanda> = Vec::new();
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
                    clientes.push(nodo);
                }
            }
            "DEMAND_SECTION" => {
                // Formato: Id demanda
                let fila: Vec<u32> = ltrim
                    .split_whitespace()
                    .map(|c| c.parse::<u32>().expect("Error parseando demanda"))
                    .collect();
                if fila.len() == 2 {
                    let demanda = Demanda {
                        id_cliente: fila[0],
                        demanda: fila[1],
                    };
                    demandas.push(demanda);
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
        clientes,
        dimension,
        capacidad,
        deposito_id,
        demanda: demandas,
        n_vehiculos,
    }
}
