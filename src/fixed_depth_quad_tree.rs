/// The QuadTree is a recursive data structure that divides a rectangle into 4 quadrants, each containing other quadrants, or a leaf
/// The default depth is 3, which gives a 8x8 grid of leaves
/// TODO use newtype pattern? Convert free functions into associated functions?
pub type QuadTree<DataT> = QuadTreeDepth3<DataT>;

/// 4 quadrants, each containing 4 quadrants, each containing 4 Leafs (8x8 grid, see README.md)
type QuadTreeDepth3<DataT> = [[[QuadTreeLeaf<DataT>; 4]; 4]; 4];

/// Leaf of the QuadTree
pub struct QuadTreeLeaf<DataT> {
    //Bucket of data within the tree. This is intended to contain references to entities owned by the game model.
    pub data: Vec<DataT>,
    //For simplicity the, positions of the data elements are stored separately from the data.
    positions: Vec<(u16, u16)>,
    //The bounding box of the leaf
    rect_x: u16, rect_y: u16, rect_w: u16, rect_h: u16,
}

/// Trait for an array of 4 QuadTreeLeaves or 4 other Quadrants
pub trait Quadrants{
    type DataT;
    /// Construct 4 empty quadrants, each containing other quadrants, or a leaf
    fn new_empty(rect_x: u16, rect_y: u16, rect_w: u16, rect_h: u16) -> Self;
    /// Remove all points from all leaves
    fn clear(&mut self);
    /// Insert a point into the correct leaf, or return false if it doesn't fit
    fn can_insert(&mut self, x: u16, y: u16) -> bool;
    // Insert a point into the correct leaf, or return false if it doesn't fit
    fn insert(&mut self, x: u16, y: u16, data: Self::DataT) -> bool;
    /// Return a reference to the leaf that contains the point
    fn get_leaf_around(&self, x: u16, y: u16) -> Option<&QuadTreeLeaf<Self::DataT>>;
    /// Return a mutable reference to the leaf that contains the point
    fn get_mut_leaf_around(&mut self, x: u16, y: u16) -> Option<&mut QuadTreeLeaf<Self::DataT>>;
    /// Convienience function for get_leaf_around that returns a reference to the vec of data
    fn broad_phase(&self, x: u16, y: u16) -> &Vec<Self::DataT> {
        &self.get_leaf_around(x, y).unwrap().data
    }
    /// Convienience function for get_mut_leaf_around that returns a mutable reference to the vec of data
    fn broad_phase_mut(&mut self, x: u16, y: u16) -> &mut Vec<Self::DataT> {
        &mut self.get_mut_leaf_around(x, y).unwrap().data
    }
    // Used for debugging
    const DEPTH: usize;
}

///Split a rect into 4 quadrants. This is a utility function used by the QuadTree constructor
fn divide_into_4(rect_x: u16, rect_y: u16, rect_w: u16, rect_h: u16) -> [(u16, u16, u16, u16); 4] {
    let half_w = rect_w / 2;
    let half_h = rect_h / 2;
    [
        (rect_x, rect_y, half_w, half_h),
        (rect_x + half_w, rect_y, half_w, half_h),
        (rect_x, rect_y + half_h, half_w, half_h),
        (rect_x + half_w, rect_y + half_h, half_w, half_h)
    ]
}

/// An array of 4 Quadrants also implements Quadrants.
/// Each depth of the tree is a different type so we use a recursive impl to implement each depth.
impl<InnerQuadrants> Quadrants for [InnerQuadrants; 4] where InnerQuadrants: Quadrants {
    type DataT = InnerQuadrants::DataT;
    /// Construct 4 empty quadrants, each containing other quadrants
    fn new_empty(rect_x: u16, rect_y: u16, rect_w: u16, rect_h: u16) -> Self {
        let rects = divide_into_4(rect_x, rect_y, rect_w, rect_h);
        [
            InnerQuadrants::new_empty(rects[0].0, rects[0].1, rects[0].2, rects[0].3),
            InnerQuadrants::new_empty(rects[1].0, rects[1].1, rects[1].2, rects[1].3),
            InnerQuadrants::new_empty(rects[2].0, rects[2].1, rects[2].2, rects[2].3),
            InnerQuadrants::new_empty(rects[3].0, rects[3].1, rects[3].2, rects[3].3)
        ]
    }
    fn clear(&mut self) {
        for quadrant_or_leaf in self.iter_mut() {
            quadrant_or_leaf.clear();
        }
    }
    fn can_insert(&mut self, x: u16, y: u16) -> bool {
        for quadrant_or_leaf in self.iter_mut() {
            //this will recurse down the tree until it finds a leaf
            //short circuit if we find a leaf that accepts the point
            if quadrant_or_leaf.can_insert(x, y) {
                return true;
            }
        }
        false
    }
    fn insert(&mut self, x: u16, y: u16, data: Self::DataT) -> bool {
        //unroll the top level for loop to show borrow checker insert is only called once
        if self[0].can_insert(x, y) {
            self[0].insert(x, y, data);
            return true;
        }
        else if self[1].can_insert(x, y) {
            self[1].insert(x, y, data);
            return true;
        }
        else if self[2].can_insert(x, y) {
            self[2].insert(x, y, data);
            return true;
        }
        else if self[3].can_insert(x, y) {
            self[3].insert(x, y, data);
            return true;
        }
        false
    }
    /// Return a reference to the vector of points in the leaf that contains the point
    fn get_leaf_around(&self, x: u16, y: u16) -> Option<&QuadTreeLeaf<Self::DataT>> {
        for quadrant_or_leaf in self.iter() {
            //this will recurse down the tree until it finds a leaf
            //short circuit if we find a leaf that could contain the point
            if let Some(vec) = quadrant_or_leaf.get_leaf_around(x, y) {
                return Some(vec);
            }
        }
        None
    }
    /// Return a mutable reference to the leaf that contains the point
    fn get_mut_leaf_around(&mut self, x: u16, y: u16) -> Option<&mut QuadTreeLeaf<Self::DataT>> {
        for quadrant_or_leaf in self.iter_mut() {
            //this will recurse down the tree until it finds a leaf
            //short circuit if we find a leaf that could contain the point
            if let Some(vec) = quadrant_or_leaf.get_mut_leaf_around(x, y) {
                return Some(vec);
            }
        }
        None
    }

    const DEPTH: usize = InnerQuadrants::DEPTH + 1;
}

/// An array of 4 QuadTreeLeafs implements Quadrants.
/// This is the bottom of the recursive impl chain, it interacts with the leaf instead of another quadrant.
impl<DataT> Quadrants for [QuadTreeLeaf<DataT>; 4] {
    type DataT = DataT;
    /// Construct 4 empty leaves
    fn new_empty(rect_x: u16, rect_y: u16, rect_w: u16, rect_h: u16) -> Self {
        let rects = divide_into_4(rect_x, rect_y, rect_w, rect_h);
        [
            QuadTreeLeaf::new_empty(rects[0].0, rects[0].1, rects[0].2, rects[0].3),
            QuadTreeLeaf::new_empty(rects[1].0, rects[1].1, rects[1].2, rects[1].3),
            QuadTreeLeaf::new_empty(rects[2].0, rects[2].1, rects[2].2, rects[2].3),
            QuadTreeLeaf::new_empty(rects[3].0, rects[3].1, rects[3].2, rects[3].3)
        ]
    }
    fn clear(&mut self) {
        for leaf in self.iter_mut() {
            leaf.clear();
        }
    }
    fn can_insert(&mut self, x: u16, y: u16) -> bool {
        for leaf in self.iter_mut() {
            //short circuit if we find a leaf that accepts the point
            if leaf.can_insert(x, y) {
                return true;
            }
        }
        false
    }
    fn insert(&mut self, x: u16, y: u16, data: Self::DataT) -> bool {
        //unroll the top level for loop to show borrow checker insert is only called once
        if self[0].can_insert(x, y) {
            self[0].insert(x, y, data);
            return true;
        }
        else if self[1].can_insert(x, y) {
            self[1].insert(x, y, data);
            return true;
        }
        else if self[2].can_insert(x, y) {
            self[2].insert(x, y, data);
            return true;
        }
        else if self[3].can_insert(x, y) {
            self[3].insert(x, y, data);
            return true;
        }
        false
    }
    /// Return a reference to the vector of points in the leaf that contains the point
    fn get_leaf_around(&self, x: u16, y: u16) -> Option<&QuadTreeLeaf<DataT>> {
        for leaf in self.iter() {
            //short circuit if we find a leaf that could contain the point
            if leaf.can_insert(x, y) {
                return Some(&leaf);
            }
        }
        None
    }
    /// Return a mutable reference to the leaf that contains the point
    fn get_mut_leaf_around(&mut self, x: u16, y: u16) -> Option<&mut QuadTreeLeaf<Self::DataT>> {
        for leaf in self.iter_mut() {
            //short circuit if we find a leaf that could contain the point
            if leaf.can_insert(x, y) {
                return Some(leaf);
            }
        }
        None
    }
    //This is a Quadrant of 4 leaves, so the depth is 1
    const DEPTH: usize = 1;
}

/// A QuadTree leaf with a constructor and a method to insert a point
impl<DataT> QuadTreeLeaf<DataT> {
    fn new_empty(rect_x: u16, rect_y: u16, rect_w: u16, rect_h: u16) -> Self {
        QuadTreeLeaf {data: Vec::new(), positions: Vec::new(), rect_x, rect_y, rect_w, rect_h}
    }
    fn clear(&mut self) {
        self.data.clear();
        self.positions.clear();
    }
    fn can_insert(&self, x: u16, y: u16) -> bool {
        x >= self.rect_x && x <= self.rect_x + self.rect_w && y >= self.rect_y && y <= self.rect_y + self.rect_h
    }
    fn insert(&mut self, x: u16, y: u16, data: DataT) -> bool {
        if self.can_insert(x, y) {
            self.data.push(data);
            self.positions.push((x, y));
            true
        } else {
            false
        }
    }
}

use super::GetX;
use super::GetY;

pub fn rebuild_from_model<Entity: GetX+GetY>(tree: &mut QuadTree<&mut Entity>, model: &mut Vec<Entity>) {
    tree.clear();
    for i in 0..model.len() {
        let entity = &mut model[i] as *mut Entity;
        //// SAFETY: This is safe because the tree is cleared before being filled.
        let entity = unsafe {&mut *entity};
        //insert a reference to the entity into the tree
        tree.insert(entity.get_x(), entity.get_y(), entity);
    }
}

pub fn build_new_from_model<'a, 'b, Entity: GetX+GetY>(model: &'a mut Vec<Entity>) -> QuadTree<&'b mut Entity> {
    let mut tree = QuadTree::new_empty(0, 0, 1000, 1000);
    for i in 0..model.len() {
        let entity = &mut model[i] as *mut Entity;
        //// SAFETY: This is safe because the tree is new before being filled.
        let entity =  unsafe{&mut *entity};
        //insert a reference to the entity into the tree
        tree.insert(entity.get_x(), entity.get_y(), entity);
    }
    tree
}

/// A version that returns a QuadTree that owns clones of the entities
pub fn build_owned_from_model<Entity: GetX+GetY+Clone>(model: &mut Vec<Entity>) -> QuadTree<Entity> {
    let mut tree = QuadTree::new_empty(0, 0, 1000, 1000);
    for i in 0..model.len() {
        let entity = model[i].clone();
        //insert a reference to the entity into the tree
        tree.insert(entity.get_x(), entity.get_y(), entity);
    }
    tree
}