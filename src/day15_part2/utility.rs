#[derive(Debug, Clone)]
pub struct LensBox {
    pub lenses: Vec<Lens>,
}

impl LensBox {
    pub fn new() -> Self {
        return LensBox { lenses: Vec::<Lens>::new() };
    }
}

#[derive(Debug, Clone)]
pub struct Lens {
    pub label: String,
    pub focal_length: u32
}

#[derive(Debug, Eq, PartialEq)]
pub enum Operation { Insert, Remove }

#[derive(Debug)]
pub struct Step {
    pub box_index: usize,
    pub operation: Operation,
    pub lens_label: String,
    pub lens_focal_length: Option<u32>
}

pub trait ConditionalModification<T> {
    fn replace_first<F>(&mut self, predicate: F, item: T) -> Option<usize>
        where F: Fn(&T) -> bool;
    fn remove_first<F>(&mut self, predicate: F) -> Option<usize>
        where F: Fn(&T) -> bool;
}

impl<T> ConditionalModification<T> for Vec<T> {
    fn replace_first<F>(&mut self, predicate: F, item: T) -> Option<usize>
        where F: Fn(&T) -> bool {

        let mut matched_index = None;
        for (index, element) in self.iter().enumerate() {
            if predicate(element) {
                matched_index = Some(index);
                break;
            }
        }

        if let Some(replacement_index) = matched_index {
            self[replacement_index] = item;
        }
        return matched_index;
    }

    fn remove_first<F>(&mut self, predicate: F) -> Option<usize>
        where F: Fn(&T) -> bool {

        let mut matched_index = None;
        for (index, element) in self.iter().enumerate() {
            if predicate(element) {
                matched_index = Some(index);
                break;
            }
        }

        if let Some(removal_index) = matched_index {
            self.remove(removal_index);
        }
        return matched_index;
    }
}