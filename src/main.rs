fn tranpose(a: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut new: Vec<Vec<u8>> = vec![];
    let range = (a[0]).len();
    for i in 0..range {
        let mut temps: Vec<u8> = vec![];
        for j in 0..a.len() {
            temps.push(a[j][i]);
        }
        new.push(temps);
    }
    return new;
}

// -> Vec<u8>

fn multiply_by_row(a: &Vec<u8>, b: &Vec<u8>) -> u8 {
    if a.len() == b.len() {
        let mut summation: u8 = 0;
        for i in 0..a.len() {
            summation += a[i] * b[i];
        }
        return summation;
    } else {
        return 0;
    }
}

fn multiply_matrices(a: Vec<Vec<u8>>, b: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let b_transpose: Vec<Vec<u8>> = tranpose(b);
    let mut outward_matrix: Vec<Vec<u8>> = vec![];
    for i in 0..a.len() {
        let mut temps: Vec<u8> = vec![];
        for j in 0..b_transpose.len() {
            temps.push(multiply_by_row(&a[i], &b_transpose[j]))
        }
        outward_matrix.push(temps);
    }
    return outward_matrix;
}

fn main() {
    // println!("{:#?}", tranpose(vec![vec![1, 2, 3], vec![4, 5, 6]]))
    // print!("{} \n", multiply_by_row(vec![1, 2, 3], vec![4, 5, 6]));
    println!(
        "{:#?}",
        multiply_matrices(
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            vec![vec![1, 2, 1], vec![0, 1, 0], vec![0, 1, 1]],
        )
    );
}
