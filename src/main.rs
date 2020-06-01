fn main() {
    
	let mut sieve: Vec<i32> = Vec::new();
	let mut primes: Vec<i32> = Vec::new();

	let limit = 1000;

	for n in 2..(limit + 1) {
		sieve.push(n);
	}

	let mut llave = sieve.clone();

	for (i,ele) in sieve.iter().enumerate() {
		if llave[i] != -1 {
	 		primes.push(*ele);
	 		llave[i] = -1;
	 		for (j,ele2) in sieve.iter().enumerate() {
	 			if (llave[j] != -1) && (ele2 % ele == 0) {
	 				llave[j] = -1
	 			}
	 		}
		}
	}

	println!("{:?}", primes);



}
