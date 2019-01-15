use std::sync::Arc;
pub enum List<T> {
    Node(T, Box<List<T>>),
    End
}

pub struct ListIterator<'a, T> {
    current: &'a List<T>
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List::End
    }

    pub fn add(self, e: T) -> List<T> {
        List::Node(e, Box::new(self))
    }

    pub fn iter(&self) ->ListIterator<T> {
        ListIterator{current: self}
    }
}

impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        match self.current {
            List::Node(elem, next) => {self.current = next; Some(elem)},
            List::End => None
        }
    }
}