# Sistema de hormigas para encontrar el camino más corto en un laberinto

# Entradas:
# A: Conjutno de aristas
# n_k: Cantidad de hormigas
# i: Vertice inicial

# Salida:
# x: caminos de las hormigas

# Pseudocodigo: Algoritmo de inicializacion
# para cada arista (i, j) de A hacer:
#   T_ij <- valor aleatorio muy pequeño
# para cada hormiga k=1,...,n_k hacer:
#   x^(0) <- i
# regresar: T, x

import random

n = 3
i = (0,0)
n_k = 10
def crear_matriz(A):
    """
    Matriz de adyacencia binaria de tamaño n x n
    """
    matriz = [[0 for _ in range(A)] for _ in range(A)]
    for j in range(A):
        for k in range(A):
            if j != k:
                matriz[j][k] = random.choice([0, 1])
            else:
                matriz[j][k] = "x"
    return matriz


A = crear_matriz(n)
print("Matriz de adyacencia creada con tamaño:", A)

def inicializacion(A, n_k, i):
    T = A.copy()
    for j in range(len(T)):
        for k in range(len(T)):
            if j != k:
                T[j][k] = random.uniform(0.001, 0.0001)

    x = [i] * n_k  
    
    return T, x

T, x= inicializacion(A, n_k, i)

print("Hormigas", x)
print("Matriz de feromonas inicializada:", T)

# Probabilidad de transicion (Elijir que arista escoger) metodo de ruleta para seleccion
# r <- U(0,1)
# suma <- 0
# mientras suma < r
#   elegir_arista(i,j) "aleatorio"
#   suma <- suma + N_ik/Suma proporcional de la feromona de la arista
#   si suma >= r entonces
#       eligir (i,j)
#   fin si
# fin si

def probabilidad_transicion(T, pos_actual ):
    pass
    # r = random.random()
    # suma = 0
    # while suma < r:
    #     # Aristas disponibles desde pos_actual
    #     p = []
    #     for j in range(len(T)):
    #         if T[pos_actual[0]][j] > 0:

    #     # elegir arista 
    # pass

# Algoritmo construccion de caminios
# Entrada:
# A: Conjunto de aristas
# T: feromona
# n_k: cantidad de hormigas
# x: caminos de las hormigas

# para cada hormiga k=1,...,n_k hacer:
#   t <- 1
#   repetir
#       Seleccionar el proximo vertice j
#       x^k (t) <- j
#       t <- t+1
#   hasta que se alcance el vertice destino
#   Eliminar todos los ciclos de x^k