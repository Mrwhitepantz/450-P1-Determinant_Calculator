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

	for col_index in 0..n {
		println!("Column: {col_index}");
		let mut max_val:  f64 = 0.0;
		let mut max_index = 0;
		for row_index in col_index..n {
			println!("Check [{row_index}, {col_index}] for max");
			if matrix_rows[row_index][col_index].abs() > max_val{
				max_val = matrix_rows[row_index][col_index];
				max_index = row_index;
				println!("New max at [{max_index}, {col_index}]");
			}
		}
		if max_index != col_index {
			println!("swapping max to row {col_index}");
			matrix_rows.swap(max_index,col_index);
			determinant *= -1.0;
		}
		for row_index in col_index+1..n{
			if matrix_rows[row_index][col_index] != 0.0{
				let scale_factor = matrix_rows[row_index][col_index] / matrix_rows[col_index][col_index];
				println!("scale factor: {scale_factor}");
				for ele_index in col_index..n {
					matrix_rows[row_index][ele_index] = matrix_rows[row_index][ele_index] - scale_factor * matrix_rows[col_index][ele_index];
				}
			}
		}
		println!("new matrix:");
		for row in &matrix_rows {
			print!("[ ");
			for col in row {
				print!("{col} ");
			}
			println!("]");
		}
	}


	for i in 0..n {
		determinant *= matrix_rows[i][i];
	}
	
	println!("{determinant}");
	return Ok(());

}