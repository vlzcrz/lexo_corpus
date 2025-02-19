import seaborn as sns
import matplotlib.pyplot as plt
import os

def lineplot_heaps_law(x_values, y_values, output_path, file_name):
    output_path_formatted = output_path + file_name
    sns.set_theme(style="whitegrid")
    plt.figure(figsize=(10, 6))

    plot = sns.lineplot(x=x_values, y=y_values)
    plt.xlabel('Total de palabras presentes')
    plt.ylabel('Total de palabras unicas presentes')
    plt.title(f'{file_name} - Escala normal')

    plt.tight_layout()
    plt.savefig(output_path_formatted, dpi=300)
    print(f"Gráfico guardado en: {os.path.abspath(output_path_formatted)}")


#plot_zipf_law([1,2,3,4,5], [6,5,4,3,2], "./books-plot/", "prueba-seaborn")