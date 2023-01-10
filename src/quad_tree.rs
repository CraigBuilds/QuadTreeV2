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
pub type QuadTree<'a, DataT> = [[[QuadTreeLeaf<'a, DataT>; 4]; 4]; 4];

/// 4 quadrants, each containing 4 quadrants, each containing 4 quadrants, each containing 4 Leafs (16x16 grid)
//pub type DeepQuadTree<'a> = [[[[QuadTreeLeaf<'a>; 4]; 4]; 4]; 4];

/// Leaf of the QuadTree
pub struct QuadTreeLeaf<'a, DataT> {
    pub vec: Vec<(u16, u16, &'a DataT)>,
    rect_x: u16, rect_y: u16, rect_w: u16, rect_h: u16,
}

/// Trait for an array of 4 QuadTreeLeaves or 4 other Quadrants
pub trait Quadrants<'a>{
    type DataT;
    ///Construct 4 empty quadrants, each containing other quadrants, or a leaf
    fn new(rect_x: u16, rect_y: u16, rect_w: u16, rect_h: u16) -> Self;
    ///Remove all points from all leaves
    fn clear(&mut self);
    /// Insert a point into the correct leaf, or return false if it doesn't fit
    fn insert(&mut self, x: u16, y: u16, data: &'a Self::DataT) -> bool;
    /// Return a reference to the leaf that contains the point
    fn get_leaf_around(&self, x: u16, y: u16) -> Option<&QuadTreeLeaf<Self::DataT>>;
    ///Get all data in all leaves
    fn all(&self) -> Vec<&'a Self::DataT>;
    //used for debugging
    const DEPTH: usize;
}

/// An array of 4 Quadrants also implements Quadrants
impl<'a, InnerQuadrants> Quadrants<'a> for [InnerQuadrants; 4] where InnerQuadrants: Quadrants<'a> {
    type DataT = InnerQuadrants::DataT;
    ///Construct 4 empty quadrants, each containing other quadrants
    fn new(rect_x: u16, rect_y: u16, rect_w: u16, rect_h: u16) -> Self {
        let rects = divide_into_4(rect_x, rect_y, rect_w, rect_h);
        [
            InnerQuadrants::new(rects[0].0, rects[0].1, rects[0].2, rects[0].3),
            InnerQuadrants::new(rects[1].0, rects[1].1, rects[1].2, rects[1].3),
            InnerQuadrants::new(rects[2].0, rects[2].1, rects[2].2, rects[2].3),
            InnerQuadrants::new(rects[3].0, rects[3].1, rects[3].2, rects[3].3)
        ]
    }
    fn clear(&mut self) {
        for quadrant_or_leaf in self.iter_mut() {
            quadrant_or_leaf.clear();
        }
    }
    fn insert(&mut self, x: u16, y: u16, data: &'a Self::DataT) -> bool {
        for quadrant_or_leaf in self.iter_mut() {
            //this will recurse down the tree until it finds a leaf
            //short circuit if we find a leaf that accepts the point
            if quadrant_or_leaf.insert(x, y, data) {
                return true;
            }
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
    ///Get all data in all leaves
    fn all(&self) -> Vec<&'a Self::DataT> {
        let mut vec = Vec::new();
        for quadrant_or_leaf in self.iter() {
            vec.extend(quadrant_or_leaf.all());
        }
        vec
    }
    const DEPTH: usize = InnerQuadrants::DEPTH + 1;
}

///QuadTreeLeaf cannot implement Quadrants, because the generic [InnerQuadrants; 4] impl would conflict with the [QuadTreeLeaf<DataT>; 4] impl
// impl<DataT> !Quadrants<DataT> for QuadTreeLeaf<DataT> {}

/// An array of 4 QuadTreeLeafs implements Quadrants
impl<'a, DataT> Quadrants<'a> for [QuadTreeLeaf<'a, DataT>; 4] {
    type DataT = DataT;
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
    fn insert(&mut self, x: u16, y: u16, data: &'a DataT) -> bool {
        for leaf in self.iter_mut() {
            //short circuit if we find a leaf that accepts the point
            if leaf.insert(x, y, data) {
                return true;
            }
        }
        false
    }
    /// Return a reference to the vector of points in the leaf that contains the point
    fn get_leaf_around(&self, x: u16, y: u16) -> Option<&QuadTreeLeaf<DataT>> {
        for leaf in self.iter() {
            //short circuit if we find a leaf that could contain the point
            if leaf.valid_point(x, y) {
                return Some(&leaf);
            }
        }
        None
    }
    ///Get all data in all leaves
    fn all(&self) -> Vec<&'a Self::DataT> {
        let mut vec = Vec::new();
        for leaf in self.iter() {
            vec.extend(leaf.all());
        }
        vec
    }
    const DEPTH: usize = 1;
}

/// A QuadTree leaf with a constructor and a method to insert a point
impl<'a, DataT> QuadTreeLeaf<'a, DataT> {
    fn new(rect_x: u16, rect_y: u16, rect_w: u16, rect_h: u16) -> Self {
        QuadTreeLeaf {vec: Vec::new(), rect_x, rect_y, rect_w, rect_h}
    }
    fn clear(&mut self) {
        self.vec.clear();
    }
    fn insert(&mut self, x: u16, y: u16, data: &'a DataT) -> bool {
        if self.valid_point(x, y) {
            self.vec.push((x, y, data));
            true
        } else {
            false
        }
    }
    fn valid_point(&self, px: u16, py: u16) -> bool {
        px >= self.rect_x && px <= self.rect_x + self.rect_w && py >= self.rect_y && py <= self.rect_y + self.rect_h
    }
    fn all(&self) -> Vec<&'a DataT> {
        let mut result = Vec::new();
        for (_, _, data) in self.vec.iter() {
            result.push(*data);
        }
        result
    }
}