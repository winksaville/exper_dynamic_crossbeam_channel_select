use std::{cell::RefCell, rc::Rc};

/// SimpleRefs is a set of thin pointer saved as
/// *const u8 to things. It uses internal mutability
/// and it is not necessary to have it defined as `mut`.
/// To make this safe you may only use `push` to store
/// items and `get` to retrieve them. They may not be
/// modified or removed.
///
/// TODO: Require the T to be a PIN so that the reference
/// stored in SimpleRefs, pointee, cannot move. It is also deseriable
/// that the pointee always be "valid" even if it's "removed".
/// One possibility is that the pointee could be an Option<T>.
struct SimpleRefs(Rc<RefCell<Vec<*const u8>>>);

impl SimpleRefs {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(Vec::new())))
    }

    pub fn get<T>(&self, idx: usize) -> &T {
        let ptr = (*self.0).borrow()[idx];

        unsafe { &*ptr.cast::<T>() }
    }

    pub fn push<T>(&self, v: &T) {
        let ptr = v as *const T as *const u8;
        (*self.0).borrow_mut().push(ptr);
    }
}

fn main() {
    let things_i32: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(Vec::new()));
    let refs_i32 = SimpleRefs::new();

    for i in 0..=1 {
        // Push an integer into things_i32
        things_i32.borrow_mut().push(i);

        // Calculate the index of and address of the i32 in things_i32
        let idx = (*things_i32).borrow().len() - 1;

        // Determine the address of Calculate the address
        let things_i32_addr = &((*things_i32).borrow()[idx]);

        // Print index and address
        println!("{idx}: things_i32_addr={things_i32_addr:p}");
        refs_i32.push(things_i32_addr);
    }

    for idx in 0..=1 {
        let things_i32_addr = &((*things_i32).borrow()[idx]);
        let ref_i32 = refs_i32.get::<i32>(idx);
        assert_eq!(things_i32_addr, ref_i32);
        println!("{idx}: things_i32_addr={things_i32_addr:p} ref_i32={ref_i32:p}");
    }
}
