use std::io;

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let mut buffer = String::new();
	stdin.read_line(&mut buffer).unwrap();
	let n = buffer.trim().parse::<usize>().unwrap();
	buffer.clear();

	// Build n x n matrix from stdin lines
	let mut _matrix_rows: Vec<Vec<f64>> = Vec::with_capacity(n);
	let mut row: usize = 0;
	
	while row < n {
		let line = stdin.read_line(&mut buffer);
		let mut _matrix_cols: Vec<f64> = buffer.split_whitespace().map(|s| s.parse::<f64>().unwrap()).collect();
		buffer.clear();
		_matrix_rows.push(_matrix_cols);
		row += 1;
	}

	let mut determinant: f64 = 1.0;


	for (col,row) in _matrix_rows.iter().enumerate() {
		determinant *= row[col];
	}
	
	println!("{determinant}");
	return Ok(());

}