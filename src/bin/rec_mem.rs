use std::collections::HashMap;

fn caminho_das_pedras(
    caminho: &str,
    cansado: bool,
    table: &mut HashMap<(usize, bool), usize>,
) -> usize {
    match caminho.chars().next() {
        Some('0') | None => 0,
        Some('m') if caminho.len() == 1 => 1,
        _ => match table.get(&(caminho.len(), cansado)) {
            Some(res) => *res,
            None => {
                let res = (1..=caminho.len().min(3 - cansado as usize))
                    .map(|x| caminho_das_pedras(&caminho[x..], x == 3, table))
                    .sum();

                table.insert((caminho.len(), cansado), res);

                res
            }
        },
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        println!("Uso: {} <caminho>", args[0]);
        return;
    }

    println!(
        "{}",
        caminho_das_pedras(&args[1], false, &mut HashMap::new())
    );
}
