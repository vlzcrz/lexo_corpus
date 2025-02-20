import os
from PDFlib import TET #TET (dependencia cargada desde RUST)

def open_and_read_pdf(pdf_path):
    # Inicializar el objeto TET
    absolute_path = os.path.abspath(pdf_path)
    tet = TET.TET()

    try:
        doc = tet.open_document(absolute_path, "")
        if doc == -1:
            print("Error al abrir el documento")
            return
        print(f"Documento abierto correctamente")
        print(tet.pcos_get_string(doc, "filename"))
        page_count = tet.pcos_get_number(doc, "length:pages")
        print(page_count)
        if page_count != 1:
            print(f"El documento tiene {page_count} páginas. Este script está diseñado para un solo PDF de una página.")
            return
        
        page = tet.open_page(doc, 1, "granularity=page")
        
        
        text = tet.get_text(page)
        tet.close_page(page)
       
        
        # Cerrar el documento
        tet.close_document(doc)
        return text
    except Exception as e:
        print(f"Error: {e}")
    finally:
        tet.delete()  

