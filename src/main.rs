use crate::find::busca_binaria;

mod find;

fn main() {
    let array = [
        2, 22, 48, 58, 61, 73, 84, 90, 100, 119, 132, 154, 160, 177, 187, 197, 201, 211, 2242,
    ];

    if let Some(busca) = busca_binaria(&array, 23){
        println!("elemento 12 encontrado na posição {:?}", busca);
    } else {
        print!("elemento 12 não encontrado")
    };
    
    
    
}
