
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut out = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            out[i][j] = matrix[j][i];
        }
    }

    out
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    print!("[\n");
    for i in 0..3 {
        print!("  [");
        for j in 0..3 {
            if j != 2 {
                print!("{} ", matrix[i][j]);
            } else {
                print!("{}", matrix[i][j]);
            }
        }
        if i != 2 {
            print!("],\n");
        } else {
            print!("]\n");
        }
    }
    print!("]\n");
}

fn main() {
    let matrix = [
        [101, 102, 103],
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}
