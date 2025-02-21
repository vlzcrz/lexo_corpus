import os
import matplotlib.pyplot as plt
from matplotlib.ticker import MaxNLocator
import seaborn as sns
import numpy as np

def lineplot_heaps_law(x_values, y_values, output_path, file_name):
    output_path_formatted = os.path.join(output_path, file_name)
    
    sns.set_theme(style="whitegrid")

    plt.figure(figsize=(12, 7))
    
    plot = sns.lineplot(
        x=x_values, y=y_values, 
        marker="o", 
        markersize=6,
        linewidth=1.5,
        color="royalblue"  
    )

    plt.xlabel('Total words registered', fontsize=12, color="darkslategray", labelpad=15)
    plt.ylabel('Total unique words found', fontsize=12, color="darkslategray", labelpad=15)
    plt.title(f'{file_name} - Normal scale', fontsize=16, fontweight='bold', color="darkslategray")
    
    plt.grid(True, linestyle=":", linewidth=0.7, alpha=0.7)
    
    plt.savefig(output_path_formatted, dpi=150, bbox_inches="tight")
    plt.close('all')



def lineplot_log10_zipf_law(x_values, y_values, slope, intersection, output_path, file_name):
    output_path_formatted = os.path.join(output_path, file_name)
    
    sns.set_theme(style="whitegrid")

    plt.figure(figsize=(12, 7))

    sns.scatterplot(x=x_values, y=y_values, color="royalblue", alpha=0.5, label="Data")
    x_regression = np.linspace(min(x_values), max(x_values))
    y_regression = slope * x_regression + intersection

    sns.lineplot(x=x_regression, y=y_regression, color="red", alpha=0.7 ,label="LinRegress")

    plt.xlabel('Log (ranking)', fontsize=12, color="darkslategray", labelpad=15)
    plt.ylabel('Log (frequency)', fontsize=12, color="darkslategray", labelpad=15)
    plt.title(f'{file_name} - Log scale', fontsize=16, fontweight='bold', color="darkslategray")
    
    plt.grid(True, linestyle=":", linewidth=0.7, alpha=0.7)
    plt.legend()

    plt.savefig(output_path_formatted, dpi=150, bbox_inches="tight")
    plt.close()
    plt.close('all')


def lineplot_csv_dataset(title, x_label, y_label, x_values, y_values, output_path, file_name):
    output_path_formatted = os.path.join(output_path, file_name)
    
    sns.set_theme(style="whitegrid")

    plt.figure(figsize=(12, 7))
    
    plot = sns.lineplot(
        x=x_values, y=y_values, 
        marker="o",  
        markersize=8,
        linewidth=1.2,
        color="#F8333C"  
    )
    plot.xaxis.set_major_locator(plt.MaxNLocator(integer=True))

    plt.xlabel(x_label, fontsize=12, color="darkslategray", labelpad=15)
    plt.ylabel(y_label, fontsize=12, color="darkslategray", labelpad=15)
    plt.title(f'{title}', fontsize=16, fontweight='bold', color="darkslategray")
    
    plt.grid(True, linestyle=":", linewidth=0.8, alpha=0.5)
    

    plt.savefig(output_path_formatted, dpi=150, bbox_inches="tight")
    plt.close('all')

def heat_map(title, x_label, y_label, inter_words, distances, frequencies, output_path, file_name):
    output_path_formatted = os.path.join(output_path, file_name)

   # Obtener todas las distancias únicas en orden creciente
    unique_distances_set = set()  # Usamos un conjunto para eliminar duplicados

    for sublist in distances:  
        for d in sublist:      
            unique_distances_set.add(d)  

    unique_distances = sorted(list(unique_distances_set))  # Convertir a lista y ordenar

    # Crear un diccionario que asigna un índice a cada distancia única
    distance_index = {}
    for i, d in enumerate(unique_distances):  
        distance_index[d] = i  


    heatmap_data = np.zeros((len(inter_words), len(unique_distances)))

    for i, (dist_list, freq_list) in enumerate(zip(distances, frequencies)):
        for dist, freq in zip(dist_list, freq_list):
            col_index = distance_index[dist]  
            heatmap_data[i, col_index] = freq

    fig_width = max(12, len(unique_distances) * 0.5)  
    fig_height = max(6, len(inter_words) * 0.8)  
    
    plt.figure(figsize=(fig_width, fig_height))
    ax = sns.heatmap(
        heatmap_data, 
        annot=True, fmt=".0f", cmap="YlGnBu", linewidths=0.5, 
        xticklabels=unique_distances, square=True, 
        annot_kws={"size": 10}, 
        cbar_kws={"ticks": np.arange(np.ceil(heatmap_data.min()), np.ceil(heatmap_data.max()) + 1, 1)}  
    )

    plt.xlabel(x_label)
    plt.ylabel(y_label)
    plt.xticks(rotation=45)  
    plt.yticks(ticks=np.arange(len(inter_words)) + 0.5, labels=inter_words, rotation=0)
    plt.title(title)

    colorbar = ax.collections[0].colorbar
    colorbar.locator = MaxNLocator(integer=True) 
    colorbar.update_ticks()

    plt.savefig(output_path_formatted, dpi=150, bbox_inches="tight")
    plt.close('all')