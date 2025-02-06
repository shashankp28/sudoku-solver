use crate::def::Sudoku;

impl Sudoku {
    pub fn print_board(&self) {
        println!("+-------+-------+-------+");
        for i in 0..9 {
            if i % 3 == 0 && i != 0 {
                println!("|-------+-------+-------|");
            }
            print!("| "); // Left border
            for j in 0..9 {
                if j % 3 == 0 && j != 0 {
                    print!("| ");
                }
                if self.row[i][j] == 0 {
                    print!(". ");
                } else {
                    print!("{} ", self.row[i][j]);
                }
            }
            println!("|");
        }
        println!("+-------+-------+-------+");
    }
}
