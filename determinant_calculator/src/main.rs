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
	
	for i in 0..n {
		println!("Column: {i}");
		let mut max: f64 = 0.0;
		let mut max_index = 0;
		for (index,row) in matrix_rows.iter_mut().enumerate() {
			if index >= i{
				println!("Checking for max at [{index}, {i}]");
				if row[i] > max {
					println!("New max found");
					max = row[i];
					max_index = index
				}
			}	
		}
		println!("Column max index: {max_index} needs to be in row {i}");
		if max_index != i {
			println!("swapping row {max_index} with row {i}");
			matrix_rows.swap(i,max_index);
			determinant *= -1.0;
		}
		println!("");
		println!("Starting Scaling");
		for (index, row) in matrix_rows.iter_mut().enumerate() {
			println!("Index: {index} i: {i}");
		    if index > i {
				println!("index greater than i");
				let scale_factor = row[i]/ *(&mut matrix_rows[i][i]);
				println!("scale factor: {} / {} = {}", row[i], &mut matrix_rows[i][i],scale_factor);
				for (j, mut col) in row.iter().enumerate() {
					println!("changing [{index}, {j}]");
					println!("should be {col} - {scale_factor}*{}",&mut matrix_rows[i][j]);
					matrix_rows[index][j] = col - scale_factor* (*(&mut matrix_rows[i][j]));
					println!("now is: {}",&mut matrix_rows[index][j]);
				}
			}
		}

		println!("New Matrix:");
		for row in &matrix_rows {
			print!("[ ");
			for col in row {
				print!("{col} ");
			}
			println!("]");
		}
	}

	for (col,row) in matrix_rows.iter().enumerate() {
		println!("{col}:{}",row[col]);
		determinant *= row[col];
	}
	
	println!("{determinant}");
	return Ok(());

}