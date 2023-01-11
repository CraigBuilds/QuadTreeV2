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
pub type QuadTree<DataT> = [[[QuadTreeLeaf<DataT>; 4]; 4]; 4];

/// 4 quadrants, each containing 4 quadrants, each containing 4 quadrants, each containing 4 Leafs (16x16 grid)
//pub type DeepQuadTree = [[[[QuadTreeLeaf; 4]; 4]; 4]; 4];

/// Leaf of the QuadTree
pub struct QuadTreeLeaf<DataT> {
    pub vec: Vec<(u16, u16, DataT)>,
    rect_x: u16, rect_y: u16, rect_w: u16, rect_h: u16,
}

/// Trait for an array of 4 QuadTreeLeaves or 4 other Quadrants
pub trait Quadrants{
    type DataT;
    ///Construct 4 empty quadrants, each containing other quadrants, or a leaf
    fn new(rect_x: u16, rect_y: u16, rect_w: u16, rect_h: u16) -> Self;
    ///Remove all points from all leaves
    fn clear(&mut self);
    /// Insert a point into the correct leaf, or return false if it doesn't fit
    fn can_insert(&mut self, x: u16, y: u16) -> bool;
    //TODO remove bool return value
    fn insert(&mut self, x: u16, y: u16, data: Self::DataT) -> bool;
    /// Return a reference to the leaf that contains the point
    fn get_leaf_around(&self, x: u16, y: u16) -> Option<&QuadTreeLeaf<Self::DataT>>;
    //used for debugging
    const DEPTH: usize;
}

/// An array of 4 Quadrants also implements Quadrants
impl<InnerQuadrants> Quadrants for [InnerQuadrants; 4] where InnerQuadrants: Quadrants {
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
    const DEPTH: usize = InnerQuadrants::DEPTH + 1;
}

///QuadTreeLeaf cannot implement Quadrants, because the generic [InnerQuadrants; 4] impl would conflict with the [QuadTreeLeaf<DataT>; 4] impl
// impl<DataT> !Quadrants<DataT> for QuadTreeLeaf<DataT> {}

/// An array of 4 QuadTreeLeafs implements Quadrants
impl<DataT> Quadrants for [QuadTreeLeaf<DataT>; 4] {
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
    const DEPTH: usize = 1;
}

/// A QuadTree leaf with a constructor and a method to insert a point
impl<DataT> QuadTreeLeaf<DataT> {
    fn new(rect_x: u16, rect_y: u16, rect_w: u16, rect_h: u16) -> Self {
        QuadTreeLeaf {vec: Vec::new(), rect_x, rect_y, rect_w, rect_h}
    }
    fn clear(&mut self) {
        self.vec.clear();
    }
    fn can_insert(&self, x: u16, y: u16) -> bool {
        x >= self.rect_x && x <= self.rect_x + self.rect_w && y >= self.rect_y && y <= self.rect_y + self.rect_h
    }
    fn insert(&mut self, x: u16, y: u16, data: DataT) -> bool {
        if self.can_insert(x, y) {
            self.vec.push((x, y, data));
            true
        } else {
            false
        }
    }
}

pub struct LeafIterator<'a, DataT> {
    ptr: &'a mut QuadTree<DataT>,
    index: (usize, usize, usize), //3 levels of depth
}

/// An Iterator that yields leaf in the tree
impl<'a, DataT> Iterator for LeafIterator<'a, DataT> {
    type Item = &'a mut QuadTreeLeaf<DataT>;

    /// Starts at (0,0,0) and ends at (3, 3, 3)
    /// It increases the index by 1 each time, and if it reaches the end of the quad, it moves to the next quad
    fn next(&mut self) -> Option<Self::Item> {
        let (i, j, k) = self.index;
        if i < 4 {
            if j < 4 {
                if k < 4 {
                    self.index.2 += 1;
                    let ptr = &mut self.ptr[i][j][k] as *mut QuadTreeLeaf<DataT>;
                    //https://stackoverflow.com/a/63438431/3052832
                    unsafe {
                        Some(&mut *ptr)
                    }
                } else {
                    self.index.2 = 0;
                    self.index.1 += 1;
                    self.next()
                }
            } else {
                self.index.1 = 0;
                self.index.0 += 1;
                self.next()
            }
        } else {
            None
        }
    }
}

pub fn into_iter<DataT>(quad_tree: &mut QuadTree<DataT>) -> LeafIterator<DataT> {
    LeafIterator {ptr: quad_tree, index: (0, 0, 0)}
}