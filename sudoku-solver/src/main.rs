fn main() {
    let mut sudoku: [[usize;9];9] =
        [
            [0,3,0,0,0,2,1,0,0],
            [8,0,1,0,0,6,0,7,0],
            [0,5,0,0,8,0,0,0,0],
            [0,0,0,0,0,7,0,0,9],
            [4,0,2,0,6,0,5,0,0],
            [0,8,0,0,0,0,0,0,0],
            [0,0,0,2,0,0,0,5,0],
            [1,0,6,0,4,0,2,0,0],
            [0,0,3,0,0,0,0,0,0],
        ];

    solve(&mut sudoku);
}

pub fn is_possible(sudoku: &[[usize;9];9], x: usize, y: usize, value: usize) -> bool {
    for i in 0..9 {
        if sudoku[x][i] == value {return false}
    }
    for i in 0..9 {
        if sudoku[i][y] == value {return false}
    }
    let square_x = x/3;
    let square_y = y/3;
    for i in 0..3 {
        for j in 0..3 {
            if sudoku [(square_x * 3) + i][(square_y * 3) +j] == value {return false}
        }
    }
    true
}

fn solve (sudoku: &mut [[usize;9];9]) {
    for x in 0..9 {
        for y in 0..9 {
            if sudoku[x][y] == 0 {
                for n in 1..=9 {
                    if is_possible(sudoku, x, y, n) {
                        sudoku[x][y] = n;
                        solve(sudoku);
                        sudoku[x][y] = 0;
                    }
                }
                return
            }
        }
    }
    print_grid(sudoku);
}


fn print_grid(sudoku: &[[usize;9];9]) {
    println!("===========================");
    for column in sudoku {
        println!("{:?}", column);
    }
    println!("===========================");
}