import os
import matplotlib.pyplot as plt
import seaborn as sns
import numpy as np

def lineplot_heaps_law(x_values, y_values, output_path, file_name):
    output_path_formatted = os.path.join(output_path, file_name)
    
    # Configurar estilo general
    sns.set_theme(style="whitegrid")

    # Crear figura
    plt.figure(figsize=(12, 7))
    
    # Graficar con estilo mejorado
    plot = sns.lineplot(
        x=x_values, y=y_values, 
        marker="o",  # Agrega marcadores en los puntos
        markersize=6,
        linewidth=1.5,
        color="royalblue"  # Color más llamativo
    )

    # Personalizar ejes y título
    plt.xlabel('Total de palabras presentes', fontsize=12, color="darkslategray", labelpad=15)
    plt.ylabel('Total de palabras únicas presentes', fontsize=12, color="darkslategray", labelpad=15)
    plt.title(f'{file_name} - Normal scale', fontsize=16, fontweight='bold', color="darkslategray")
    
    # Cuadrícula más sutil
    plt.grid(True, linestyle=":", linewidth=0.7, alpha=0.7)
    

    # Guardar el gráfico
    plt.savefig(output_path_formatted, dpi=150, bbox_inches="tight")

    print(f"Gráfico guardado en: {os.path.abspath(output_path_formatted)}")



def lineplot_log10_zipf_law(x_values, y_values, slope, intersection, output_path, file_name):
    output_path_formatted = os.path.join(output_path, file_name)
    
    # Configurar estilo general
    sns.set_theme(style="whitegrid")

    # Crear figura
    plt.figure(figsize=(12, 7))
    
    # Graficar con estilo mejorado

    sns.scatterplot(x=x_values, y=y_values, color="royalblue", alpha=0.5, label="Data")
    x_regression = np.linspace(min(x_values), max(x_values))
    y_regression = slope * x_regression + intersection

    sns.lineplot(x=x_regression, y=y_regression, color="red", alpha=0.7 ,label="LinRegress")


    # Personalizar ejes y título
    plt.xlabel('Log (ranking)', fontsize=12, color="darkslategray", labelpad=15)
    plt.ylabel('Log (frequency)', fontsize=12, color="darkslategray", labelpad=15)
    plt.title(f'{file_name} - Log scale', fontsize=16, fontweight='bold', color="darkslategray")
    
    # Cuadrícula más sutil
    plt.grid(True, linestyle=":", linewidth=0.7, alpha=0.7)
    plt.legend()

    # Guardar el gráfico
    plt.savefig(output_path_formatted, dpi=150, bbox_inches="tight")

    print(f"Gráfico guardado en: {os.path.abspath(output_path_formatted)}")
