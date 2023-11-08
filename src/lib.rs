pub mod algoritmos;

#[cfg(test)]
mod tests {
    use crate::algoritmos::busca_binaria::busca_binaria;

    #[test]
    fn procure_por_valor_em_array_retorne_a_posicao() {
        let busca = busca_binaria(&[1, 2, 3, 4, 5, 6], 4);
        assert_eq!(busca, Some(3))
    }

    #[test]
    fn procure_por_valor_em_array_retorne_erro() {
      let busca = busca_binaria(&[1, 2, 3, 4, 5, 6], 40);
      assert_eq!(busca, None)
  }

}
