// #![allow(unused)]
use rand::prelude::*;
use std::env;
use std::fmt;
use std::fmt::Display;

type MatrizFeromonas = Matriz<f64>;
type ConjuntoAristas = Matriz<usize>;
type Hormigas = Vec<Hormiga>;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: u32,
    y: u32,
    weigh: Option<f64>,
}

impl Point {
    fn new(x: u32, y: u32) -> Point {
        Point { x, y, weigh: None }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Hormiga {
    camino: Vec<Point>,
}

impl Hormiga {
    fn new(inicio: Point) -> Hormiga {
        Hormiga {
            camino: vec![inicio],
        }
    }
}

impl fmt::Display for Hormiga {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, punto) in self.camino.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", punto)?;
        }
        write!(f, "]")
    }
}

#[derive(Clone, Debug)]
struct Matriz<T> {
    n: usize,
    matriz: Vec<Vec<T>>,
}

impl<T: std::fmt::Debug + Clone + Copy + Default + PartialEq + Display> Matriz<T> {
    fn mostrar(&self) {
        for row in &self.matriz {
            for value in row {
                print!("{:>6} ", value);
            }
            println!();
        }
        println!()
    }

    fn new(n: usize) -> Matriz<T> {
        let matriz = vec![vec![T::default(); n]; n];
        Matriz { n, matriz }
    }

    fn copy_to_feromone(&self) -> Matriz<f64> {
        let mut feromone = Matriz::<f64>::new(self.n);
        for i in 0..self.n {
            for j in 0..self.n {
                if self.matriz[i][j] != T::default() {
                    // Generar un valor aleatorio entre 0.0001 y 1.0 yup!
                    let mut rng = rand::rng();
                    let valor_aleatorio: f64 = rng.random_range(0.01..=0.1);
                    // Redondear a 3 decimales
                    let valor_aleatorio = (valor_aleatorio * 1_000.0).round() / 1_000.0;
                    feromone.matriz[i][j] = valor_aleatorio;
                }
            }
        }
        feromone
    }
}

fn algoritmo_inicializacion(
    matriz: &ConjuntoAristas,
    n_hormigas: usize,
    inicio: Point,
) -> (Vec<Hormiga>, MatrizFeromonas) {
    // Creacion de hormigas iniciales
    let mut hormigas = Vec::with_capacity(n_hormigas);
    for _ in 0..n_hormigas {
        hormigas.push(Hormiga::new(inicio));
    }
    // Creacion de matriz de feromonas con valores pequenos aleatorios
    let feromonas = matriz.copy_to_feromone();
    (hormigas, feromonas)
}

fn seleccion_ruleta(
    origen: Point,
    mut vertices_factibles: Vec<Point>,
    cantidad_feromona: f64,
) -> Option<Point> {
    let mut rng = rand::rng();

    let umbral = rng.random_range(0.0..=1.0);
    let mut proporcion_t = 0.0;

    loop {
        // Eligir de forma aleatorio un vertice 'j' perteneciente a los vertices factibles
        let index = rng.random_range(0..=vertices_factibles.len() - 1);
        // Remover j de las posibles selecciones
        let j = vertices_factibles.remove(index);

        proporcion_t += j.weigh.unwrap() / cantidad_feromona;
        if proporcion_t >= umbral {
            return Some(j);
        }
        // NOTE: Puede que haya errores invalidos desde los vertices factibles o los pesos de puntos con valores
        // a 0
        if vertices_factibles.is_empty() {
            // Si no hay mas vertices factibles, retornar None
            return None;
        }
    }
}

fn construccion_caminos(
    matriz: ConjuntoAristas,
    feromonas: MatrizFeromonas,
    hormigas: &mut Hormigas,
    destino: Point,
) {
    for hormiga in hormigas {
        let t = 1;
        loop {
            let origen = hormiga.camino.last().unwrap();
            // Encontrar vertices factibles desde origen
            let mut vertices_factibles = Vec::new();
            for i in 0..matriz.n {
                if matriz.matriz[origen.x as usize][i] == 1 && i != origen.x as usize {
                    let mut punto = Point::new(i as u32, 0);
                    // Asignar peso de feromona
                    punto.weigh = Some(feromonas.matriz[origen.x as usize][i]);
                    vertices_factibles.push(punto);
                }
            }

            // Calcular cantidad de feromona total
            let cantidad_feromona: f64 = vertices_factibles
                .iter()
                .map(|p| p.weigh.unwrap_or(0.0)) // En teoria nunca es 0
                .sum();
            // Seleccionar un vertice usando la seleccion de ruleta
            if let Some(siguiente) =
                seleccion_ruleta(*origen, vertices_factibles, cantidad_feromona)
            {
                // Agregar el vertice seleccionado al camino de la hormiga
                hormiga.camino.push(siguiente);
            } else {
                // TODO: Que pasara si no hay vertices factibles?
                panic!("No hay vertices factibles para la hormiga");
            }

            // hasta llegar al vertice destino
            if hormiga.camino.last().unwrap() == &destino {
                break;
            }
        }
    }
}

fn sistema_hormigas(n_matriz: usize, n_hormigas: usize, inicio: Vec<u32>, destino: Vec<u32>) {
    // ALGORITMO DE INICIALIZACION
    let mut adyacencia: Matriz<usize> = Matriz::new(n_matriz);
    adyacencia.matriz[0][1] = 1;
    adyacencia.matriz[1][0] = 1;

    adyacencia.matriz[0][2] = 1;
    adyacencia.matriz[2][0] = 1;

    adyacencia.matriz[0][3] = 1;
    adyacencia.matriz[3][0] = 1;

    adyacencia.matriz[0][4] = 1;
    adyacencia.matriz[4][0] = 1;

    adyacencia.matriz[1][2] = 1;
    adyacencia.matriz[2][1] = 1;

    adyacencia.matriz[2][3] = 1;
    adyacencia.matriz[3][2] = 1;

    adyacencia.matriz[3][4] = 1;
    adyacencia.matriz[4][3] = 1;
    println!("Matriz de adyacencia:");
    adyacencia.mostrar();
    let inicio = Point::new(inicio[0], inicio[1]);
    let (mut hormigas, feromonas) = algoritmo_inicializacion(&adyacencia, n_hormigas, inicio);
    // Mostrar hormigas y feromonas
    println!("Hormigas iniciales:");
    for (i, hormiga) in hormigas.iter().enumerate() {
        println!("Hormiga {}: {}", i + 1, hormiga);
    }
    println!();
    println!("Matriz de feromonas inicial:");
    feromonas.mostrar();

    // ALGORITMO DE CONSTRUCCION DE CAMINOS
    construccion_caminos(
        adyacencia,
        feromonas,
        &mut hormigas,
        Point::new(destino[0], destino[1]),
    );
    println!("Hormigas despues de la construccion de caminos:");
    for (i, hormiga) in hormigas.iter().enumerate() {
        println!("Hormiga {}: {}", i + 1, hormiga);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!(
            "Uso: {} <n_aristas> <n_hormigas> <inicio_x,inicio_y> <final_x,final_y>",
            args[0]
        );
        return;
    }
    let n_matriz: usize = args[1].parse().expect("Tamaño de matriz invalido");
    if n_matriz < 5 {
        eprintln!("El tamaño de la matriz debe ser al menos 5");
        return;
    }
    let n_hormigas: usize = args[2].parse().expect("Numero de hormigas invalido");
    if n_hormigas < 1 {
        eprintln!("El numero de hormigas debe ser al menos 1");
        return;
    }
    let inicio: Vec<u32> = args[3]
        .split(',')
        .map(|s| s.parse().expect("Formato invalido de punto"))
        .collect();
    if inicio.len() != 2 {
        eprintln!("Formato invalido de punto, debe ser: x,y");
        return;
    }
    let destino: Vec<u32> = args[4]
        .split(',')
        .map(|s| s.parse().expect("Formato invalido de punto"))
        .collect();
    if destino.len() != 2 {
        eprintln!("Formato invalido de punto, debe ser: x,y");
        return;
    }

    sistema_hormigas(n_matriz, n_hormigas, inicio, destino);
    println!(
        "Sistema de hormigas inicializado con matriz de n={}  y {} hormigas.",
        n_matriz, n_hormigas
    );
}
