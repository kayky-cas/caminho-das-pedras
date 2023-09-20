use std::collections::HashMap;

fn caminho_das_pedras(caminho: &str, cansado: bool) -> usize {
    match caminho.chars().next() {
        Some('0') | None => 0,
        Some('m') if caminho.len() == 1 => 1,
        _ => (1..=caminho.len().min(3 - cansado as usize))
            .map(|x| caminho_das_pedras(&caminho[x..], x == 3))
            .sum(),
    }
}

fn caminho_das_pedras_mem(
    caminho: &str,
    cansado: bool,
    pos: usize,
    table: &mut HashMap<(usize, bool), usize>,
) -> usize {
    match caminho.chars().next() {
        Some('0') => 0,
        Some('m') if caminho.len() == 1 => 1,
        None => 0,
        _ => match table.get(&(pos, cansado)) {
            Some(res) => *res,
            None => {
                let res = (1..=caminho.len().min(3 - cansado as usize))
                    .map(|x| caminho_das_pedras(&caminho[x..], x == 3))
                    .sum();

                table.insert((pos, cansado), res);

                res
            }
        },
    }
}

fn main() {
    let buff = "m0011100101010010101010010101001010100111111m";
    let _buff =
        "m0011100101010010101010010101001010100111111001010101001010100101010101111100110111111001111010101010011111110101010110101010101110011111001010101010010101010010111001010101010101010101010101010101010101010100101001101010101001101011011010101011101010101101101101010110101011011011111110110111101011011100101011111110101101110111111011011011010100m";

    println!("{}", caminho_das_pedras(&buff[..], false));

    println!(
        "{}",
        caminho_das_pedras_mem(&buff[..], false, 0, &mut HashMap::new())
    );
}
