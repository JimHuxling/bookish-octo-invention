use self::Content::{Possibilities, Assigned};


pub struct Grid<'a> {
    pub content: [[Content; 9]; 9],
    pub unassigned_cells: &'a Vec<UnassignedCell>
}

pub struct UnassignedCell {
    pub row: usize,
    pub col: usize
}


pub enum Content {
    Possibilities(Vec<u64>),
    Assigned(u64)
}

impl<'a> Grid<'a> {
    fn create_with(&self, row: usize, col: usize, possibility: u64, unassigned_cells: Vec<UnassignedCell>) -> Grid {
        Grid {
            content: *&self.content,
            unassigned_cells: &unassigned_cells
        }
    }
}

impl Copy for UnassignedCell {}
impl Clone for UnassignedCell {
     fn clone(&self) -> Self {
         UnassignedCell {
             row: self.row,
             col: self.col
        }
     }
}


pub fn solve(grid: Grid) -> Option<Grid> {
    let mut unassigned_cells = grid.unassigned_cells.clone();
    unassigned_cells.reverse();
    let option_cell = &unassigned_cells.pop();
    unassigned_cells.reverse();

    match option_cell {
        &None => {
            if no_conflict(&grid) {
                Some(grid)
            } else {
                None
            }
        },
        &Some(unassigned_cell) => {
            let UnassignedCell {row: row, col: col} = unassigned_cell;
            match grid.content[col][row]{
                Assigned(_)=> {
                    if no_conflict(&grid) {
                        Some(grid)
                    } else {
                        None
                    }
                },
                Possibilities(ref mut possibilities) => {
                    match possibilities.pop() {
                        Some(possibility) => {
                            let newGrid = grid.create_with(row, col, possibility, unassigned_cells.clone());
                            solve(newGrid)
                        },
                        None => None
                    }
                }
            }
        }
    }
}

fn no_conflict(_grid: &Grid) -> bool {
    false
}
