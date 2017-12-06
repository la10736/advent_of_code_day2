use std::fs::File;
use std::io::prelude::*;
fn main() {
    let path = std::env::args().nth(1).unwrap();
    let mut f = File::open(path).expect("file not found");
    let mut content = String::new();
    f.read_to_string(&mut content);
    let rows = rows(content);
    let rule = match std::env::args().nth(2) {
        Some(ref opt) if opt == "-q"=> row_quotient::<&Vec<i32>>,
        _ => row_difference::<&Vec<i32>>
    };
    let checksum: i32 = rows.iter().map(rule).sum();
    println!("rows = {:?}", rows);
    println!("checksum = {}", checksum);
}

fn rows<S:AsRef<str>>(data: S) -> Vec<Vec<i32>> {
    data.as_ref().lines().map(
        |l| l.split_whitespace()
            .map(|t| t.parse().unwrap())
            .collect()
    ).collect()
}

fn row_difference<V: AsRef<[i32]>>(row: V) -> i32 {
    let row = row.as_ref();
    row.iter().max().unwrap() - row.iter().min().unwrap()
}

fn row_quotient<V: AsRef<[i32]>>(row: V) -> i32 {
    let row = row.as_ref();
    row.iter().map(|n|
        row.iter().filter_map(|q|
            match n % q {
                0 => Some(n/q),
                _ => None
            }
        ).max().unwrap()
    ).max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_read_rows() {
        assert_eq!(vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]],
                   rows("5 1 9 5\n\
                          7 5 3\n\
                          2 4 6 8"))
    }

    #[test]
    fn get_row_difference() {
        assert_eq!(8, row_difference(vec![5, 1, 9, 5]));
        assert_eq!(4, row_difference(vec![7, 5, 3]));
        assert_eq!(6, row_difference(vec![2, 4, 6, 8]));
    }

    #[test]
    fn get_row_quotient() {
        assert_eq!(4, row_quotient(vec![5, 9, 2, 8]));
        assert_eq!(3, row_quotient(vec![9, 4, 7, 3]));
        assert_eq!(2, row_quotient(vec![3, 8, 6, 4]));
    }
}
