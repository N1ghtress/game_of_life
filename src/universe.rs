use std::fmt::{
    Display,
    Write
};

use rand::{Rng};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1
}

pub struct Universe {
    generation: u32,
    width: u32,
    height: u32,
    underpopulation: u8,
    overpopulation: u8,
    aging: Vec<u8>,
    reproduction: Vec<u8>,
    cells: Vec<Cell>
}

impl Universe {
    pub fn new(
        width: u32,
        height: u32,
        life_rate: f64,
        underpopulation: u8,
        overpopulation: u8,
        reproduction: Vec<u8>) -> Universe
    {
        Universe {
            generation: 0,
            width,
            height,
            underpopulation,
            overpopulation,
            aging: (underpopulation..overpopulation).collect(),
            reproduction,
            cells: {
                let mut cells = vec![Cell::Dead; (width*height) as usize];
                let mut rng = rand::thread_rng();

                for i in 0..cells.len() {
                    cells[i] = match rng.gen::<f64>() {
                        x if x <= life_rate => Cell::Alive,
                        _ => Cell::Dead
                    }
                }

                cells
            }
        }    
    }

    pub fn from_fixed_cells(width: u32, height: u32) -> Universe {
        Universe {
            generation: 0,
            width,
            height,
            underpopulation: 2,
            overpopulation: 3,
            aging: vec![2, 3],
            reproduction: vec![3],
            cells: {
                let mut cells = Vec::new();
                
                for i in 0..width*height {
                    if i % 4 == 0 || i % 13 == 0 {
                        cells.push(Cell::Alive);
                    }
                    cells.push(Cell::Dead);
                }

                cells
            }
        }
    }

    pub fn from_life_rate(width: u32, height: u32, life_rate: f64) -> Universe {
        Universe {
            generation: 0,
            width,
            height,
            underpopulation: 2,
            overpopulation: 3,
            aging: vec![2, 3],
            reproduction: vec![3],
            cells: {
                let mut cells = vec![Cell::Dead; (width*height) as usize];
                let mut rng = rand::thread_rng();

                for i in 0..cells.len() {
                    cells[i] = match rng.gen::<f64>() {
                        x if x <= life_rate => Cell::Alive,
                        _ => Cell::Dead
                    }
                }

                cells
            }
        }
    }

    pub fn tick(&mut self) {
        let mut next_generation = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let alive_neighbours = self.alive_neighbour_count(row, col);
                
                match (cell, alive_neighbours) {
                    (Cell::Alive, x) if x < self.underpopulation => next_generation[idx] = Cell::Dead,
                    (Cell::Alive, x) if x > self.overpopulation => next_generation[idx] = Cell::Dead,
                    (Cell::Alive, x) if self.aging.contains(&x) => next_generation[idx] = Cell::Alive,
                    (Cell::Dead, x) if self.reproduction.contains(&x) => next_generation[idx] = Cell::Alive,
                    (otherwise, _) => next_generation[idx] = otherwise
                }
            }
        }

        self.generation += 1;
        self.cells = next_generation;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

	pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn alive_neighbour_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbour_row = (row + delta_row) % self.height;
                let neighbour_col = (col + delta_col) % self.width;
                let idx = self.get_index(neighbour_row, neighbour_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Life in generation: {}\n", self.generation))?;
        for row in 0..self.height {
			for col in 0..self.width {
				let cell = self.cells[self.get_index(row, col)];
				let cell_str = match cell {
					Cell::Alive => "??? ",
					Cell::Dead => "??? ",
				};

				f.write_str(cell_str)?;
			}
			f.write_char('\n')?;
		}

		Ok(())
    }
}