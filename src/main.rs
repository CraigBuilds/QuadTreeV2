mod quad_tree;
use quad_tree::*;
mod entity;
use entity::*;

fn main() {
    
    let entities = vec![
        Entity{x: 0, y: 0, width: 2, height: 2},
        Entity{x: 1, y: 1, width: 2, height: 2},
    ];

    let mut tree = QuadTree::new(0,0,128,128); //128x128 world, 8x8 grid, so every leaf is 16x16

    loop {
        
        //rebuild QuadTree every frame (only allocs when needed)
        tree.clear();
        for e in entities.iter() {
            tree.insert(e.x, e.y, e);
        }

        //check for collisions
        for e in entities.iter() {
            //instead of checking e against all other elements, we can check it against the elements in the same quadrant
            let candidates = tree.get_leaf_around(e.x, e.y);
            let candidates = if candidates.is_some() {candidates.unwrap()} else { continue };
            for candidate in candidates.vec.iter() {
                //skip self check
                if candidate.0 as *const u16 == e.x as *const u16 { continue }
                //do collision check
                println!("potential collision between {:?} and {:?}", e, candidate);
            }
        }

        println!("");
    }
}