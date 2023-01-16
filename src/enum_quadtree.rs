/// The QuadTree is a recursive data structure that divides a rectangle into 4 quadrants, each containing other quadrants, or a leaf
pub enum QuadTree<DataT> {
    Leaf {
        data: Vec<DataT>,
        positions: Vec<(u16, u16)>,
        rect_x: u16,
        rect_y: u16,
        rect_w: u16,
        rect_h: u16,
    },
    Quads([Box<QuadTree<DataT>>; 4]),
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

/// A QuadTree is a recursive data structure that divides a rectangle into 4 quadrants, each containing other quadrants, or a leaf
impl<DataT> QuadTree<DataT> {
    /// Construct 4 empty quadrants, each containing other quadrants, or a leaf
    pub fn new(rect_x: u16, rect_y: u16, rect_w: u16, rect_h: u16, depth: u16) -> Self {
        if depth == 0 {
            QuadTree::Leaf {
                data: Vec::new(),
                positions: Vec::new(),
                rect_x,
                rect_y,
                rect_w,
                rect_h,
            }
        } else {
            let rect = divide_into_4(rect_x, rect_y, rect_w, rect_h);
            QuadTree::Quads([
                Box::new(QuadTree::new(rect[0].0, rect[0].1, rect[0].2, rect[0].3, depth - 1)),
                Box::new(QuadTree::new(rect[1].0, rect[1].1, rect[1].2, rect[1].3, depth - 1)),
                Box::new(QuadTree::new(rect[2].0, rect[2].1, rect[2].2, rect[2].3, depth - 1)),
                Box::new(QuadTree::new(rect[3].0, rect[3].1, rect[3].2, rect[3].3, depth - 1)),
            ])
        }
    }
    /// Remove all points from all leaves
    pub fn clear(&mut self) {
        match self {
            QuadTree::Leaf { data, positions, .. } => {
                data.clear();
                positions.clear();
            }
            QuadTree::Quads(quads) => {
                for quad in quads.iter_mut() {
                    quad.clear();
                }
            }
        }
    }
    /// Insert a point into the correct leaf, or return false if it doesn't fit
    fn can_insert(&mut self, x: u16, y: u16) -> bool {
        match self {
            QuadTree::Leaf { rect_x, rect_y, rect_w, rect_h, .. } => {
                x >= *rect_x && x < *rect_x + *rect_w && y >= *rect_y && y < *rect_y + *rect_h
            }
            QuadTree::Quads(quads) => {
                for quad in quads.iter_mut() {
                    //short circuit if we find a leaf that accepts the point
                    if quad.can_insert(x, y) {
                        return true;
                    }
                }
                false
            }
        }
    }
    // Insert a point into the correct leaf, or return false if it doesn't fit
    pub fn insert(&mut self, x: u16, y: u16, data: DataT) -> bool {
        match self {
            QuadTree::Leaf { rect_x, rect_y, rect_w, rect_h, data: leaf_data, positions: leaf_positions, .. } => {
                if x >= *rect_x && x < *rect_x + *rect_w && y >= *rect_y && y < *rect_y + *rect_h {
                    leaf_data.push(data);
                    leaf_positions.push((x, y));
                    true
                } else {
                    false
                }
            }
            QuadTree::Quads(quads) => {
                //unroll the top level for loop to show borrow checker insert is only called once
                if quads[0].can_insert(x, y) {
                    quads[0].insert(x, y, data);
                    return true;
                }
                else if quads[1].can_insert(x, y) {
                    quads[1].insert(x, y, data);
                    return true;
                }
                else if quads[2].can_insert(x, y) {
                    quads[2].insert(x, y, data);
                    return true;
                }
                else if quads[3].can_insert(x, y) {
                    quads[3].insert(x, y, data);
                    return true;
                }
                false
            }
        }
    }
    /// Return a reference to the leaf that contains the point
    fn get_leaf_around(&self, x: u16, y: u16) -> Option<&QuadTree<DataT>> {
        match self {
            QuadTree::Leaf { rect_x, rect_y, rect_w, rect_h, .. } => {
                if x >= *rect_x && x < *rect_x + *rect_w && y >= *rect_y && y < *rect_y + *rect_h {
                    Some(self)
                } else {
                    None
                }
            }
            QuadTree::Quads(quads) => {
                for quad in quads.iter() {
                    //short circuit if we find a leaf that accepts the point
                    if let Some(leaf) = quad.get_leaf_around(x, y) {
                        return Some(leaf);
                    }
                }
                None
            }
        }
    }
    /// Return a mutable reference to the leaf that contains the point
    fn get_mut_leaf_around(&mut self, x: u16, y: u16) -> Option<&mut QuadTree<DataT>> {
        match self {
            QuadTree::Leaf { rect_x, rect_y, rect_w, rect_h, .. } => {
                if x >= *rect_x && x < *rect_x + *rect_w && y >= *rect_y && y < *rect_y + *rect_h {
                    Some(self)
                } else {
                    None
                }
            }
            QuadTree::Quads(quads) => {
                for quad in quads.iter_mut() {
                    //short circuit if we find a leaf that accepts the point
                    if let Some(leaf) = quad.get_mut_leaf_around(x, y) {
                        return Some(leaf);
                    }
                }
                None
            }
        }
    }
    /// Convienience function for get_leaf_around that returns a reference to the vec of data
    pub fn broad_phase(&self, x: u16, y: u16) -> &Vec<DataT> {
        match self.get_leaf_around(x, y).unwrap() {
            QuadTree::Leaf { data, .. } => data,
            _ => unreachable!("get_leaf_around returned a non-leaf"),
        }
    }
    /// Convienience function for get_mut_leaf_around that returns a mutable reference to the vec of data
    pub fn broad_phase_mut(&mut self, x: u16, y: u16) -> &mut Vec<DataT> {
        match self.get_mut_leaf_around(x, y).unwrap() {
            QuadTree::Leaf { data, .. } => data,
            _ => unreachable!("get_leaf_around returned a non-leaf"),
        }
    }
}
use super::GetX;
use super::GetY;
pub fn rebuild_tree<Entity: GetX+GetY>(tree: &mut QuadTree<&mut Entity>, model: &mut Vec<Entity>) {
    tree.clear();
    for i in 0..model.len() {
        let entity = &mut model[i] as *mut Entity;
        let entity = unsafe {&mut *entity};
        //insert a reference to the entity into the tree
        tree.insert(entity.get_x(), entity.get_y(), entity);
    }
}