//todo move the new function out of the trait and make it a const fn
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

/// 4 quadrants, each containing 4 quadrants, each containing 4 Leafs (8x8 grid, see README.md)
pub type QuadTree<'a> = [[[QuadTreeLeaf<'a>; 4]; 4]; 4];

/// 4 quadrants, each containing 4 quadrants, each containing 4 quadrants, each containing 4 Leafs (16x16 grid)
//pub type DeepQuadTree<'a> = [[[[QuadTreeLeaf<'a>; 4]; 4]; 4]; 4];

/// Leaf of the QuadTree
pub struct QuadTreeLeaf<'a> {
    pub vec: Vec<(&'a u16, &'a u16)>,
    rect_x: u16, rect_y: u16, rect_w: u16, rect_h: u16,
}

/// Trait for an array of 4 QuadTreeLeaves or 4 other Quadrants
pub trait Quadrants<'a> {
    type InnerType;
    ///Construct 4 empty quadrants, each containing other quadrants, or a leaf
    fn new(rect_x: u16, rect_y: u16, rect_w: u16, rect_h: u16) -> Self;
    ///Remove all points from all leaves
    fn clear(&mut self);
    /// Insert a point into the correct leaf, or return false if it doesn't fit
    fn insert(&mut self, x: &'a u16, y: &'a u16) -> bool;
    /// Return a reference to the leaf that contains the point
    fn get_leaf_around(&self, x: &'a u16, y: &'a u16) -> Option<&QuadTreeLeaf>;
    //used for debugging
    const DEPTH: usize;
}

/// An array of 4 Quadrants also implements Quadrants
impl<'a, T> Quadrants<'a> for [T; 4] where T: Quadrants<'a> {
    type InnerType = T;
    ///Construct 4 empty quadrants, each containing other quadrants
    fn new(rect_x: u16, rect_y: u16, rect_w: u16, rect_h: u16) -> Self {
        let rects = divide_into_4(rect_x, rect_y, rect_w, rect_h);
        [
            T::new(rects[0].0, rects[0].1, rects[0].2, rects[0].3),
            T::new(rects[1].0, rects[1].1, rects[1].2, rects[1].3),
            T::new(rects[2].0, rects[2].1, rects[2].2, rects[2].3),
            T::new(rects[3].0, rects[3].1, rects[3].2, rects[3].3)
        ]
    }
    fn clear(&mut self) {
        for quadrant_or_leaf in self.iter_mut() {
            quadrant_or_leaf.clear();
        }
    }
    fn insert(&mut self, x: &'a u16, y: &'a u16) -> bool {
        for quadrant_or_leaf in self.iter_mut() {
            //this will recurse down the tree until it finds a leaf
            //short circuit if we find a leaf that accepts the point
            if quadrant_or_leaf.insert(x, y) {
                return true;
            }
        }
        false
    }
    /// Return a reference to the vector of points in the leaf that contains the point
    fn get_leaf_around(&self, x: &'a u16, y: &'a u16) -> Option<&QuadTreeLeaf> {
        for quadrant_or_leaf in self.iter() {
            //this will recurse down the tree until it finds a leaf
            //short circuit if we find a leaf that could contain the point
            if let Some(vec) = quadrant_or_leaf.get_leaf_around(x, y) {
                return Some(vec);
            }
        }
        None
    }
    const DEPTH: usize = T::DEPTH + 1;
}

/// An array of 4 QuadTreeLeafs implements Quadrants
impl<'a> Quadrants<'a> for [QuadTreeLeaf<'a>; 4] {
    type InnerType = QuadTreeLeaf<'a>;
    ///Construct 4 empty leaves
    fn new(rect_x: u16, rect_y: u16, rect_w: u16, rect_h: u16) -> Self {
        let rects = divide_into_4(rect_x, rect_y, rect_w, rect_h);
        [
            QuadTreeLeaf::new(rects[0].0, rects[0].1, rects[0].2, rects[0].3),
            QuadTreeLeaf::new(rects[1].0, rects[1].1, rects[1].2, rects[1].3),
            QuadTreeLeaf::new(rects[2].0, rects[2].1, rects[2].2, rects[2].3),
            QuadTreeLeaf::new(rects[3].0, rects[3].1, rects[3].2, rects[3].3)
        ]
    }
    fn clear(&mut self) {
        for leaf in self.iter_mut() {
            leaf.clear();
        }
    }
    fn insert(&mut self, x: &'a u16, y: &'a u16) -> bool {
        for leaf in self.iter_mut() {
            //short circuit if we find a leaf that accepts the point
            if leaf.insert(x, y) {
                return true;
            }
        }
        false
    }
    /// Return a reference to the vector of points in the leaf that contains the point
    fn get_leaf_around(&self, x: &'a u16, y: &'a u16) -> Option<&QuadTreeLeaf> {
        for leaf in self.iter() {
            //short circuit if we find a leaf that could contain the point
            if leaf.valid_point(x, y) {
                return Some(&leaf);
            }
        }
        None
    }
    const DEPTH: usize = 1;
}

/// A QuadTree leaf with a constructor and a method to insert a point
impl<'a> QuadTreeLeaf<'a> {
    fn new(rect_x: u16, rect_y: u16, rect_w: u16, rect_h: u16) -> Self {
        QuadTreeLeaf {vec: Vec::new(), rect_x, rect_y, rect_w, rect_h}
    }
    fn clear(&mut self) {
        self.vec.clear();
    }
    fn insert(&mut self, x: &'a u16, y: &'a u16) -> bool {
        if self.valid_point(x, y) {
            self.vec.push((x, y));
            true
        } else {
            false
        }
    }
    fn valid_point(&self, px: &'a u16, py: &'a u16) -> bool {
        let px = *px;
        let py = *py;
        px >= self.rect_x && px <= self.rect_x + self.rect_w && py >= self.rect_y && py <= self.rect_y + self.rect_h
    }
}