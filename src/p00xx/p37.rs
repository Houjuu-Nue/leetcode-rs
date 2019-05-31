//!
//! Sudoku Solver
//!
//! https://leetcode.com/problems/sudoku-solver/
//!
//! Write a program to solve a Sudoku puzzle by filling the empty cells.
//!
//! A sudoku solution must satisfy **all of the following rules**:
//!
//! 1. Each of the digits `1-9` must occur exactly once in each row.
//!
//! 2. Each of the digits `1-9` must occur exactly once in each column.
//!
//! 3. Each of the the digits `1-9` must occur exactly once in each of the 9 `3x3` sub-boxes of the grid.
//!
//! Empty cells are indicated by the character `'.'`.
//!
//! ![](https://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Sudoku-by-L2G-20050714.svg/250px-Sudoku-by-L2G-20050714.svg.png)
//!
//! A sudoku puzzle
//!
//! ![](https://upload.wikimedia.org/wikipedia/commons/thumb/3/31/Sudoku-by-L2G-20050714_solution.svg/250px-Sudoku-by-L2G-20050714_solution.svg.png)
//!
//! ...and its solution numbers marked in red.
//!
//! **Note:**
//! - The given board contain only digits `1-9` and the character `'.'`.
//!
//! - You may assume that the given Sudoku puzzle will have a single unique solution.
//!
//! - The given board size is always `9x9`.
//!

#![allow(unused_variables)]
#![allow(dead_code)]

pub type Input = Vec<Vec<char>>;
pub trait Solution {
    fn solve_sudoku(&self, board: &mut Vec<Vec<char>>);
}

// -----------------------------------------------------------------------------
use std::collections::HashSet;

/// Approach 0
pub struct Solution0;
impl Solution for Solution0 {

    fn solve_sudoku(&self, board: &mut Vec<Vec<char>>) {
        
        let areas = build_status(&board);

        while let Some((area, index)) = choose_area(&areas) {

        }
        
        unimplemented!()
    }
}

fn choose_area(areas: &AreaStatus) -> Option<(AreaType, usize)> {

    let (mut area, mut index, mut min) = (AreaType::Row, 0, 9);

    for i in 0..9 {
        if areas.rows[i].leave < min {
            area = AreaType::Row;
            index = i;
            min = areas.rows[i].leave;
        }
    }
    for i in 0..9 {
        if areas.columns[i].leave < min {
            area = AreaType::Column;
            index = i;
            min = areas.columns[i].leave;
        }
    }
    for i in 0..9 {
        if areas.subboxs[i].leave < min {
            area = AreaType::Subbox;
            index = i;
            min = areas.subboxs[i].leave;
        }
    }

    if min == 9 {
        None
    } else {
        Some((area, index))
    }
}

struct AreaStatus {
    rows   : [RowArea; 9],
    columns: [ColumnArea; 9],
    subboxs: [SubboxArea; 9],
}

fn build_status(board: &[Vec<char>]) -> AreaStatus {

    // rows
    let mut rows: [RowArea; 9] = Default::default();
    for i in 0..9 {
        let mut area = RowArea::default();
        let mut candidate: HashSet<char> = (1..=9).map(|num| num as u8 as char).collect();
        for j in 0..9 {
            let ch = board[i][j];
            if ch == '.' {
                area.leave += 1;
            } else {
                candidate.remove(&ch);
            }
            area.maps.insert(j, ch);
            area.row = i;
        }
        area.miss = candidate.into_iter().collect();
        rows[i] = area;
    }

    // columns
    let mut columns: [ColumnArea; 9] = Default::default();
    for i in 0..9 {
        let mut area = ColumnArea::default();
        let mut candidate: HashSet<char> = (1..=9).map(|num| num as u8 as char).collect();
        for j in 0..9 {
            let ch = board[j][i];
            if ch == '.' {
                area.leave += 1;
            } else {
                candidate.remove(&ch);
            }
            area.maps.insert(j, ch);
            area.column = i;
        }
        area.miss = candidate.into_iter().collect();
        columns[i] = area;
    }

    // sub-box
    let mut subboxs: [SubboxArea; 9] = Default::default();
    for i in 0..9 {
        let mut area = SubboxArea::default();
        let mut candidate: HashSet<char> = (1..=9).map(|num| num as u8 as char).collect();
        for j in 0..9 {
            let row    = 3 * (i / 3) + j / 3;
            let column = 3 * (i % 3) + j % 3;
            let ch = board[row][column];
            if ch == '.' {
                area.leave += 1;
            } else {
                candidate.remove(&ch);
            }
            area.maps.insert(j, ch);
            area.sub_box = i;
        }
        area.miss = candidate.into_iter().collect();
        subboxs[i] = area;
    }

    AreaStatus { rows, columns, subboxs }
}

trait Area {
    fn try_fill_number(&mut self, board: &mut [Vec<char>]) -> Result<FillResult, ()>;
    fn is_can_number_fill(&self, board: &[Vec<char>], number: char, row: usize, column: usize) -> bool;
}

enum AreaType { Row, Column, Subbox }

struct FillResult {
    typ: AreaType,
    number: char,
    second_index: usize,
}

#[derive(Default)]
struct RowArea {
    maps: Vec<char>,
    miss: Vec<char>,
    row: usize,
    leave: usize,
}
impl Area for RowArea {

    fn try_fill_number(&mut self, board: &mut [Vec<char>]) -> Result<FillResult, ()> {

        for column in 0..9 {
            if self.maps[column] == '.' {
                for &candidate in self.miss.iter() {
                    if self.is_can_number_fill(&board, candidate, self.row, column) {
                        let fill = FillResult {
                            typ: AreaType::Row,
                            number: candidate,
                            second_index: column,
                        };
                        return Ok(fill)
                    }
                }
            }
        }

        Err(())
    }

    fn is_can_number_fill(&self, board: &[Vec<char>], number: char, row: usize, column: usize) -> bool {
        
        false
    }
}


#[derive(Default)]
struct ColumnArea {
    maps: Vec<char>,
    miss: Vec<char>,
    column: usize,
    leave: usize,
}
impl Area for ColumnArea {
    
    fn try_fill_number(&mut self, board: &mut [Vec<char>]) -> Result<FillResult, ()> {
        unimplemented!()
    }

    fn is_can_number_fill(&self, board: &[Vec<char>], number: char, row: usize, column: usize) -> bool {
        false
    }
}

#[derive(Default)]
struct SubboxArea {
    maps: Vec<char>,
    miss: Vec<char>,
    sub_box: usize,
    leave: usize,
}
impl Area for SubboxArea {

    fn try_fill_number(&mut self, board: &mut [Vec<char>]) -> Result<FillResult, ()> {
        unimplemented!()
    }

    fn is_can_number_fill(&self, board: &[Vec<char>], number: char, row: usize, column: usize) -> bool {
        false
    }
}
// -----------------------------------------------------------------------------

