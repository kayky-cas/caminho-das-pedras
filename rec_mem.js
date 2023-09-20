function caminhoDasPedras(caminho, cansado = false, pos = 0, table = {}) {
	if (caminho[pos] == "0" || pos >= caminho.length) return 0;
	if (caminho[pos] == "m" && pos == caminho.length - 1) return 1;

	if (table[pos] && table[pos][cansado] != undefined) {
		return table[pos][cansado];
	}

	let res = 0;
	for (let i = 1; i <= 3 - cansado; i++) {
		res += caminhoDasPedras(caminho, i == 3, pos + i, table);
	}

	if (!table[pos]) table[pos] = {};

	table[pos][cansado] = res;

	return res;
}

const args = process.argv.slice(2);

if (args.length != 1) {
	console.error("Uso: node rec_mem.js <caminho>");
	process.exit(1);
}

console.log(caminhoDasPedras(args[0]));
