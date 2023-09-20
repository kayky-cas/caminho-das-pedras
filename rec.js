function caminhoDasPedras(caminho, cansado = false, pos = 0) {
	if (caminho[pos] == "0" || pos >= caminho.length) {
		return 0;
	}

	if (caminho[pos] == "m" && pos == caminho.length - 1) {
		return 1;
	}

	let sum = 0;

	for (let i = 1; i <= 3 - cansado; i++) {
		sum += caminhoDasPedras(caminho, i == 3, pos + i);
	}

	return sum;
}

const args = process.argv.slice(2);

if (args.length != 1) {
	console.log("Uso: node rec.js <caminho>");
	process.exit(1);
}

console.log(caminhoDasPedras(args[0]));
