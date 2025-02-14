import os
from PDFlib import TET #TET (dependencia cargada desde RUST)

def open_and_read_pdf(pdf_path):
    # Inicializar el objeto TET
    absolute_path = os.path.abspath(pdf_path)
    print(f"Intentando abrir: {absolute_path}")  # Para depuración
    tet = TET.TET()

    try:
        # Abrir el documento PDF
        doc = tet.open_document(absolute_path, "")
        if doc == -1:
            print("Error al abrir el documento")
            return
        print(f"Documento abierto correctamente")
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
        tet.close_page(page)
       
        
        # Cerrar el documento
        tet.close_document(doc)
        return text
    except Exception as e:
        print(f"Error: {e}")
    finally:
        tet.delete()  # Asegúrate de liberar los recursos

