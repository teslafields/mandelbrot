/// Pascal's Triangle
/// 1
/// 1 1
/// 1 2 1
/// 1 3 3 1
/// 1 4 6 4 1

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
