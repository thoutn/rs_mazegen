use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;

use std::rc::{Rc, Weak};
use crate::maze::{cell, grid};


pub struct Image {
    cell_size: u16,
    wall_thickness: u16,

    size: u16,

}


impl Image {
    pub fn new(cell_size: u16, wall_thickness: u16) -> Self {
        Self {
            cell_size,
            wall_thickness,

            size: (cell_size + wall_thickness) as u16,

        }
    }


    fn set_size(&self, size: u16) -> u32 {
        (size * self.cell_size + (size + 1) * self.wall_thickness) as u32
    }


    pub fn save(&self, grid: &grid::Grid, filename: &str) {
        let width = self.set_size(grid.width);
        let height= self.set_size(grid.height);

        // colours specified
        let white = Rgb::from([240, 240, 240]);
        let black = Rgb::from([0, 0, 0]);

        let mut img: RgbImage = ImageBuffer::new(width, height);
        // clears the background and sets it to a WHITE
        draw_filled_rect_mut(&mut img,
                             Rect::at(0, 0).of_size(width, height),
                             white);

        for row in 0..(grid.height) as usize {
            for col in 0..(grid.width ) as usize {
                let cell = Rc::clone(&grid.cells[row][col].as_ref().unwrap());
                let c = (&*cell).borrow();

                let x1 = (c.col * self.size) as u32;
                let y1 = (c.row * self.size) as u32;
                let x2 = ((c.col + 1) * self.size) as u32;
                let y2 = ((c.row + 1) * self.size) as u32;

                // draws a line with the specific thickness stored in 'self.wall_thickness'
                let mut draw_line = |sx: u32, sy: u32, ex: u32, ey: u32| {
                    if sx == ex {
                        draw_filled_rect_mut(&mut img,
                                             Rect::at(sx as i32, sy as i32).of_size(
                                                 self.wall_thickness as u32,
                                                 (ey - sy) + self.wall_thickness as u32),
                                             black);
                    } else {
                        draw_filled_rect_mut(&mut img,
                                             Rect::at(sx as i32, sy as i32).of_size(
                                                 (ex - sx) + self.wall_thickness as u32,
                                                 self.wall_thickness as u32),
                                             black);
                    }
                };

                // draws the top and left walls
                if c.top.is_none() {
                    draw_line(x1, y1, x2, y1);
                }
                if c.left.is_none() {
                    draw_line(x1, y1, x1, y2);
                }

                // draws all internal walls
                if c.bottom.as_ref().is_none() || !c.is_linked_to(Rc::downgrade(
                        &c.bottom.as_ref().unwrap().upgrade().unwrap())) {
                    draw_line(x1, y2, x2, y2);
                }
                if c.right.as_ref().is_none() || !c.is_linked_to(Rc::downgrade(
                        &c.right.as_ref().unwrap().upgrade().unwrap())) {
                    draw_line(x2, y1, x2, y2);
                }

            }
        }

        img.save(filename).unwrap();
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::algos::sidewinder::*;


    #[test]
    fn draw() {
        let mut grid = grid::Grid::new(10, 10);
        grid.init_grid();

        build_maze(&grid);

        let img = Image::new(20, 2);
        img.save(&grid, "test_maze.png");

        assert!(true);
    }
}