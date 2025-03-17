from paddleocr import PaddleOCR, draw_ocr
from pathlib import Path
# draw result
from PIL import Image


def get_content_reconstruct(txts):
    content = ""
    txt_reconstructed = ""
    on_reconstruction = False
    for txt in txts:
        if on_reconstruction:
            if txt[-1] == '-':
                txt_reconstructed += txt[:-1]
                continue
            else:
                content += f"{txt_reconstructed}{txt} "
                txt_reconstructed = ""
                on_reconstruction = False
                continue
        
        if txt[-1] == '-':
            txt_reconstructed += txt[:-1]
            on_reconstruction = True
            continue

        content += f"{txt} "

    return content

def get_content_raw(txts):
    content = ""
    for txt in txts:
        content += f"{txt} "
    return content

def get_text_on_image(page):
    # Configurar rutas
    img_path = Path(__file__).resolve().parent / "books-snaps" / f"snapshot350dpi_page_{page}.png"
    font_path = Path(__file__).resolve().parent / "fonts" / "Inter_no-static.ttf"
    debug_path = Path(__file__).resolve().parent / "paddle-debug" / f"debug_paddleocr_page_{page}.jpg"
    
    # Inicializar OCR (considerar mover esto fuera si se llama repetidamente)
    ocr = PaddleOCR(use_angle_cls=False, lang='en')
    
    # Realizar OCR
    result = ocr.ocr(str(img_path), cls=False)
    
    # Verificar que hay resultados
    if not result or len(result) == 0:
        return ""
    
    # Extraer texto del resultado
    # PaddleOCR devuelve: [página][línea][posición/texto+confianza]
    txts = [line[1][0] for line in result[0]]
    content = get_content_raw(txts)
    
    # Opcional: Guardar imagen de debug
    try:
        image = Image.open(img_path).convert('RGB')
        boxes = [line[0] for line in result[0]]
        scores = [line[1][1] for line in result[0]]
        im_show = draw_ocr(image, boxes, txts, scores, font_path=str(font_path))
        im_show = Image.fromarray(im_show)
        im_show.save(str(debug_path))
    except Exception as e:
        print(f"Error al generar imagen de debug: {e}")
    
    return content


#Primeramente se debe rasterizar cada pagina de un pdf a una resolución de 350 antes de llamar a esta función (ya que lee el pdf a partir de su rasterización)
def get_text():
    # Paddleocr supports Chinese, English, French, German, Korean and Japanese
    # You can set the parameter `lang` as `ch`, `en`, `french`, `german`, `korean`, `japan`
    # to switch the language model in order
    ocr = PaddleOCR(use_angle_cls=True, lang='en') # need to run only once to download and load model into memory
    folder_path = Path(__file__).resolve().parent / "books-snaps" 
    file_count = sum(1 for file in folder_path.iterdir() if file.is_file())
    content = ""

    for page in range(1, file_count + 1):
        img_path = Path(__file__).resolve().parent / "books-snaps" / f"snapshot350dpi_page_{page}.png"
        result = ocr.ocr(str(img_path), cls=False)
        result = result[0]
        txts = [line[1][0] for line in result]
        content += f"{get_content_reconstruct(txts)} "
        
    return content

#Lee directamente desde el pdf sin necesidad de rasterizar cada pagina a una resolución DPI de 350
#Hay un estimado de DPI por defecto usado por paddle OCR de 200 DPI.
def get_text_from_pdf(file_name):
    # Paddleocr supports Chinese, English, French, German, Korean and Japanese
    # You can set the parameter `lang` as `ch`, `en`, `french`, `german`, `korean`, `japan`
    # to switch the language model in order
    ocr = PaddleOCR(use_angle_cls=True, lang='en') # need to run only once to download and load model into memory
    pdf_path = Path(__file__).resolve().parent / "books-pdf" / f"{file_name}.pdf"
    
    content = ""
    result = ocr.ocr(str(pdf_path), cls=False)
    for idx in range(len(result)):
        res = result[idx]
        if res == None:
            continue
        txts = [line[1][0] for line in res]
        content += f"{get_content_raw(txts)} "
  
    return content

