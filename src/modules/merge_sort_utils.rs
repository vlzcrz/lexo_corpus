// Función de sorting para vectores aplicando mergesort
pub fn merge_sort(
    vector_pointer_values: &mut Vec<u32>,
    vector_pointer_keys: &mut Vec<String>,
    left: u32,
    right: u32,
) {
    // Condición de salida
    if left >= right {
        return;
    }

    // Delimitador "mid" para separar el vector en 2
    let mid = left + (right - left) / 2;

    merge_sort(vector_pointer_values, vector_pointer_keys, left, mid);
    merge_sort(vector_pointer_values, vector_pointer_keys, mid + 1, right);

    // función merge
    merge(vector_pointer_values, vector_pointer_keys, left, mid, right);
}

pub fn merge(
    vector_pointer_values: &mut Vec<u32>,
    vector_pointer_keys: &mut Vec<String>,
    left: u32,
    mid: u32,
    right: u32,
) {
    // Declaramos la cantidad de valores que existen para el lado derecho y izquierdo del vector
    let q_left: u32 = mid - left + 1;
    let q_right: u32 = right - mid;

    // Vector de referencias mutables

    // Creación vector auxiliar para mantener las referencias de los datos a manipular
    let mut aux_vec_left: Vec<u32> = Vec::with_capacity(q_left as usize);
    let mut aux_vec_right: Vec<u32> = Vec::with_capacity(q_right as usize);

    let mut aux_vec_keys_left: Vec<String> = Vec::with_capacity(q_left as usize);
    let mut aux_vec_keys_right: Vec<String> = Vec::with_capacity(q_right as usize);

    // Copiamos la data al vector auxiliar
    //      Data auxiliar para indexar
    let mut i: u32 = 0;
    let mut j: u32 = 0;

    //      Copia de valores a los vectores auxiliares
    while i < q_left {
        aux_vec_left.push(vector_pointer_values[(left + i) as usize]);
        aux_vec_keys_left.push(vector_pointer_keys[(left + i) as usize].clone());
        i += 1;
    }

    while j < q_right {
        aux_vec_right.push(vector_pointer_values[(mid + j + 1) as usize]);
        aux_vec_keys_right.push(vector_pointer_keys[(mid + j + 1) as usize].clone());
        j += 1;
    }

    // Reiniciamos los valores indexados
    i = 0;
    j = 0;
    let mut k: u32 = left;

    // Modificaremos el vector principal para ordenarlo usando los auxiliares que tienen los valores ya guardados
    while (i < q_left) && (j < q_right) {
        if aux_vec_left[i as usize] <= aux_vec_right[j as usize] {
            vector_pointer_values[k as usize] = aux_vec_left[i as usize];
            vector_pointer_keys[k as usize] = aux_vec_keys_left[i as usize].clone();

            i += 1;
        } else {
            vector_pointer_values[k as usize] = aux_vec_right[j as usize];
            vector_pointer_keys[k as usize] = aux_vec_keys_right[j as usize].clone();
            j += 1;
        }
        k += 1;
    }

    // En caso de que se hayan acabado los valores del vector derecho
    while i < q_left {
        vector_pointer_values[k as usize] = aux_vec_left[i as usize];
        vector_pointer_keys[k as usize] = aux_vec_keys_left[i as usize].clone();
        i += 1;
        k += 1;
    }
    // En caso de que se hayan acabado los valores del vector izquierdo
    while j < q_right {
        vector_pointer_values[k as usize] = aux_vec_right[j as usize];
        vector_pointer_keys[k as usize] = aux_vec_keys_right[j as usize].clone();
        j += 1;
        k += 1;
    }
    return;
}
