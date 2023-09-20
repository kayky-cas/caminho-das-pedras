use std::collections::HashMap;

fn caminho_das_pedras(caminho: &str, cansado: bool) -> usize {
    match caminho.chars().next() {
        Some('0') | None => 0,
        Some('m') if caminho.len() == 1 => 1,
        _ => {
            let mut res = caminho_das_pedras(&caminho[1..], false);

            if caminho.len() >= 2 {
                res += caminho_das_pedras(&caminho[2..], false);
            }

            if caminho.len() >= 3 && !cansado {
                res += caminho_das_pedras(&caminho[3..], true);
            }

            res
        }
    }
}

fn caminho_das_pedras_mem(
    caminho: &str,
    cansado: bool,
    table: &mut HashMap<(usize, bool), usize>,
) -> usize {
    match caminho.chars().next() {
        Some('0') => 0,
        Some('m') if caminho.len() == 1 => 1,
        None => 0,
        _ => match table.get(&(caminho.len(), cansado)) {
            Some(res) => *res,
            None => {
                let res = (1..=caminho.len().min(3 - cansado as usize))
                    .map(|x| caminho_das_pedras_mem(&caminho[x..], x == 3, table))
                    .sum();

                table.insert((caminho.len(), cansado), res);

                res
            }
        },
    }
}

fn caminho_das_pedras_mem_no_rec(caminho: &str) -> usize {
    let caminho = caminho.as_bytes();
    let mut table = vec![[0; 2]; caminho.len()];

    table[0][0] = 1;
    table[0][1] = 1;

    for i in 1..caminho.len() {
        if caminho[i] == b'0' {
            table[i][0] = 0;
            table[i][1] = 0;
            continue;
        }

        let mut res = table[i - 1][0];

        if i >= 2 {
            res += table[i - 2][0];
        }

        table[i][1] = res;

        if i >= 3 {
            res += table[i - 3][1];
        }

        table[i][0] = res;
    }

    table[caminho.len() - 1][0]
}

fn main() {
    let buff = "m001110010101001010101001010100101010011111111111111111111111111111111m";
    let buff = "m01110010m";
    let buff = "m01101m";
    let buff =
        "m1111100101010010101010010101001010100111111001010101001010100101010101111100110111111001111010101010011111110101010110101010101110011111001010101010010101010010111001010101010101010101010101010101010101010100101001101010101001101011011010101011101010101101101101010110101011011011111110110111101011011100101011111110101101110111111011011011010100m";

    println!(
        "{}",
        caminho_das_pedras_mem(&buff[..], false, &mut HashMap::new())
    );
    println!("{}", caminho_das_pedras_mem_no_rec(&buff[..]));

    println!("{}", caminho_das_pedras(&buff[..], false));
}
