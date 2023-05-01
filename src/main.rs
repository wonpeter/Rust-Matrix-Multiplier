// Date: Aug 14, 2022
// Description: For your convenience, matrices can be printed
//              out by uncommenting print statements.
//
//              Border condition assumption:
//              To calculate convolutions, we apply a padding
//              of size 2. Thus, when calculating the result
//              of a convolution with [-1, 0, 1] applied
//              horizontally, we use padding of size 2 on the
//              left and right side of the randomized matrix.

use std::env;
use std::time::Instant;
use rand;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 3 {
        panic!("2 arguments required: rows columns");
    }

    let rows: usize = args[1].trim().parse().expect("Invalid rows argument");
    let cols: usize = args[2].trim().parse().expect("Invalid cols argument");

    let arr: Vec<u8> = construct_randomized_matrix(rows, cols);

    // println!("=== Original matrix ===");
    // print_2d_array_u8(&arr, rows, cols);

    // compute Dy
    let dy_start = Instant::now();
    let dy: Vec<i16> = compute_dy(&arr, rows, cols);
    let dy_duration = dy_start.elapsed();
    // println!("=== Dy ===");
    // print_2d_array_i16(&dy, rows + 2, cols);

    // Compute Dx
    let dx_start = Instant::now();
    let dx: Vec<i16> = compute_dx(&arr, rows, cols);
    let dx_duration = dx_start.elapsed();
    // println!("=== Dx ===");
    // print_2d_array_i16(&dx, rows, cols + 2);

    println!("=== Results ===");
    println!("Dx min: {} max: {} duration: {:?}", get_min(&dx), get_max(&dx), dx_duration);
    println!("Dy min: {} max: {} duration: {:?}", get_min(&dy), get_max(&dy), dy_duration);
}

// Constructs a matrix of specified dimensions with random non-negative values
fn construct_randomized_matrix(rows: usize, cols: usize) -> Vec<u8> {
    let mut arr: Vec<u8> = vec![0; rows * cols];

    for row in 0..rows {
        for col in 0..cols {
            arr[row * cols + col] = rand::random();
        }
    }

    return arr;
}

// Calculates convolution of 2D matrix arr and [-1, 0, 1] (applied horizontally).
// By applying horizontally, [-1, 0, 1] is treated as the 1x3 matrix
// [[-1, 0, 1]].
fn compute_dx(arr: &Vec<u8>, rows: usize, cols: usize) -> Vec<i16> {
    let new_cols: usize = cols + 2;
    let mut dx: Vec<i16> = vec![0; rows * new_cols];

    // Compute values of first 2 and last 2 columns, where padding is used. For optimization, the 0
    // of [-1, 0, 1] is ignored.
    for row in 0..rows {
        dx[row * new_cols + new_cols - 1] = arr[row * cols + cols - 1] as i16;
        dx[row * new_cols] = -1 * arr[row * cols] as i16;

        if cols > 1 {
            dx[row * new_cols + new_cols - 2] = arr[row * cols + cols - 2] as i16;
            dx[row * new_cols + 1] = -1 * arr[row * cols + 1] as i16;
        }
    }

    // Compute inner values of resulting matrix. Once again, for optimization, the 0 of [-1, 0, 1]
    // is ignored.
    if cols > 2 {
        for row in 0..rows {
            for col in 0..cols - 2 {
                dx[row * new_cols + 2 + col] = arr[row * cols + col] as i16 - arr[row * cols + col + 2] as i16;
            }
        }
    }

    return dx;
}

// Calculates convolution of 2D matrix arr and [-1, 0, 1] (applied vertically).
// By applying vertically, [-1, 0, 1] is treated as the 3x1 matrix
// [[-1], [0], [1]].
fn compute_dy(arr: &Vec<u8>, rows: usize, cols: usize) -> Vec<i16> {
    let new_rows: usize = rows + 2;
    let mut dy: Vec<i16> = vec![0; cols * new_rows];

    // Compute values of first 2 and last 2 rows, where padding is used. For optimization, the 0 of
    // [-1, 0, 1] is ignored.
    for col in 0..cols {
        dy[cols * new_rows - col - 1] = arr[cols * rows - col - 1] as i16;
        dy[col] = -1 * arr[col] as i16;

        if rows > 1 {
            dy[cols * (new_rows - 1) - col - 1] = arr[cols * (rows - 1) - col - 1] as i16;
            dy[cols + col] = -1 * arr[cols + col] as i16;
        }
    }

    // Compute inner values of resulting matrix. Once again, for optimization, the 0 of [-1, 0, 1]
    // is ignored.
    if rows > 2 {
        for row in 0..rows - 2 {
            for col in 0..cols {
                dy[(row + 2) * cols + col] = arr[row * cols + col] as i16 - arr[(row + 2) * cols + col] as i16;
            }
        }
    }

    return dy;
}

// Utility used to print vector of unsigned char
fn print_2d_array_u8(arr: &[u8], rows: usize, cols: usize) {
    println!("Array of size {}x{}\nRaw values:", rows, cols);

    for i in 0..rows*cols {
        print!("{},", arr[i]);
    }

    println!("\nFormatted values:");

    for row in 0.. rows {
        print!("[");
        for col in 0.. cols {
            print!("{},", arr[row * cols + col]);
        }
        println!("]");
    }

    println!("]");
    return;
}

// Utility used to print vector of 16-bit ints
fn print_2d_array_i16(arr: &[i16], rows: usize, cols: usize) {
    println!("Array of size {}x{}\nRaw values:", rows, cols);

    for i in 0..rows*cols {
        print!("{},", arr[i]);
    }

    println!("\nFormatted values:");

    for row in 0.. rows {
        print!("[");
        for col in 0.. cols {
            print!("{},", arr[row * cols + col]);
        }
        println!("]")
    }

    println!("]");
    return;
}

fn get_min(matrix: &Vec<i16>) -> i16 {
    let value = matrix.iter().min();

    return match value {
        Some(min) => *min,
        None => 0
    }
}

fn get_max(matrix: &Vec<i16>) -> i16 {
    let value = matrix.iter().max();

    return match value {
        Some(max) => *max,
        None => 0
    }
}
