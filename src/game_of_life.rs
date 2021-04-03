use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use crate::dom;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub struct Universe {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Cell>,
}

impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe { width, height, cells }
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

struct Canvas {
    grid_color: &'static str,
    dead_color: &'static str,
    alive_color: &'static str,
    cell_size: u32,
    ctx: CanvasRenderingContext2d,
    universe: Universe,
    height: u32,
    width: u32,
}

impl Canvas {
    pub fn new() -> Canvas {
        let canvas = dom::canvas("canvas");
        let ctx = dom::canvas_context::<CanvasRenderingContext2d>(&canvas, "2d");

        let cell_size = 20;
        let width = canvas.client_width() as u32 / cell_size;
        let height = canvas.client_height() as u32 / cell_size;
        let universe = Universe::new(64, 64);

        Canvas {
            grid_color: "#CCCCCC",
            dead_color: "#FFFFFF",
            alive_color: "#000000",
            cell_size,
            ctx,
            universe,
            height,
            width,
        }
    }

    pub fn draw_grid(&self) {
        self.ctx.begin_path();
        self.ctx.set_stroke_style(&JsValue::from_str(self.grid_color));
        (0..self.width).for_each(|i| {
            self.ctx.move_to((i * (self.cell_size + 1) + 1).into(), 0.0);
            self.ctx.line_to(
                (i * (self.cell_size + 1) + 1).into(),
                ((self.cell_size + 1) * self.height + 1).into(),
            );
        });
        (0..self.height).for_each(|j| {
            self.ctx.move_to(0.0, (j * (self.cell_size + 1) + 1).into());
            self.ctx.line_to(
                ((self.cell_size + 1) * self.width + 1).into(),
                (j * (self.cell_size + 1) + 1).into(),
            );
        });
        self.ctx.stroke();
    }

    pub fn draw_cells(&self) {
        self.ctx.begin_path();

        (0..self.universe.height).for_each(|row| {
            (0..self.universe.width).for_each(|col| {
                let idx = self.universe.get_index(row, col);
                let color = match self.universe.cells[idx] {
                    Cell::Dead => self.dead_color,
                    Cell::Alive => self.alive_color,
                };
                self.ctx.set_fill_style(&JsValue::from_str(color));
                self.ctx.fill_rect(
                    (col * (self.cell_size + 1) + 1).into(),
                    (row * (self.cell_size + 1) + 1).into(),
                    self.cell_size.into(),
                    self.cell_size.into(),
                );
            });
        });

        self.ctx.stroke();
    }

    pub fn step(&mut self) {
        self.universe.tick();
    }

    pub fn render(&self) {
        self.draw_grid();
        self.draw_cells();
    }
}

#[wasm_bindgen]
#[allow(dead_code)]
pub fn game_of_life() -> Result<(), JsValue> {
    let canvas = Canvas::new();
    let canvas = Rc::new(RefCell::new(canvas));

    {
        let canvas = canvas.clone();
        dom::set_interval(50, move || {
            canvas.borrow_mut().step();
        });
    }

    dom::request_animation_frame(move |_t, _dt| {
        canvas.borrow().render();
    });

    Ok(())
}
