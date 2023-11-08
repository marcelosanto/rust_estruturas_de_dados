pub fn busca_binaria<T: PartialOrd + Copy>(array: &[T], buscado: T) -> Option<usize> {
    if array.is_empty() {
        return None;
    } else {
        let mut inicio = 0;
        let mut fim = array.len() - 1;

        while inicio <= fim {
            let meio = (inicio + fim) / 2;

            match array[meio] {
                x if x == buscado => {
                    return Some(meio);
                }
                x if x < buscado => inicio = meio + 1,
                _ => fim = meio - 1,
            }
        }
    }
    None
}
