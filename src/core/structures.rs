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
                Possibilities(ref possibilities) => {
                    //TODO
                    None
                }
            }
        }
    }
}

fn no_conflict(_grid: &Grid) -> bool {
    false
}
