import fitz  # PyMuPDF (dependencia)
import os
from pathlib import Path

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

def page_snapshots_by_pdf_division():
    fracts_folder = Path("books-fracts")
    snaps_folder = Path("books-snaps") 
    snaps_folder.mkdir(parents=True, exist_ok=True)
    for pdf_path in fracts_folder.iterdir():
        doc = fitz.open(pdf_path)
        page = doc.load_page(1)
        val = snaps_folder / f"snapshot350dpi_{pdf_path.name}.png"
        pix = page.get_pixmap(dpi=350)
        pix.save(val)

def page_snapshots_all_pdf_pages(input_pdf):
    snaps_folder = Path(__file__).resolve().parent / "books-snaps"
    snaps_folder.mkdir(parents=True, exist_ok=True)
    doc = fitz.open(input_pdf)
    for i in range(len(doc)):
        page = doc.load_page(i)
        val = snaps_folder / f"snapshot350dpi_page_{i+1}.png"
        pix = page.get_pixmap(dpi=350)
        pix.save(val)

def page_snapshots_by_pdf_page(input_pdf, page_input):
    snaps_folder = Path(__file__).resolve().parent / "books-snaps"
    snaps_folder.mkdir(parents=True, exist_ok=True)
    doc = fitz.open(input_pdf)
    page = doc.load_page(page_input - 1)
    val = snaps_folder / f"snapshot350dpi_page_{page_input}.png"
    pix = page.get_pixmap(dpi=350)
    pix.save(val)
    

def get_pages_qty(input_pdf):
    doc = fitz.open(input_pdf)
    doc_pages = len(doc)
    doc.close()
    return doc_pages
    
def generate_page_snapshot(input_pdf, requested_page=None):
    try:
        snaps_folder = Path(__file__).resolve().parent / "books-snaps"
        snaps_folder.mkdir(parents=True, exist_ok=True)
        
        doc = fitz.open(input_pdf)
        doc_pages = len(doc)
        
        # Si no se especifica página o está fuera de rango, devolver error
        if requested_page is None:
            doc.close()
            return {"status": "error", "message": "No page specified", "pages": doc_pages}
        
        if requested_page < 0 or requested_page >= doc_pages:
            doc.close()
            return {"status": "error", "message": f"Page out of range (0-{doc_pages-1})", "pages": doc_pages}
        
        # Generar snapshot
        page = doc.load_page(requested_page)
        output_path = snaps_folder / f"snapshot350dpi_page_{requested_page}.png"
        pix = page.get_pixmap(dpi=350)
        pix.save(output_path)
        
        doc.close()
        return 
    except Exception as e:
        return {"status": "error", "message": str(e), "pages": 0}
  
