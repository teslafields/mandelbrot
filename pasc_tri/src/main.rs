//! # This is the Pascal's triangle builder
//!
//! ## Pascal's triangle rules:
//! 1. The *nth* row and the *kth* column is denoted as (n, k);
//! 2. Position (0, 0) is equal to 1;
//! 3. Position (n, k) is obtained by the following equation *(n, k) = (n-1, k-1) + (n-1, k)*, for
//!    any non-negative 0 <= *k* <= *n*.
//!
//! ## Example of Pascal's triangle for n = 5 and k = 5:
//!``` 
//! 1
//! 1 1
//! 1 2 1
//! 1 3 3 1
//! 1 4 6 4 1
//! ```
//!
//! ## Usage:
//! `./pasc_tri n_row`

use std::collections::HashMap;


fn calc_position(row: i32, col: i32, triangle: &mut Vec<Vec<i32>>, computed: &mut HashMap<String, i32>) -> i32 {
    let key = format!("{}{}", row, col);
    if computed.contains_key(&key) {
        return *computed.get(&key).unwrap();
    }
    let mut n: i32 = 1;
    if col == 0 || row == 0 || col == row {
    } else {
        n = calc_position(row - 1, col - 1, triangle, computed)
                + calc_position(row -1, col, triangle, computed);
    }
    triangle[row as usize][col as usize] = n;
    computed.insert(key, n);
    return n;

}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} ROW", args[0]);
        std::process::exit(1);
    }
    let row: i32 = args[1].parse::<i32>().unwrap_or_else(|_| {
        println!("Error parsing {}", args[1]);
        std::process::exit(1);
    });

    let mut triangle: Vec<Vec<i32>> = vec![vec![0; row as usize]; row as usize];
    triangle[0][0] = 1;

    let mut computed = HashMap::new();
    for i in 0..row {
        calc_position(row-1, i, &mut triangle, &mut computed);
    }
    for row in triangle {
        println!("{:?}", row);
    }
}
