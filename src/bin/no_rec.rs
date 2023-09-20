use std::collections::HashMap;

fn caminho_das_pedras(caminho: &str) -> usize {
    let mut table = HashMap::with_capacity(caminho.len());

    table.insert((0, false), 1);
    table.insert((0, true), 1);

    for (i, ch) in caminho.chars().enumerate().skip(1) {
        if ch == '0' {
            continue;
        }

        let res = (1..=2).flat_map(|x| table.get(&(i - x, false))).sum();
        table.insert((i, true), res);
        table.insert((i, false), res + table.get(&(i - 3, true)).unwrap_or(&0));
    }

    *table.get(&(caminho.len() - 1, false)).unwrap_or(&0)
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        println!("Uso: {} <caminho>", args[0]);
        return;
    }

    println!("{}", caminho_das_pedras(&args[1]));
}
