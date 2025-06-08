mod sistema_hormigas;

use crate::sistema_hormigas::*;
use std::env;

fn sistema_hormigas(n_hormigas: usize, inicio: Nodo, destino: Nodo) {
    // EJEMPLO
    let conjunto_aristas: Vec<Vec<Nodo>> =
        vec![vec![1, 2, 3], vec![3, 4], vec![1], vec![0, 4], vec![2]];
    let (feromonas, mut hormigas) = algoritmo_inicializacion(&conjunto_aristas, n_hormigas, inicio);
    println!("{:?}", feromonas);
    println!("{:?}", hormigas);
    construccion_caminos(&conjunto_aristas, &feromonas, &mut hormigas, destino);

    println!("{:?}", feromonas);
    println!("{:?}", hormigas);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!(
            "Uso: $ ./{} <n_hormigas> <nodo_inicio> <nodo_destino>",
            args[0]
        );
        return;
    }
    let n_hormigas: usize = args[1].parse().expect("Numero de hormigas invalido");
    if n_hormigas < 1 {
        eprintln!("El numero de hormigas debe ser al menos 1");
        return;
    }
    let inicio: Nodo = args[2].parse().expect("Numero de nodo invalido");
    if inicio < 0 {
        eprintln!("Verificar el ejemplo y sus nodos disponibles");
        return;
    }
    let destino: Nodo = args[3].parse().expect("Numero de nodo invalido");
    if destino < 0 {
        eprintln!("Verificar el ejemplo y sus nodos disponibles");
        return;
    }

    println!(
        "Sistema de hormigas inicializado con {} hormigas.",
        n_hormigas
    );

    sistema_hormigas(n_hormigas, inicio, destino);
}
