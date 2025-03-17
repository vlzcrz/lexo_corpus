from pathlib import Path
import numpy as np
from rapidocr import RapidOCR

def identificar_columnas_por_indentacion(boxes, txts, scores, tolerancia_x=70, min_cajas_columna=20, tolerancia_y=50):
    if len(boxes) == 0:
        return []
    
    # Extraer posiciones X iniciales (indentación) de cada caja y crear ids únicos
    posiciones_x_iniciales = []
    for i, box in enumerate(boxes):
        box_array = np.array(box)
        x_inicial = np.min(box_array[:, 0])
        x_final = np.max(box_array[:, 0])
        y_inicial = np.min(box_array[:, 1])
        y_final = np.max(box_array[:, 1])
        ancho = x_final - x_inicial
        alto = y_final - y_inicial
        
        posiciones_x_iniciales.append({
            'indice': i,
            'x_inicial': x_inicial,
            'x_final': x_final,
            'y_inicial': y_inicial,
            'y_final': y_final,
            'ancho': ancho,
            'alto': alto,
            'box': box,
            'texto': txts[i],
            'score': scores[i],
            'id': i  # Identificador único para cada caja
        })
    
    # Ordenar todas las cajas por posición Y para facilitar el análisis de cercanía vertical
    posiciones_x_iniciales.sort(key=lambda x: x['y_inicial'])
    
    # Agrupar por indentación similar
    grupos_indentacion = []
    
    for info_box in posiciones_x_iniciales:
        x_inicial = info_box['x_inicial']
        
        # Buscar si existe un grupo con indentación similar
        grupo_encontrado = False
        for grupo in grupos_indentacion:
            if abs(x_inicial - grupo['indentacion_base']) <= tolerancia_x:
                grupo['cajas'].append(info_box)
                grupo_encontrado = True
                break
        
        # Si no encontramos grupo, crear uno nuevo
        if not grupo_encontrado:
            grupos_indentacion.append({
                'indentacion_base': x_inicial,
                'cajas': [info_box]
            })
    
    # Filtrar grupos con suficientes cajas y calcular ancho basado en la caja más ancha
    columnas_potenciales = []
    
    for grupo in grupos_indentacion:
        if len(grupo['cajas']) >= min_cajas_columna:
            # Encontrar la caja más ancha del grupo
            caja_mas_ancha = max(grupo['cajas'], key=lambda x: x['ancho'])
            
            columnas_potenciales.append({
                'x_min': grupo['indentacion_base'],
                'x_max': grupo['indentacion_base'] + caja_mas_ancha['ancho'],
                'ancho': caja_mas_ancha['ancho'],
                'num_cajas': len(grupo['cajas']),
                'cajas_iniciales': grupo['cajas']
            })
    
    # Si no hay columnas potenciales, intentar con menos restricciones
    if len(columnas_potenciales) == 0 and grupos_indentacion:
        grupos_indentacion.sort(key=lambda x: len(x['cajas']), reverse=True)
        
        # Tomar los 2 grupos más numerosos
        for i in range(min(2, len(grupos_indentacion))):
            grupo = grupos_indentacion[i]
            caja_mas_ancha = max(grupo['cajas'], key=lambda x: x['ancho'])
            
            columnas_potenciales.append({
                'x_min': grupo['indentacion_base'],
                'x_max': grupo['indentacion_base'] + caja_mas_ancha['ancho'],
                'ancho': caja_mas_ancha['ancho'],
                'num_cajas': len(grupo['cajas']),
                'cajas_iniciales': grupo['cajas']
            })
    
    # Ordenar las columnas de izquierda a derecha
    columnas_potenciales.sort(key=lambda x: x['x_min'])
    
    # Evitar solapamiento horizontal entre columnas potenciales
    if len(columnas_potenciales) > 1:
        columnas_ajustadas = [columnas_potenciales[0]]
        
        for i in range(1, len(columnas_potenciales)):
            col_actual = columnas_potenciales[i]
            col_anterior = columnas_ajustadas[-1]
            
            # Si hay solapamiento, ajustar los límites
            if col_actual['x_min'] < col_anterior['x_max']:
                # Punto medio entre el fin de la columna anterior y el inicio de la actual
                punto_medio = (col_anterior['x_max'] + col_actual['x_min']) / 2
                col_anterior['x_max'] = punto_medio
                col_actual['x_min'] = punto_medio
                col_anterior['ancho'] = col_anterior['x_max'] - col_anterior['x_min']
                col_actual['ancho'] = col_actual['x_max'] - col_actual['x_min']
            
            columnas_ajustadas.append(col_actual)
        
        columnas_potenciales = columnas_ajustadas
    
    # Conjunto para rastrear cajas ya asignadas y evitar duplicados
    cajas_asignadas = set()
    
    # Segmentar columnas por distancia vertical
    columnas_finales = []
    
    for columna in columnas_potenciales:
        # Recolectar todas las cajas que caen dentro de los límites horizontales de la columna
        cajas_en_columna = []
        
        for i, box in enumerate(boxes):
            if i in cajas_asignadas:
                continue  # Saltar cajas ya asignadas a otra columna
                
            box_array = np.array(box)
            centro_x = np.mean(box_array[:, 0])
            
            # Si el centro de la caja está dentro de los límites horizontales de la columna
            if columna['x_min'] <= centro_x <= columna['x_max']:
                y_inicial = np.min(box_array[:, 1])
                y_final = np.max(box_array[:, 1])
                
                cajas_en_columna.append({
                    'box': box,
                    'texto': txts[i],
                    'score': scores[i],
                    'y_inicial': y_inicial,
                    'y_final': y_final,
                    'altura': y_final - y_inicial,
                    'id': i  # Guardar el índice original
                })
        
        # Ordenar cajas de arriba a abajo
        cajas_en_columna.sort(key=lambda item: item['y_inicial'])
        
        # Agrupar en segmentos (párrafos) basados en la distancia vertical
        segmentos = []
        segmento_actual = []
        
        for i, caja in enumerate(cajas_en_columna):
            if not segmento_actual:
                # Si el segmento está vacío, agregar la primera caja
                segmento_actual.append(caja)
            else:
                # Calcular la distancia vertical entre esta caja y la última en el segmento
                ultima_caja = segmento_actual[-1]
                distancia_y = caja['y_inicial'] - ultima_caja['y_final']
                
                # Si la distancia es pequeña, agregar al segmento actual
                if distancia_y <= tolerancia_y:
                    segmento_actual.append(caja)
                else:
                    # Si la distancia es grande, terminar el segmento actual y empezar uno nuevo
                    if segmento_actual:
                        segmentos.append(segmento_actual)
                    segmento_actual = [caja]
        
        # Agregar el último segmento si no está vacío
        if segmento_actual:
            segmentos.append(segmento_actual)
        
        # Agregar cada segmento como una columna independiente y marcar cajas como asignadas
        for segmento in segmentos:
            if segmento:  # Asegurarse de que no esté vacío
                # Marcar todas las cajas de este segmento como asignadas
                for caja in segmento:
                    cajas_asignadas.add(caja['id'])
                
                columnas_finales.append(segmento)
    
    # Verificar si hay cajas sin asignar
    cajas_sin_asignar = []
    for i, box in enumerate(boxes):
        if i not in cajas_asignadas:
            box_array = np.array(box)
            y_inicial = np.min(box_array[:, 1])
            y_final = np.max(box_array[:, 1])
            
            cajas_sin_asignar.append({
                'box': box,
                'texto': txts[i],
                'score': scores[i],
                'y_inicial': y_inicial,
                'y_final': y_final,
                'id': i
            })
    
    # Si hay cajas sin asignar, intentar agruparlas por proximidad vertical
    if cajas_sin_asignar:        
        # Ordenar por posición vertical
        cajas_sin_asignar.sort(key=lambda item: item['y_inicial'])
        
        # Agrupar en segmentos basados en la distancia vertical
        segmento_actual = []
        segmentos_adicionales = []
        
        for caja in cajas_sin_asignar:
            if not segmento_actual:
                segmento_actual.append(caja)
            else:
                ultima_caja = segmento_actual[-1]
                distancia_y = caja['y_inicial'] - ultima_caja['y_final']
                
                if distancia_y <= tolerancia_y:
                    segmento_actual.append(caja)
                else:
                    if segmento_actual:
                        segmentos_adicionales.append(segmento_actual)
                    segmento_actual = [caja]
        
        if segmento_actual:
            segmentos_adicionales.append(segmento_actual)
        
        # Añadir estos segmentos adicionales a las columnas finales
        for segmento in segmentos_adicionales:
            if segmento:
                columnas_finales.append(segmento)
    
    # Si no encontramos columnas, intentar segmentar todas las cajas por distancia vertical
    if len(columnas_finales) == 0:        
        # Ordenar todas las cajas de arriba a abajo
        todas_cajas = []
        for i, box in enumerate(boxes):
            box_array = np.array(box)
            y_inicial = np.min(box_array[:, 1])
            y_final = np.max(box_array[:, 1])
            
            todas_cajas.append({
                'box': box,
                'texto': txts[i],
                'score': scores[i],
                'y_inicial': y_inicial,
                'y_final': y_final
            })
        
        todas_cajas.sort(key=lambda item: item['y_inicial'])
        
        # Agrupar en segmentos basados en la distancia vertical
        segmentos = []
        segmento_actual = []
        
        for i, caja in enumerate(todas_cajas):
            if not segmento_actual:
                segmento_actual.append(caja)
            else:
                ultima_caja = segmento_actual[-1]
                distancia_y = caja['y_inicial'] - ultima_caja['y_final']
                
                if distancia_y <= tolerancia_y:
                    segmento_actual.append(caja)
                else:
                    if segmento_actual:
                        segmentos.append(segmento_actual)
                    segmento_actual = [caja]
        
        # Agregar el último segmento
        if segmento_actual:
            segmentos.append(segmento_actual)
        
        # Cada segmento es una columna
        columnas_finales = segmentos
        
    return columnas_finales

# Función principal
def procesar_imagen_ocr():
    engine = RapidOCR()

    folder_path = Path(__file__).resolve().parent / "books-snaps" 
    file_count = sum(1 for file in folder_path.iterdir() if file.is_file())
    content = ""

    for page in range(1, file_count + 1):
        img_path = Path(__file__).resolve().parent / "books-snaps" / f"snapshot350dpi_page_{page}.png"

        result = engine(img_path)
        columnas = identificar_columnas_por_indentacion(result.boxes, result.txts, result.scores, )
        
        
        for i, columna in enumerate(columnas):
            content += " ".join([item['texto'] for item in columna])
            
        
    return content
