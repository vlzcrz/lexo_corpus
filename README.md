# Lexo Corpus PR

## Descripción

El proposito del proyecto es el analisis de corpus producto del contenido extraido de los documentos con formato 'txt' y 'pdf'.
Cuenta con el analisis automatizado de grandes volumenes de texto presentes en documentos, obteniendo como resultados graficas y listas de palabras relevantes en 'csv'.

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

Instalar gcc compiler (varia según el sistema operativo, para ubuntu viene empaquetado en build-essential).
tambien instalar las librerias de desarrollo python-dev para establecer la interacción entre ambos lenguajes (python-rust binding)

- [UBUNTU]

  - sudo apt install build-essential
  - sudo apt install python3-dev

## ⚠️ Nota importante ⚠️

Como la compilación y construcción del binario se realizo en Linux (Ubuntu 22.04) el binario no
tendra complicaciones de ejecución si es ejecutado bajo el mismo OS o distro similar de Linux.

Si no esta utilizando Ubuntu 22.04 o superior se recomienda el primer metodo, donde se elabora y construye
el binario con las dependencias de su sistema gracias al empaquetador de Rust (Cargo), este binario generado sera ejecutado sin problemas en su OS y sera compatible
con su OS (Windows, macOS, etc.).

Caso contrario es libre de utilizar el segundo metodo o el de su preferencia.

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
   source lexo_corpus_env/bin/activate
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
   - Traslada la carpeta 'tetlib' adentro de la carpeta 'python' presente en el proyecto

5. Crea las siguientes carpetas en la raiz del proyecto:

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

## Ejecutando el binario

[UBUNTU]

1. Una vez descargado el binario, situelo dentro de una carpeta y de permisos de ejecución.

   ```bash
   mkdir lexo_corpus_dir
   sudo mv lexo_corpus /lexo_corpus_dir
   ```

   ```bash
   sudo chmod +x /lexo_corpus_dir/lexo_corpus
   ```

2. Inicializar y activar el entorno virtual de python

   ```bash
   python3 -m venv lexo_corpus_env
   ```

   ***

   ⚠️ _En caso de error al crear el env debera instalar el siguiente paquete de recursos. (La versión del paquete depende de la versión de ubuntu)_

   ```bash
   sudo apt install python3.10-venv
   ```

   ***

   Active y mantenga activado en todo momento el python environment

   ```bash
   source lexo_corpus_env/bin/activate
   ```

3. Instale las dependencias de python

   ```bash
   pip install --upgrade pymupdf
   ```

   ```bash
   pip install numpy matplotlib seaborn

   ```

4. Crea las siguientes carpetas en lexo_corpus_dir:

   - "books-pdf" (carpeta donde almacenaras los pdf a analizar)
   - "books-txt" (carpeta donde almacenaras los txt a analizar)
   - "labeled-data" (carpeta donde almacenaras los csv para automatizar el analisis de varios documentos con su año.)
     Ej:

   ```csv
   document,year
   joe-biden-1.txt,2025
   joe-biden-2.txt,2024
   ```

## Demostración PROYECTO LEXO CORPUS

![](demo.gif)
