#!/bin/bash
set -euo pipefail
 
echo "=== Iniciando script de prueba: $(date) ==="
echo "Directorio inicial: $(pwd)"

# --- Configuración ---
# Directorio del proyecto de Rust
PROJECT_DIR="rutas_vehiculares"

# Directorio donde se encuentran los archivos de prueba .vrp (definido tras entrar al proyecto)

# Parámetros para el algoritmo (se reciben como argumentos)
if [ $# -lt 3 ]; then
    echo "Uso: $0 <n_hormigas> <rho> <max_iteraciones>"
    exit 1
fi
N_HORMIGAS=$1
RHO=$2
MAX_ITERACIONES=$3
shift 3

# Los siguientes dos parámetros son requeridos por el programa,
# pero sus valores no se usan en el modo 'test' que hemos configurado.
IMPORTANCIA_DISTANCIA=0 
IMPORTANCIA_FEROMONA=0

# --- Ejecución ---

# 1. Navegar al directorio del proyecto
cd "$PROJECT_DIR" || { echo "No se pudo encontrar el directorio del proyecto: $PROJECT_DIR"; exit 1; }
echo "Directorio tras cd: $(pwd)"

# 1.1 Definir ruta a archivos de prueba dentro del proyecto
FILES_DIR="src/files"

# Mostrar contenido de carpeta de archivos antes de iniciar
echo "Archivos de prueba disponibles en $FILES_DIR:"
ls -l "$FILES_DIR"

# Asegurar que las carpetas `resultados`, `logs` y `plots` existen
mkdir -p resultados logs plots
echo "Directorios preparados y listados a continuación:" 
ls -ld resultados logs plots

# 2. Compilar el proyecto en modo 'release' para optimizar la velocidad.
#    Esto se hace una sola vez al principio.
echo "Compilando el proyecto en modo release..."
cargo build --release

# Verificar que la compilación fue exitosa
if [ $? -ne 0 ]; then
    echo "La compilación falló. Abortando."
    exit 1
fi

# Opciones de paralelismo
# Número máximo de procesos concurrentes (ajústalo según tu CPU)
MAX_JOBS=5

# 3. Iterar sobre todos los archivos .vrp y lanzar las pruebas en paralelo
echo "Iniciando pruebas en paralelo..."
# Recorremos cada archivo .vrp
for filepath in "$FILES_DIR"/*.vrp; do
    if [ -f "$filepath" ]; then
        # Obtener nombre base para logs
        file_stem=$(basename "$filepath" .vrp)
        echo "-> Lanzando prueba para: $filepath"

        # Ejecutar con 'nice' (opcional) y redirigir salida a logs  
        ./target/release/rutas_vehiculares \
            $N_HORMIGAS $RHO $MAX_ITERACIONES $IMPORTANCIA_DISTANCIA $IMPORTANCIA_FEROMONA \
            "$filepath" test >"logs/${file_stem}.log" 2>&1 &

        # Esperar si hay demasiados jobs corriendo
        while [ "$(jobs -p | wc -l)" -ge "$MAX_JOBS" ]; do
            # Opcional: mostrar número de jobs activos
            echo "Procesos en ejecución: $(jobs -p | wc -l), esperando..."
            sleep 1
        done
    fi
done

echo "Listado de logs generados:" 
ls -l logs || echo "No hay logs"
 
echo "Esperando a que terminen los procesos en background..."
# 4. Esperar a que todos los procesos en segundo plano terminen
wait

echo "Procesos en background finalizados."
echo "Listado de resultados CSV:" 
ls -l resultados || echo "No hay archivos CSV en resultados"

echo "=== Ejecución de Rust finalizada: $(date) ==="
echo "Generando boxplots de resultados..."
# Activar entorno virtual (si existe)
if [ -f "../.venv/bin/activate" ]; then
    source "../.venv/bin/activate"
fi
# Ejecutar plot_results.py pasando directorio de resultados y destino de plots
python3 ../plot_results.py -i resultados -o plots
echo "Listado de plots generados:" 
ls -l plots || echo "No hay archivos en plots"
echo "=== Script de prueba completado: $(date) ==="
