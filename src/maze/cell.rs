use core::hash::Hash;
use std::cmp::{ PartialEq, Eq };


pub trait Same: PartialEq + Eq + Hash {}


#[derive(PartialEq, Eq, Hash)]
pub struct Cell<'a, T>
where T: Same
{
    pub row: u16,
    pub col: u16,

    links: Vec<&'a T>,

    pub top: Option<&'a T>,
    pub bottom: Option<&'a T>,
    pub left: Option<&'a T>,
    pub right: Option<&'a T>,
}


impl<'a, T: Same> Same for Cell<'a, T> {}


impl<'a, T: Same> Cell<'a, T> {
    pub fn new(row: u16, col: u16) -> Self {
        Self {
            row,
            col,

            links: Vec::new(),

            top: None,
            bottom: None,
            left: None,
            right: None,
        }
    }


    pub fn link_to(&mut self, cell: &'a mut T) {
        self.links.push(cell);
    }


    pub fn has_linked_cells(&self) -> bool {
        !self.links.is_empty()
    }


    pub fn is_linked_to(&self, cell: &T) -> bool {
        self.links.contains(&cell)
    }


    pub fn neighbours(&self) -> Vec<Option<&T>> {
        let mut lst: Vec<Option<&T>> = Vec::new();

        if self.top.is_some() { lst.push(self.top); }
        if self.bottom.is_some() { lst.push(self.bottom); }
        if self.left.is_some() { lst.push(self.left); }
        if self.right.is_some() { lst.push(self.right); }

        lst

    }

}