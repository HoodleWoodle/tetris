use crate::engine::vec::Vec2u;
use std::{
    u32,
    rc::Rc,
    cell::RefCell,
};

struct Node {
    origin: Vec2u,              // upper left of the rectangle this node represents
    size: Vec2u,                // size of the rectangle this node represents
    empty: bool,               // false if this node is a leaf and is filled

    first: Option<Rc<RefCell<Node>>>,    // left (or top) subdivision
    second: Option<Rc<RefCell<Node>>>,   // right (or bottom) subdivision
}

impl Node {
    fn new(origin: Vec2u, size: Vec2u) -> Node {
        Node {
            origin,
            size,
            empty: true,

            first: None,
            second: None,
        }
    }

    //# fn print(&self, name: &str, level: usize) {
    //#     //if !self.empty {
    //#     //    return;
    //#     //}
    //#     if level != 0 {
    //#         for _ in 0..(level - 1) {
    //#             print!("      ");
    //#         }
    //#     }
    //#     if name == "root" {
    //#         println!("{} [empty: {}, origin: {:?}, size: {:?}]", name, self.empty, self.origin, self.size);
    //#     } else {
    //#         println!("|---- {} [empty: {}, origin: {:?}, size: {:?}]", name, self.empty, self.origin, self.size);
    //#     }
    //#     
    //#     if let Some(first) = self.first.as_ref() {
    //#         first.borrow().print("first", level + 1);
    //#     }
    //#     if let Some(second) = self.second.as_ref() {
    //#         second.borrow().print("second", level + 1);
    //#     }
    //# }
}

// TODO: this texture packer is fuckin bad ... just replace it with something else
pub struct TexturePacker {
    size: Vec2u,
    buffer: Vec<u8>,
    root: Rc<RefCell<Node>>,
}

impl TexturePacker {
    pub fn new() -> TexturePacker {
        let size = Vec2u::new(512, 512);
        
        let capacity = (size.x * size.y) as usize;
        let mut buffer = Vec::with_capacity(capacity);
        buffer.resize(capacity, 0);

        TexturePacker {
            size,
            buffer,
            root: Rc::new(RefCell::new(Node::new(Vec2u::new(0, 0), size)))
        }
    }

    pub fn size(&self) -> Vec2u {
        self.size
    }

    pub fn data(self) -> Vec<u8> {
        self.buffer
    }

	fn resize(&mut self, new_size: Vec2u) {
        let capacity = (new_size.x * new_size.y) as usize;
        let mut new_buffer = Vec::with_capacity(capacity);
        new_buffer.resize(capacity, 0);
        
		for y in 0..(self.size.y as usize) {
            for x in 0..(self.size.x as usize) {
                new_buffer[x + y * (new_size.x as usize)] = self.buffer[x + y * (self.size.x as usize)];
            }
        }

        self.size = new_size;
		self.buffer = new_buffer;
	}

    pub fn pack(&mut self, buffer: &[u8], buffer_size: Vec2u) -> Vec2u {
        //# self.root.borrow().print("root", 0);
        //# println!("_______________________________________________");
        //# println!("{:?}", buffer_size);
        //# println!("-----------------------------------------------");

        let node = self.pack_internal(Rc::clone(&self.root), buffer_size);
        //# println!("packed");
        let node = node.or_else(|| {
            assert!(false); // TODO: not implemented

            self.resize(Vec2u::new(self.size.x * 2, self.size.y * 2));
            self.pack_internal(Rc::clone(&self.root), buffer_size)
        })
        // Note: this assertion will be hit when trying to pack a texture larger than the current size of the texture
        .unwrap();
        
        assert!(buffer_size.x == node.borrow().size.x);
        assert!(buffer_size.y == node.borrow().size.y);
        
        // copy the texture to the texture atlas' buffer
        for ly in 0..buffer_size.y {
            for lx in 0..buffer_size.x {
                let x = node.borrow().origin.x + lx;
                let y = node.borrow().origin.y + ly;
                self.buffer[(x + y * self.size.x) as usize] = buffer[(lx + buffer_size.x * ly) as usize];
            }
        }
        
		return node.borrow().origin;
    }

    fn pack_internal(&mut self, node: Rc<RefCell<Node>>, buffer_size: Vec2u) -> Option<Rc<RefCell<Node>>> {
        if !node.borrow().empty {
            //# println!("filled");
			// the node is filled, not gonna fit anything else here
            assert!(node.borrow().first.is_none() && node.borrow().second.is_none());
			return None;
        } else if node.borrow().first.is_some() && node.borrow().second.is_some() {
            //# println!("non-leaf");
			// non-leaf, try inserting to the left and then to the right
            let new = self.pack_internal(Rc::clone(node.borrow().first.as_ref().unwrap()), buffer_size);
            new.or_else(|| self.pack_internal(Rc::clone(node.borrow().second.as_ref().unwrap()), buffer_size))
        } else {
			// this is an unfilled leaf - try to fill it
			let mut real_size = node.borrow().size;

			// if along a boundary, calculate the actual size
			if node.borrow().origin.x + node.borrow().size.x == u32::MAX { // TODO: MAX ?
                real_size.x = self.size.x - node.borrow().origin.x;
            }
			if node.borrow().origin.y + node.borrow().size.y == u32::MAX {
                real_size.y = self.size.y - node.borrow().origin.y; // TODO: MAX ?
            }

			if node.borrow().size.x == buffer_size.x && node.borrow().size.y == buffer_size.y {
                //# println!("unfilled leaf - fill");
				// perfect size - just pack into this node
				node.borrow_mut().empty = false;
				return Some(Rc::clone(&node));
			} else if real_size.x < buffer_size.x || real_size.y < buffer_size.y {
                //# println!("unfilled leaf - too small");
				// not big enough
				return None;
			} else {
                //# println!("unfilled leaf - split");
                // large enough - split until a perfect fit is available
                let first;
                let second;

				// determine how much space is left if we split each way
				let x_remain = real_size.x - buffer_size.x;
				let y_remain = real_size.y - buffer_size.y;

				// split the way that will leave the most room
				let mut vertical_split = x_remain < y_remain;
				if x_remain == 0 && y_remain == 0 {
                    // edge case - we are are going to hit the border of the texture atlas perfectly, split at the border instead
                    vertical_split = node.borrow().size.x <= node.borrow().size.y;
                }
                
                if vertical_split {
                    // split vertically (first is top)
                    let origin = node.borrow().origin;
                    let size = Vec2u::new(node.borrow().size.x, buffer_size.y);
                    first = Node::new(origin, size);

                    let origin = Vec2u::new(node.borrow().origin.x, node.borrow().origin.y + buffer_size.y);
                    let size = Vec2u::new(node.borrow().size.x, node.borrow().size.y - buffer_size.y);
					second = Node::new(origin, size);
                } else {
                    // split horizontally (first is left)
                    let origin = node.borrow().origin;
                    let size = Vec2u::new(buffer_size.x, node.borrow().size.y);
                    first = Node::new(origin, size);
                    
                    let origin = Vec2u::new(node.borrow().origin.x + buffer_size.x, node.borrow().origin.y);
                    let size = Vec2u::new(node.borrow().size.x - buffer_size.x, node.borrow().size.y);
					second = Node::new(origin, size);
                }
                
                node.borrow_mut().first = Some(Rc::new(RefCell::new(first)));
                node.borrow_mut().second = Some(Rc::new(RefCell::new(second)));
                return self.pack_internal(Rc::clone(node.borrow().first.as_ref().unwrap()), buffer_size);
            }
        }
    }
}