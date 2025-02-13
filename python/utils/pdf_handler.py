import fitz  # PyMuPDF (dependencia)
import os
import sys
from PDFlib import TET #TET (dependencia cargada desde RUST)



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


def open_and_read_pdf(pdf_path):
    # Inicializar el objeto TET
    absolute_path = os.path.abspath(pdf_path)
    print(f"Intentando abrir: {absolute_path}")  # Para depuración
    tet = TET.TET()

    try:
        # Abrir el documento PDF
        doc = tet.open_document(absolute_path, "")
        print(doc)
        if doc == -1:
            print("Error al abrir el documento")
            return
        print(f"Documento abierto correctamente: {doc}")
        print(tet.pcos_get_string(doc, "filename"))
        # Obtener el número total de páginas del documento
        page_count = tet.pcos_get_number(doc, "length:pages")
        print(page_count)
        # Verificar que el documento tiene solo una página
        if page_count != 1:
            print(f"El documento tiene {page_count} páginas. Este script está diseñado para un solo PDF de una página.")
            return
        
        # Abrir la única página
        page = tet.open_page(doc, 1, "granularity=page")  # Solo se abre la página 1
        
        # Obtener el texto de la página
        text = tet.get_text(page)
        print(f"Texto de la página 1:\n{text}")
        tet.close_page(page)
       
        
        # Cerrar el documento
        tet.close_document(doc)
        return text
    except Exception as e:
        print(f"Error: {e}")
    finally:
        tet.delete()  # Asegúrate de liberar los recursos

