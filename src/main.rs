use sudoku_solver::def::Sudoku;
fn main() {
    let board = [
        [0, 8, 0, 0, 3, 0, 4, 0, 0],
        [0, 0, 0, 0, 5, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 4, 5, 8, 0],
        [0, 5, 7, 0, 0, 2, 0, 9, 0],
        [9, 0, 0, 0, 0, 0, 0, 0, 4],
        [0, 3, 0, 4, 0, 0, 6, 5, 0],
        [0, 7, 9, 2, 0, 0, 0, 0, 0],
        [5, 0, 0, 0, 6, 0, 0, 0, 0],
        [0, 0, 6, 0, 4, 0, 0, 2, 0],
    ];

    let sudoku = Sudoku::new(board);
    sudoku.print_board();

    println!("Row: {:?}", sudoku.row);
    println!("Column: {:?}", sudoku.col);
    println!("Block: {:?}", sudoku.block);
    println!("Row Remaining: {:?}", sudoku.row_remaining);
    println!("Column Remaining: {:?}", sudoku.col_remaining);
    println!("Block Remaining: {:?}", sudoku.block_remaining);
}
