// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

use std::rc::Rc;

// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

use std::cell::RefCell;
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // -----------------------------------

    // let a = Rc::new( Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // println!("count after creating a = {}", Rc::strong_count(&a));

    // let b = Cons(3, Rc::clone(&a));
    // println!("count after creating b = {}", Rc::strong_count(&a));

    // {
    //     let c = Cons(4, Rc::clone(&a));
    //     println!("count after creating c = {}", Rc::strong_count(&a));
    // }

    // println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // -----------------------------------

    let value = Rc::new(RefCell::new(5));
    println!("value = {:?}", value);

    println!();
    println!();

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    println!("a = {:?}", a);

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    println!("b = {:?}", b);

    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    println!("c = {:?}", c);

    println!();
    println!();

    *value.borrow_mut() += 10;
    println!("change value after = {:?}", value);

    println!();
    println!();

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
    
}
