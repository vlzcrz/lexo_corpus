# Lexo Corpus PR

## Descripción

El proposito del proyecto es el analisis de corpus producto del contenido extraido de los documentos con formato .txt y .pdf
Cuenta con el analisis automatizado de grandes volumenes de texto presentes en documentos y obtener como resultados graficas y listas relevantes.

## Características

Funcionalidades a detalle

- ✅ Obtener el recuento total de palabras presentes en un corpus (texto extraido del documento)
- ✅ Obtener el recuento de los 50 mas frecuentes palabras presentes en un corpus (texto extraido del documento)
- ✅ Obtener el recuento de las distancias totales de cada inter-word (palabras especificas de interes) presentes en un corpus (texto extraido del documento)
- ✅ Elaborar la grafica de la ley de zipf ya sea para cada documento ó dataset
- ✅ Elaborar la grafica de la ley de heaps ya sea para cada documento ó dataset
- ✅ Elaborar un heatmap de las inter-word de interes para cada documento ó dataset
- ✅ Procesar corpus para diferentes casos presentes en documentos pdf (texto plano, ó mediante extracción de texto en imagenes)
- ✅ Acceso a la API de la libreria externa

## Tecnologías Utilizadas

- [Rust](https://www.rust-lang.org/tools/install)
- [Python (v3.8+)](https://www.python.org/downloads/)

## Dependencias Requeridas

Instalar gcc compiler (varia según el sistema operativo, para ubuntu viene empaquetado em build-essential)
tambien instalar las librerias de desarrollo python-dev para establecer la interacción entre ambos lenguajes (python-rust binding)

- [UBUNTU]

  sudo apt install build-essential
  sudo apt install python3-dev

## Instalación

1. Clona este repositorio:

   ```bash
   git clone https://github.com/vlzcrz/lexo_corpus.git
   cd repositorio
   ```

2. Crea el virtual env de python para este proyecto (con el nombre: 'lexo_corpus_env'):

   ```bash
   python3 -m venv lexo_corpus_env
   ```

3. Activa el entorno virtual y instala las dependencias de python para este proyecto:

   Activar el venv:

   ```bash
   source /lexo_corpus_env/bin/activate
   ```

   Instala dependencias python:

   ```bash
   pip install maturin
   ```

   ```bash
   pip install --upgrade pymupdf
   ```

   ```bash
   pip install numpy matplotlib seaborn
   ```

4. Copiar y pegar la carpeta completa de la libreria externa TET:

   - Renombra la carpeta a TET-5.6... a 'tetlib'
   - Traslada la carpeta 'tetlib' dentro de la carpeta 'python' dentro del proyecto

5. Crear las carpetas que permitiran la interacción con el programa
   Crea las siguientes carpetas en la raiz del proyecto:

   - "books-pdf" (carpeta donde almacenaras los pdf a analizar)
   - "books-txt" (carpeta donde almacenaras los txt a analizar)
   - "labeled-data" (carpeta donde almacenaras los csv para automatizar el analisis de varios documentos con su año.)
     Ej:

   ```csv
   document,year
   joe-biden-1.txt,2025
   joe-biden-2.txt,2024
   ```

6. Iniciar el proyecto (con el venv activado):
   ```bash
   cargo run
   ```
