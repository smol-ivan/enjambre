use rand::Rng;
use rand_distr::{Distribution, Normal};

pub struct Configuracion {
    pub max_iter: usize,
    pub debug: bool,
    pub poblacion: usize,
    pub c1: f64,
    pub c2: f64,
}

///
/// DEFINICION DE CONSTANTES PARA MODELO DE VELOCIDAD
///
const INERCIA: f64 = 0.729;
const FACTOR_CONSTRICCION: f64 = 0.7298;
const DIMENSIONES: usize = 2;

///
/// DEFINICION DE FUNCIONES OBJETIVO
///
pub trait FuncionObjetivo {
    fn evaluar(&self, posicion: &Vec<f64>) -> f64;
    fn min_posicion(&self) -> f64;
    fn max_posicion(&self) -> f64;
}

pub struct FuncionEsfera;

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
}

pub struct FuncionRosenbrock;

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
}

pub struct FuncionRastrigin;

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
}

pub struct FuncionSchwefel;

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
}

pub struct FuncionAckley;

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
    fn optimo_teorico(&self) -> (Vec<f64>, f64) {
        (vec![0.0; DIMENSIONES], 0.0)
    }
}

pub trait ModeloVelocidad {
    fn actualizar(&self, particula: &mut Particula, mejor_global: &Vec<f64>, c1: f64, c2: f64);
}

pub struct ModeloInercia;

impl ModeloVelocidad for ModeloInercia {
    fn actualizar(&self, particula: &mut Particula, mejor_global: &Vec<f64>, c1: f64, c2: f64) {
        let mut rng = rand::thread_rng();

        for i in 0..particula.posicion.len() {
            let r1: f64 = rng.gen_range(0.0..1.0);
            let r2: f64 = rng.gen_range(0.0..1.0);

            let componente_inercia = INERCIA * particula.velocidad[i];
            let componente_personal =
                c1 * r1 * (particula.mejor_posicion_personal[i] - particula.posicion[i]);
            let componente_global = c2 * r2 * (mejor_global[i] - particula.posicion[i]);

            particula.velocidad[i] = componente_inercia + componente_personal + componente_global;
        }

        // Actualizar posición
        for i in 0..particula.posicion.len() {
            particula.posicion[i] += particula.velocidad[i];
        }
    }
}

pub struct ModeloConstriccion;

impl ModeloVelocidad for ModeloConstriccion {
    fn actualizar(&self, particula: &mut Particula, mejor_global: &Vec<f64>, c1: f64, c2: f64) {
        let mut rng = rand::thread_rng();

        for i in 0..particula.posicion.len() {
            let r1: f64 = rng.gen_range(0.0..1.0);
            let r2: f64 = rng.gen_range(0.0..1.0);

            let a1 = c1 * r1 * (particula.mejor_posicion_personal[i] - particula.posicion[i]);
            let a2 = c2 * r2 * (mejor_global[i] - particula.posicion[i]);

            particula.velocidad[i] = FACTOR_CONSTRICCION * (particula.velocidad[i] + a1 + a2);
        }

        // Actualizar posición
        for i in 0..particula.posicion.len() {
            particula.posicion[i] += particula.velocidad[i];
        }
    }
}

pub struct ModeloBarebones;

impl ModeloVelocidad for ModeloBarebones {
    fn actualizar(&self, particula: &mut Particula, mejor_global: &Vec<f64>, _c1: f64, _c2: f64) {
        let mut rng = rand::thread_rng();

        for i in 0..particula.posicion.len() {
            // Calcular la media: μd ← (p_besti,d + g_bestd) / 2
            let mu_d = (particula.mejor_posicion_personal[i] + mejor_global[i]) / 2.0;

            // Calcular la desviación estándar: σd ← |p_besti,d - g_bestd|
            let sigma_d = (particula.mejor_posicion_personal[i] - mejor_global[i]).abs();

            // Generar nuevo valor: pi,d(t + 1) ~ N(μd, σd)
            let normal = Normal::new(mu_d, sigma_d).unwrap();
            particula.posicion[i] = normal.sample(&mut rng);
        }

        // La velocidad no se usa
        for i in 0..particula.velocidad.len() {
            particula.velocidad[i] = 0.0;
        }
    }
}

#[derive(Clone, Debug)]
pub struct Particula {
    pub posicion: Vec<f64>,
    pub velocidad: Vec<f64>,
    pub mejor_posicion_personal: Vec<f64>,
    pub mejor_valor_personal: f64,
}

impl Particula {
    fn new(funcion: &dyn FuncionObjetivo) -> Self {
        let mut rng = rand::thread_rng();
        let mut posicion = Vec::new();
        let mut velocidad = Vec::new();

        for _ in 0..DIMENSIONES {
            posicion.push(rng.gen_range(funcion.min_posicion()..funcion.max_posicion()));
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

    pub fn evaluar_y_actualizar(&mut self, funcion: &dyn FuncionObjetivo) {
        // Mantener las cotas del dominio de la función
        for i in 0..self.posicion.len() {
            if self.posicion[i] > funcion.max_posicion() {
                self.posicion[i] = funcion.max_posicion();
            } else if self.posicion[i] < funcion.min_posicion() {
                self.posicion[i] = funcion.min_posicion();
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

fn inicializar_poblacion(pob: usize, funcion: &dyn FuncionObjetivo) -> Vec<Particula> {
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

pub fn pso(
    config: Configuracion,
    funcion: Box<dyn FuncionObjetivo>,
    modelo_velocidad: Box<dyn ModeloVelocidad>,
) -> (Vec<f64>, f64) {
    let mut poblacion = inicializar_poblacion(config.poblacion, funcion.as_ref());

    let (mut mejor_global, mut mejor_valor_global) = encontrar_mejor_global(&poblacion);

    for _ in 1..=config.max_iter {
        // Evaluar y actualizar mejor personal
        for particula in &mut poblacion {
            particula.evaluar_y_actualizar(funcion.as_ref());
        }

        let (nuevo_mejor_global, nuevo_mejor_valor) = encontrar_mejor_global(&poblacion);

        if nuevo_mejor_valor < mejor_valor_global {
            mejor_global = nuevo_mejor_global;
            mejor_valor_global = nuevo_mejor_valor;
        }

        // Actualizar velocidad y posición
        for particula in &mut poblacion {
            modelo_velocidad.actualizar(particula, &mejor_global, config.c1, config.c2);
        }
    }

    (mejor_global, mejor_valor_global)
}
