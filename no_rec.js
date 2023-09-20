function caminhoDasPedras(caminho) {
	const table = Array.from({ length: caminho.length }, () => [0, 0]);
	table[0] = [1, 1];

	for (let i = 1; i < caminho.length; i++) {
		if (caminho[i] === "0") {
			continue;
		}

		let res = table[i - 1][0];
		if (i >= 2) {
			res += table[i - 2][0];
		}

		table[i][1] = res;

		if (i >= 3) {
			res += table[i - 3][1];
		}

		table[i][0] = res;
	}

	return table[caminho.length - 1][0];
}

const args = process.argv.slice(2);

if (args.length != 1) {
	console.log("Uso: node no_rec.js <caminho>");
	process.exit(1);
}

console.log(caminhoDasPedras(args[0]));
