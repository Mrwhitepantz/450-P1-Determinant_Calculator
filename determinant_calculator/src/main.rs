use std::io;

fn main() -> io::Result<()> {
	// set up stdin for reading
	let stdin = io::stdin();
	let mut buffer = String::new();
	
	// read matrix dimension and clear buffer
	stdin.read_line(&mut buffer).unwrap();
	let n = buffer.trim().parse::<usize>().unwrap();
	buffer.clear();

	// build n x n matrix from rows provided through stdin
	let mut _matrix_rows: Vec<Vec<f64>> = Vec::with_capacity(n);

	for _ in 0..n {
		let line = stdin.read_line(&mut buffer);
		let mut _matrix_cols: Vec<f64> = buffer.split_whitespace().map(|s| s.parse::<f64>().unwrap()).collect();
		buffer.clear();
		_matrix_rows.push(_matrix_cols);
	}

	let mut determinant: f64 = 1.0;


	for (col,row) in _matrix_rows.iter().enumerate() {
		determinant *= row[col];
	}
	
	println!("{determinant}");
	return Ok(());

}