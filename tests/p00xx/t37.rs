
use leetcode_rs::p00xx::p37::*;

#[derive(Debug, Clone)]
struct TestCase {
    input: Input,
}

#[test]
fn t37() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        // Box::new(Solution1) as Box<dyn Solution>,
    ];
    
    let test_cases = [
        TestCase {
            input: vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ],
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, mut test_case) in test_cases.iter().cloned().enumerate() {

            print_sudoku(&test_case.input);
            solution.solve_sudoku(&mut test_case.input);

            assert!(is_valid_sudoku(&test_case.input),
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {:?}\n",
                i, j, &test_case.input);
        }
    }
}

fn print_sudoku(board: &[Vec<char>]) {
    println!("Sudoku Status:");
    for (i, row) in board.iter().enumerate() {
        if i % 3 == 0 {
            println!("\t----------------------");
        }
        let row_content: String = row.iter()
            .enumerate()
            .map(|(i, ch)|
                if i % 3 == 0 {
                    format!("|{} ", ch)
                } else {
                    format!("{} ", ch)
                }
            ).collect();
        println!("\t{}|", row_content);
    }
    println!("\t----------------------");
}

fn is_valid_sudoku(board: &[Vec<char>]) -> bool {

    use std::collections::HashSet;

    for i in 0..9 {
        let mut rows    = HashSet::new();
        let mut columns = HashSet::new();
        let mut subbox  = HashSet::new();

        for j in 0..9 {
            // verify rows
            let row_ch = board[i][j];
            if row_ch != '.' && rows.contains(&row_ch) {
                return false
            } else {
                rows.insert(row_ch);
            }
            
            // verify columns
            let column_ch = board[j][i];
            if column_ch != '.' && columns.contains(&column_ch) {
                return false
            } else {
                columns.insert(column_ch);
            }

            // verify sub-box
            let row    = 3 * (i / 3) + j / 3;
            let column = 3 * (i % 3) + j % 3;
            let subbox_ch = board[row][column];
            if subbox_ch != '.' && subbox.contains(&subbox_ch) {
                return false
            } else {
                subbox.insert(subbox_ch);
            }
        }
    }
   
    true
}

