use rand::Rng;

const MAX_ITER: usize = 1;
const DIMENSIONES: usize = 2;
const INERCIA: f64 = 0.729;
const DEBUG: bool = false;

// Trait para definir funciones objetivo
trait FuncionObjetivo {
    fn evaluar(&self, posicion: &Vec<f64>) -> f64;
    fn min_posicion(&self) -> f64;
    fn max_posicion(&self) -> f64;
    fn nombre(&self) -> &str;
    fn optimo_teorico(&self) -> (Vec<f64>, f64);
}

// Implementación de la función Esfera
struct FuncionEsfera;

impl FuncionObjetivo for FuncionEsfera {
    fn evaluar(&self, posicion: &Vec<f64>) -> f64 {
        posicion.iter().map(|x| x * x).sum()
    }

    fn min_posicion(&self) -> f64 {
        -5.12
    }
    fn max_posicion(&self) -> f64 {
        5.12
    }
    fn nombre(&self) -> &str {
        "Función Esfera"
    }

    fn optimo_teorico(&self) -> (Vec<f64>, f64) {
        (vec![0.0; DIMENSIONES], 0.0)
    }
}

// Implementación de la función Rosenbrock
struct FuncionRosenbrock;

impl FuncionObjetivo for FuncionRosenbrock {
    fn evaluar(&self, posicion: &Vec<f64>) -> f64 {
        let mut suma = 0.0;
        for i in 0..posicion.len() - 1 {
            let xi = posicion[i];
            let xi1 = posicion[i + 1];
            suma += 100.0 * (xi1 - xi * xi).powi(2) + (1.0 - xi).powi(2);
        }
        suma
    }

    fn min_posicion(&self) -> f64 {
        -2.048
    }
    fn max_posicion(&self) -> f64 {
        2.048
    }
    fn nombre(&self) -> &str {
        "Función Rosenbrock"
    }

    fn optimo_teorico(&self) -> (Vec<f64>, f64) {
        (vec![1.0; DIMENSIONES], 0.0)
    }
}

// Implementación de la función Rastrigin
struct FuncionRastrigin;

impl FuncionObjetivo for FuncionRastrigin {
    fn evaluar(&self, posicion: &Vec<f64>) -> f64 {
        let a = 10.0;
        let n = posicion.len() as f64;
        let suma: f64 = posicion
            .iter()
            .map(|x| x * x - a * (2.0 * std::f64::consts::PI * x).cos())
            .sum();
        a * n + suma
    }

    fn min_posicion(&self) -> f64 {
        -5.12
    }
    fn max_posicion(&self) -> f64 {
        5.12
    }
    fn nombre(&self) -> &str {
        "Función Rastrigin"
    }

    fn optimo_teorico(&self) -> (Vec<f64>, f64) {
        (vec![0.0; DIMENSIONES], 0.0)
    }
}

// Implementación de la función Schwefel
struct FuncionSchwefel;

impl FuncionObjetivo for FuncionSchwefel {
    fn evaluar(&self, posicion: &Vec<f64>) -> f64 {
        let d = posicion.len() as f64;
        let suma: f64 = posicion.iter().map(|x| x * (x.abs().sqrt()).sin()).sum();
        418.9829 * d - suma
    }

    fn min_posicion(&self) -> f64 {
        -500.0
    }
    fn max_posicion(&self) -> f64 {
        500.0
    }
    fn nombre(&self) -> &str {
        "Función Schwefel"
    }

    fn optimo_teorico(&self) -> (Vec<f64>, f64) {
        (vec![420.9687; DIMENSIONES], 0.0)
    }
}

// Implementación de la función Ackley
struct FuncionAckley;

impl FuncionObjetivo for FuncionAckley {
    fn evaluar(&self, posicion: &Vec<f64>) -> f64 {
        let a = 20.0;
        let b = 0.2;
        let c = 2.0 * std::f64::consts::PI;
        let d = posicion.len() as f64;

        let suma_cuadrados: f64 = posicion.iter().map(|x| x * x).sum();
        let suma_cosenos: f64 = posicion.iter().map(|x| (c * x).cos()).sum();

        let termino1 = -a * (-b * (suma_cuadrados / d).sqrt()).exp();
        let termino2 = -(suma_cosenos / d).exp();

        termino1 + termino2 + a + std::f64::consts::E
    }

    fn min_posicion(&self) -> f64 {
        -32.768
    }
    fn max_posicion(&self) -> f64 {
        32.768
    }
    fn nombre(&self) -> &str {
        "Función Ackley"
    }

    fn optimo_teorico(&self) -> (Vec<f64>, f64) {
        (vec![0.0; DIMENSIONES], 0.0)
    }
}

#[derive(Clone, Debug)]
struct Particula {
    posicion: Vec<f64>,
    velocidad: Vec<f64>,
    mejor_posicion_personal: Vec<f64>,
    mejor_valor_personal: f64,
}

impl Particula {
    fn new<F: FuncionObjetivo>(funcion: &F) -> Self {
        let mut rng = rand::thread_rng();
        let mut posicion = Vec::new();
        let mut velocidad = Vec::new();

        for _ in 0..DIMENSIONES {
            posicion.push(rng.gen_range(funcion.min_posicion()..funcion.max_posicion()));
            // Inicializar velocidad en un rango razonable (sin limitación posterior)
            velocidad.push(rng.gen_range(-2.0..2.0));
        }

        let mejor_posicion_personal = posicion.clone();
        let mejor_valor_personal = funcion.evaluar(&posicion);

        Particula {
            posicion,
            velocidad,
            mejor_posicion_personal,
            mejor_valor_personal,
        }
    }

    fn actualizar<F: FuncionObjetivo>(
        &mut self,
        mejor_global: &Vec<f64>,
        c1: f64,
        c2: f64,
        funcion: &F,
    ) {
        let mut rng = rand::thread_rng();

        for i in 0..DIMENSIONES {
            let r1: f64 = rng.gen_range(0.0..1.0);
            let r2: f64 = rng.gen_range(0.0..1.0);

            let vel_anterior = self.velocidad[i];
            let componente_inercia = INERCIA * self.velocidad[i];
            let componente_personal =
                c1 * r1 * (self.mejor_posicion_personal[i] - self.posicion[i]);
            let componente_global = c2 * r2 * (mejor_global[i] - self.posicion[i]);

            // Actualizar velocidad según la fórmula PSO con inercia
            // v(t+1) = w * v(t) + c1 * r1 * (p_best - x) + c2 * r2 * (g_best - x)
            self.velocidad[i] = componente_inercia + componente_personal + componente_global;

            if DEBUG && (self.posicion[0] + self.posicion[1]).abs() < 15.0 {
                println!("    Dim {}: r1={:.3}, r2={:.3}", i, r1, r2);
                println!(
                    "    Vel anterior: {:.3}, inercia: {:.3}, personal: {:.3}, global: {:.3}",
                    vel_anterior, componente_inercia, componente_personal, componente_global
                );
                println!("    Nueva velocidad: {:.3}", self.velocidad[i]);
            }

            // Actualizar posición (sin limitación de velocidad según pseudocódigo)
            let pos_anterior = self.posicion[i];
            self.posicion[i] += self.velocidad[i];

            // Solo mantener las cotas del dominio de la función
            if self.posicion[i] > funcion.max_posicion() {
                self.posicion[i] = funcion.max_posicion();
            } else if self.posicion[i] < funcion.min_posicion() {
                self.posicion[i] = funcion.min_posicion();
            }

            if DEBUG && (pos_anterior + self.posicion[i]).abs() < 15.0 {
                println!(
                    "    Pos anterior: {:.3}, nueva pos: {:.3}",
                    pos_anterior, self.posicion[i]
                );
            }
        }

        // Evaluar nueva posición
        let nueva_calidad = funcion.evaluar(&self.posicion);

        // Actualizar mejor posición personal si es mejor
        if nueva_calidad < self.mejor_valor_personal {
            self.mejor_posicion_personal = self.posicion.clone();
            self.mejor_valor_personal = nueva_calidad;
        }
    }
}

fn inicializar_poblacion<F: FuncionObjetivo>(pob: usize, funcion: &F) -> Vec<Particula> {
    (0..pob).map(|_| Particula::new(funcion)).collect()
}

fn encontrar_mejor_global(poblacion: &Vec<Particula>) -> (Vec<f64>, f64) {
    let mejor = poblacion
        .iter()
        .min_by(|a, b| {
            a.mejor_valor_personal
                .partial_cmp(&b.mejor_valor_personal)
                .unwrap()
        })
        .unwrap();

    (
        mejor.mejor_posicion_personal.clone(),
        mejor.mejor_valor_personal,
    )
}

fn pso<F: FuncionObjetivo>(pob: usize, c1: f64, c2: f64, funcion: &F) -> (Vec<f64>, f64) {
    let mut poblacion = inicializar_poblacion(pob, funcion);

    // Mejor global inicial
    let (mut mejor_global, mut mejor_valor_global) = encontrar_mejor_global(&poblacion);

    for iteracion in 1..=MAX_ITER {
        for particula in &mut poblacion {
            let calidad = funcion.evaluar(&particula.posicion);

            // Actualizar mejor personal si es necesario
            if calidad < particula.mejor_valor_personal {
                particula.mejor_posicion_personal = particula.posicion.clone();
                particula.mejor_valor_personal = calidad;
            }
        }

        // Encontrar el mejor global de esta iteración
        let (nuevo_mejor_global, nuevo_mejor_valor) = encontrar_mejor_global(&poblacion);

        // Actualizar mejor global si es necesario
        if nuevo_mejor_valor < mejor_valor_global {
            mejor_global = nuevo_mejor_global;
            mejor_valor_global = nuevo_mejor_valor;
        }

        for particula in &mut poblacion {
            particula.actualizar(&mejor_global, c1, c2, funcion);
        }
    }

    (mejor_global, mejor_valor_global)
}

fn main() {
    // Elegir función objetivo (puedes cambiar esta línea para probar diferentes funciones):
    // let funcion = FuncionEsfera;       // Dominio: [-5.12, 5.12], Óptimo: (0,0) = 0
    // let funcion = FuncionRosenbrock;   // Dominio: [-2.048, 2.048], Óptimo: (1,1) = 0
    // let funcion = FuncionRastrigin;    // Dominio: [-5.12, 5.12], Óptimo: (0,0) = 0
    // let funcion = FuncionSchwefel;     // Dominio: [-500, 500], Óptimo: (420.9687, 420.9687) = 0
    // let funcion = FuncionAckley;       // Dominio: [-32.768, 32.768], Óptimo: (0,0) = 0

    let funcion = FuncionEsfera; // ← Cambiar aquí para probar otras funciones

    let (optimo_pos, optimo_val) = funcion.optimo_teorico();

    println!("=== Optimización por Cúmulo de Partículas (PSO) ===");
    println!("Función objetivo: {}", funcion.nombre());
    println!("Dimensiones: {}", DIMENSIONES);
    println!("Máximo de iteraciones: {}", MAX_ITER);
    println!();

    let poblacion = 2;
    let c1 = 2.0;
    let c2 = 2.0;

    println!("Parámetros PSO:");
    println!("  Población: {}", poblacion);
    println!("  c1 (importancia personal): {}", c1);
    println!("  c2 (importancia global): {}", c2);
    println!("  w (factor de inercia): {}", INERCIA);
    println!();

    let (mejor_posicion, mejor_valor) = pso(poblacion, c1, c2, &funcion);

    println!();
    println!("=== RESULTADOS FINALES ===");
    println!("Mejor posición encontrada: {:?}", mejor_posicion);
    println!("Mejor valor de la función: {:.10}", mejor_valor);
    println!(
        "Óptimo teórico: {:?} con valor {:.10}",
        optimo_pos, optimo_val
    );

    // Calcular el error
    let error = mejor_valor.sqrt();
    println!("Error (distancia euclidiana al óptimo): {:.10}", error);
}

// Función helper para probar múltiples funciones
#[allow(dead_code)]
fn probar_todas_las_funciones() {
    println!("{}", "=".repeat(60));
    println!("Probando Función Esfera");
    println!("{}", "=".repeat(60));
    let funcion_esfera = FuncionEsfera;
    let (mejor_posicion, mejor_valor) = pso(30, 2.0, 2.0, &funcion_esfera);
    let (optimo_pos, optimo_val) = funcion_esfera.optimo_teorico();
    println!("Resultado: {:?} = {:.6}", mejor_posicion, mejor_valor);
    println!("Óptimo:    {:?} = {:.6}", optimo_pos, optimo_val);
    println!("Error:     {:.6}", (mejor_valor - optimo_val).abs());

    println!("\n{}", "=".repeat(60));
    println!("Probando Función Rosenbrock");
    println!("{}", "=".repeat(60));
    let funcion_rosenbrock = FuncionRosenbrock;
    let (mejor_posicion, mejor_valor) = pso(30, 2.0, 2.0, &funcion_rosenbrock);
    let (optimo_pos, optimo_val) = funcion_rosenbrock.optimo_teorico();
    println!("Resultado: {:?} = {:.6}", mejor_posicion, mejor_valor);
    println!("Óptimo:    {:?} = {:.6}", optimo_pos, optimo_val);
    println!("Error:     {:.6}", (mejor_valor - optimo_val).abs());

    println!("\n{}", "=".repeat(60));
    println!("Probando Función Rastrigin");
    println!("{}", "=".repeat(60));
    let funcion_rastrigin = FuncionRastrigin;
    let (mejor_posicion, mejor_valor) = pso(30, 2.0, 2.0, &funcion_rastrigin);
    let (optimo_pos, optimo_val) = funcion_rastrigin.optimo_teorico();
    println!("Resultado: {:?} = {:.6}", mejor_posicion, mejor_valor);
    println!("Óptimo:    {:?} = {:.6}", optimo_pos, optimo_val);
    println!("Error:     {:.6}", (mejor_valor - optimo_val).abs());

    println!("\n{}", "=".repeat(60));
    println!("Probando Función Schwefel");
    println!("{}", "=".repeat(60));
    let funcion_schwefel = FuncionSchwefel;
    let (mejor_posicion, mejor_valor) = pso(50, 2.0, 2.0, &funcion_schwefel);
    let (optimo_pos, optimo_val) = funcion_schwefel.optimo_teorico();
    println!("Resultado: {:?} = {:.6}", mejor_posicion, mejor_valor);
    println!("Óptimo:    {:?} = {:.6}", optimo_pos, optimo_val);
    println!("Error:     {:.6}", (mejor_valor - optimo_val).abs());

    println!("\n{}", "=".repeat(60));
    println!("Probando Función Ackley");
    println!("{}", "=".repeat(60));
    let funcion_ackley = FuncionAckley;
    let (mejor_posicion, mejor_valor) = pso(30, 2.0, 2.0, &funcion_ackley);
    let (optimo_pos, optimo_val) = funcion_ackley.optimo_teorico();
    println!("Resultado: {:?} = {:.6}", mejor_posicion, mejor_valor);
    println!("Óptimo:    {:?} = {:.6}", optimo_pos, optimo_val);
    println!("Error:     {:.6}", (mejor_valor - optimo_val).abs());
}
