use algoritmos::busca_binaria::busca_binaria;

pub mod algoritmos;

fn main() {
    let array = [
        2, 22, 48, 58, 61, 73, 84, 90, 100, 119, 132, 154, 160, 177, 187, 197, 201, 211, 2242,
    ];

    if let Some(busca) = busca_binaria(&array, 177) {
        println!("elemento 12 encontrado na posição {:?}", busca);
    } else {
        print!("elemento 12 não encontrado")
    };

    if let Some(busca) = busca_binaria(
        &[
            "Alice",
            "Ezequias",
            "Ezequiana",
            "Gabriel",
            "Marcelo",
            "Maquerle",
            "Magdalena",
        ],
        "Gabriel",
    ) {
        println!("elemento `Gabriel` encontrado na posição {:?}", busca);
    } else {
        print!("elemento `Gabriel` não encontrado")
    };
}
