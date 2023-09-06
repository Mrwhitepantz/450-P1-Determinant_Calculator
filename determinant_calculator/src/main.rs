use std::io;

fn main() -> io::Result<()> {
	// take stdin to lines iterator
	let mut lines = io::stdin().lines();
	let n = lines.next().unwrap()?.trim().parse::<f64>().unwrap() as usize;

	// Build n x n matrix from stdin lines
	let mut matrix_rows: Vec<Vec<f64>> = Vec::with_capacity(n);
	while let Some(line) = lines.next() {
		let mut matrix_cols:Vec<f64> = Vec::with_capacity(n);
		while let Some(num) = line?.split_whitespace().next() {
			matrix_cols.push(num.parse::<f64>().unwrap());
		}
		matrix_rows.push(matrix_cols);
	}

	let mut determinant: f64 = 1.0;
	for (i,ele) in matrix_rows.iter().enumerate() {
		determinant *= ele[i];
	}
	
	return Ok(());

}