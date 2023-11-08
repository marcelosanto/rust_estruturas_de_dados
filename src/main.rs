use crate::find::busca_binaria;

mod find;

fn main() {
    let busca = busca_binaria(&[0,2,4,6,8,10,12,14], 12).expect("Numero nao encontrado");
    println!("{:?}", busca);
}
