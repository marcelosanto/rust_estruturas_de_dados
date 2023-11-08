pub fn busca_binaria(array: &[i32], buscado: i32) -> Option<&str> {
  
  if array.is_empty() {
    None
  } else {
    let mut inicio = 0;
    let mut fim = array.len() - 1;
    let mut encontrado = false;

    while inicio <= fim {
        let meio = (inicio+fim)/2;

        match array[meio] {
            x if x == buscado => {
              encontrado = !encontrado;
              break;
            },
            x if x < buscado => inicio = meio + 1,
            _ => fim = meio - 1
        }
    }

   match  encontrado {
      true => Some("Encontrado"),
      false => Some("Nao encontrado")
   }
  }
}