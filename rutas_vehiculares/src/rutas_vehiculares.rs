use rand::prelude::*;

pub type NodoId = u32;
pub type Feromona = f64;
pub type Distancia = f64;
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Cliente {
    pub id: u32,
    pub demanda: u32,
}

#[derive(Debug)]
pub struct DatosVRP {
    pub nodos: Vec<Nodo>,
    pub dimension: u32,
    pub capacidad: u32,
    pub deposito_id: u32,
    pub clientes: Vec<Cliente>,
    pub n_vehiculos: u32,
}

#[derive(Debug, Clone)]
pub struct Hormiga {
    pub rutas: Vec<Camino>,
}

impl Hormiga {
    fn new() -> Hormiga {
        Hormiga { rutas: Vec::new() }
    }
}

pub struct Solucion {
    pub rutas: Vec<Camino>,
    pub costo_total: u32,
    pub vehiculos_usados: u32,
}

pub fn algoritmo_inicializacion(ca: &ConjuntoAristas) -> ConjuntoFeromonas {
    let mut rng = rand::rng();
    let dimension = ca.len();

    // Inicializar matriz de feromonas con el tamaño correcto
    let mut cf: ConjuntoFeromonas = vec![vec![0.0; dimension]; dimension];

    for i in 0..dimension {
        for j in 0..dimension {
            if i != j && ca[i][j] != 0 {
                // Solo inicializar feromona donde hay arista válida
                let mut feromona: Feromona = rng.random_range(0.01..=0.1);
                feromona = (feromona * 1_000.0).round() / 1_000.0;

                // Asignacion de feromona a arista (bidireccional)
                cf[i][j] = feromona;
                cf[j][i] = feromona;
            }
        }
    }
    cf
}

pub fn inicializacion_hormigas(n_hormigas: usize) -> Hormigas {
    let mut h: Hormigas = Vec::with_capacity(n_hormigas);
    for _ in 0..n_hormigas {
        h.push(Hormiga::new());
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
        let origen_indice = (origen - 1) as usize;
        let destino_indice = (destino - 1) as usize;

        let feromona_ij = cf[origen_indice][destino_indice];
        let distancia_ij = cd[origen_indice][destino_indice];

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

pub fn evapozacion_feromona(_ca: &ConjuntoAristas, cf: &mut ConjuntoFeromonas, p: Rho) {
    // Evaporación de feromona en todas las aristas
    // Fórmula: τ_ij = (1 - ρ) * τ_ij
    // donde ρ (rho) es la tasa de evaporación

    for i in 0..cf.len() {
        for j in 0..cf[i].len() {
            if i != j {
                // Aplicar evaporación
                cf[i][j] *= 1.0 - p;

                // Mantener un valor mínimo de feromona para evitar que se haga 0
                let feromona_minima = 0.001;
                if cf[i][j] < feromona_minima {
                    cf[i][j] = feromona_minima;
                }
            }
        }
    }
}

pub fn actualizacion_feromona(
    h: &Hormigas,
    cf: &mut ConjuntoFeromonas,
    evaluaciones: &Evaluaciones,
) {
    for (index, hormiga) in h.iter().enumerate() {
        let evaluacion = &evaluaciones[index];

        // Solo actualizar feromonas si la solución es factible
        if evaluacion.es_factible {
            // Aportación basada en la calidad de la solución
            // Mejor solución (menor costo) → mayor aportación
            let aportacion: f64 = 1.0 / evaluacion.costo_total as f64;

            // Para cada ruta de la hormiga
            for ruta in &hormiga.rutas {
                // Verificar que la ruta tenga al menos 2 nodos
                if ruta.len() < 2 {
                    continue;
                }

                // Para cada arista de la ruta (nodo_i -> nodo_j)
                for ventana in ruta.windows(2) {
                    let origen_id = ventana[0];
                    let destino_id = ventana[1];

                    let origen_indice = (origen_id - 1) as usize;
                    let destino_indice = (destino_id - 1) as usize;

                    // Verificar que los índices estén dentro de los límites
                    if origen_indice < cf.len() && destino_indice < cf[origen_indice].len() {
                        // Actualizar feromona
                        cf[origen_indice][destino_indice] += aportacion;
                        cf[destino_indice][origen_indice] += aportacion;
                    }
                }
            }
        }
        // Nota: Las soluciones no factibles no aportan feromona
    }
}

pub fn evaluacion_soluciones(
    h: &Hormigas,
    cd: &ConjuntoDistancias,
    capacidad_maxima: u32,
    n_vehiculos: u32,
    clientes: &Vec<Cliente>,
) -> Evaluaciones {
    let mut evaluaciones: Evaluaciones = Vec::with_capacity(h.len());

    for hormiga in h.iter() {
        let mut costo_total = 0u32;
        let vehiculos_usados = hormiga.rutas.len() as u32;

        // Verificar si usa más vehículos de los permitidos
        if vehiculos_usados > n_vehiculos {
            evaluaciones.push(EvaluacionSolucion {
                costo_total: u32::MAX, // Penalización por usar demasiados vehículos
                vehiculos_usados,
                es_factible: false,
            });
            continue;
        }

        let mut es_factible = true;

        // Evaluar cada ruta de la hormiga
        for ruta in &hormiga.rutas {
            if ruta.len() < 2 {
                continue; // Ruta vacía o con solo el depósito, saltar
            }

            let mut distancia_ruta = 0u32;
            let mut demanda_ruta = 0u32;

            // Calcular distancia y demanda total de la ruta
            for ventana in ruta.windows(2) {
                let nodo_actual_id = ventana[0];
                let nodo_siguiente_id = ventana[1];

                let nodo_actual_indice = (nodo_actual_id - 1) as usize;
                let nodo_siguiente_indice = (nodo_siguiente_id - 1) as usize;

                // Verificar que los índices estén dentro de los límites
                if nodo_actual_indice < cd.len()
                    && nodo_siguiente_indice < cd[nodo_actual_indice].len()
                {
                    distancia_ruta += cd[nodo_actual_indice][nodo_siguiente_indice] as u32;
                } else {
                    es_factible = false;
                    break;
                }
            }

            // Calcular demanda total de la ruta (excluyendo depósito)
            for &nodo_id in ruta.iter() {
                if let Some(cliente) = clientes.iter().find(|c| c.id == nodo_id) {
                    demanda_ruta += cliente.demanda;
                }
            }

            // Verificar restricción de capacidad
            if demanda_ruta > capacidad_maxima {
                es_factible = false;
            }

            costo_total += distancia_ruta;
        }

        // Verificar que todos los clientes fueron visitados
        let mut clientes_visitados: Vec<NodoId> = Vec::new();
        for ruta in &hormiga.rutas {
            for &nodo_id in ruta.iter() {
                if clientes.iter().any(|c| c.id == nodo_id) {
                    clientes_visitados.push(nodo_id);
                }
            }
        }

        // Remover duplicados y verificar que se visitaron todos los clientes
        clientes_visitados.sort();
        clientes_visitados.dedup();

        let mut clientes_esperados: Vec<NodoId> = clientes.iter().map(|c| c.id).collect();
        clientes_esperados.sort();

        if clientes_visitados != clientes_esperados {
            es_factible = false;
        }

        let evaluacion = EvaluacionSolucion {
            costo_total,
            vehiculos_usados,
            es_factible,
        };

        evaluaciones.push(evaluacion);
    }

    evaluaciones
}

pub fn calcular_conjunto_distancias(datos: &DatosVRP) -> ConjuntoDistancias {
    let n = datos.dimension as usize;
    let mut conjunto_distancias = vec![vec![0.0; n]; n];

    for i in 0..n {
        for j in 0..n {
            if i == j {
                conjunto_distancias[i][j] = 0.0;
            } else {
                // Obtener coordenadas de los nodos usando índices
                let nodo_i = &datos.nodos[i];
                let nodo_j = &datos.nodos[j];

                // Calcular distancia euclidiana: sqrt((x2-x1)² + (y2-y1)²)
                let dx = (nodo_j.x as f64 - nodo_i.x as f64).powi(2);
                let dy = (nodo_j.y as f64 - nodo_i.y as f64).powi(2);
                let distancia = (dx + dy).sqrt();

                // Asignar distancia al conjunto
                conjunto_distancias[i][j] = distancia;
                conjunto_distancias[j][i] = distancia;
            }
        }
    }

    conjunto_distancias
}

pub fn create_conjunto_aristas(dimension: usize) -> ConjuntoAristas {
    let mut ca: ConjuntoAristas = vec![vec![0; dimension]; dimension];

    for i in 0..dimension {
        for j in 0..dimension {
            if i != j {
                ca[i][j] = 1;
                ca[j][i] = 1;
            }
        }
    }
    ca
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
            if let Some(indice) = comment.find("No of trucks:") {
                let after = &comment[indice + "No of trucks:".len()..];
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
