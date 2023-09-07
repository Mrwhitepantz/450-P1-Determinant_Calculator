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
	let mut matrix_rows: Vec<Vec<f64>> = Vec::with_capacity(n);

	for _ in 0..n {
		let _ = stdin.read_line(&mut buffer);
		let matrix_cols: Vec<f64> = buffer.split_whitespace().map(|s| s.parse::<f64>().unwrap()).collect();
		buffer.clear();
		matrix_rows.push(matrix_cols);
	}
	
	let mut determinant: f64 = 1.0;

	println!("Read Matrix:");
	for row in &matrix_rows {
		print!("[ ");
		for col in row {
			print!("{col} ");
		}
		println!("]");
	}

	println!("-----------------");

	println!("Starting Triangularization");


	for (col,row) in matrix_rows.iter().enumerate() {
		println!("{col}:{}",row[col]);
		determinant *= row[col];
	}
	
	println!("{determinant}");
	return Ok(());

}