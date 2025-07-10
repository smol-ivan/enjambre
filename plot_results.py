#!/usr/bin/env python3
"""
Script para generar boxplots de los resultados del algoritmo de rutas vehiculares.
Ejemplo de uso:
    python plot_results.py                # Procesa todos los CSV en rutas_vehiculares/resultados
    python plot_results.py file1.csv file2.csv  # Procesa únicamente esos archivos
Opciones:
    -i, --input-dir   Directorio donde buscar CSV (por defecto: rutas_vehiculares/resultados)
    -o, --output-dir  Directorio donde guardar las gráficas (por defecto: rutas_vehiculares/plots)
"""
import os
import argparse
import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt

sns.set(style="whitegrid")

def main():
    parser = argparse.ArgumentParser(description="Generar boxplots a partir de archivos CSV de resultados.")
    parser.add_argument('csv_files', nargs='*', help='Archivos CSV específicos a procesar')
    parser.add_argument('-i', '--input-dir', default='rutas_vehiculares/resultados',
                        help='Directorio de entrada con archivos CSV')
    parser.add_argument('-o', '--output-dir', default='rutas_vehiculares/plots',
                        help='Directorio donde guardar las gráficas')
    args = parser.parse_args()

    # Obtener lista de CSV a procesar
    if args.csv_files:
        csv_paths = args.csv_files
    else:
        csv_paths = [os.path.join(args.input_dir, f)
                     for f in os.listdir(args.input_dir) if f.endswith('.csv')]

    os.makedirs(args.output_dir, exist_ok=True)

    for csv_path in csv_paths:
        if not os.path.isfile(csv_path):
            print(f"Archivo no encontrado: {csv_path}")
            continue

        df = pd.read_csv(csv_path)
        df_melted = df.melt(id_vars='ejec', var_name='configuracion', value_name='costo')

        # Crear boxplot
        basename = os.path.basename(csv_path)
        name, _ = os.path.splitext(basename)
        plt.figure(figsize=(10, 6))
        sns.boxplot(data=df_melted, x='configuracion', y='costo')
        plt.title(f'Distribución de costos para {name}')
        plt.xlabel('Configuración de parámetros')
        plt.ylabel('Costo total')
        plt.xticks(rotation=45)
        plt.tight_layout()

        save_path = os.path.join(args.output_dir, f'{name}_boxplot.png')
        plt.savefig(save_path)
        plt.close()
        print(f'Guardado: {save_path}')

if __name__ == '__main__':
    main()
