import fitz  # PyMuPDF (dependencia)
import os


def split_pdf(input_pdf):    
   
    doc = fitz.open(input_pdf)
    # Crear la carpeta de salida si no existe
    os.makedirs("books-fracts", exist_ok=True)

    # Guardar cada página como un archivo PDF separado
    for page_num in range(len(doc)):
        new_doc = fitz.open()  # Se crea un nuevo documento vacío / accion por defecto cuando no encuentra el path del archivo
        new_doc.insert_pdf(doc, from_page=page_num, to_page=page_num)  # Se inserta en el nuevo pdf el contenido del pdf original, especificando un intervalo de paginas a extraer
        output_path = os.path.join("books-fracts", f"page_{page_num + 1}.pdf")
        new_doc.save(output_path)  
        new_doc.close() 

    doc.close()
