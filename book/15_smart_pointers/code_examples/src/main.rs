use crate::List::{Cons, Nil};
use crate::List2::{Cons as Cons2, Nil as Nil2};
use crate::List3::{Cons as Cons3, Nil as Nil3};
use crate::List4::{Cons as Cons4, Nil as Nil4};
use crate::List5::{Cons as Cons5, Nil as Nil5};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
enum List2<'a> {
    Cons(i32, &'a List2<'a>),
    Nil,
}

#[derive(Debug)]
enum List3 {
    Cons(i32, Rc<List3>),
    Nil,
}

#[derive(Debug)]
enum List4 {
    Cons(Rc<RefCell<i32>>, Rc<List4>),
    Nil,
}

#[derive(Debug)]
enum List5 {
    Cons(i32, RefCell<Rc<List5>>),
    Nil,
}

impl List5 {
    fn tail(&self) -> Option<&RefCell<Rc<List5>>> {
        match self {
            Cons5(_, item) => Some(item),
            Nil5 => None,
        }
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

#[derive(Debug)]
struct LLNode {
    data: i32,
    next: Option<Box<LLNode>>,
}
impl LLNode {
    fn new(num: i32) -> Self {
        LLNode {
            data: num,
            next: None,
        }
    }
    fn set_next(&mut self, next: LLNode) {
        self.next = Some(Box::new(next));
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list = {:?}", list);

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Nicky"));
    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    let a = Cons2(5, &Cons2(10, &Nil2));
    let b = Cons2(3, &a);
    let c = Cons2(4, &a);

    let a = Rc::new(Cons3(5, Rc::new(Cons3(10, Rc::new(Nil3)))));
    let b = Cons3(3, Rc::clone(&a));
    let c = Cons3(4, Rc::clone(&a));

    let a = Rc::new(Cons3(5, Rc::new(Cons3(10, Rc::new(Nil3)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons3(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons3(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons4(Rc::clone(&value), Rc::new(Nil4)));

    let b = Cons4(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons4(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    println!("==== REFERENCE CYCLES ====");
    let a = Rc::new(Cons5(5, RefCell::new(Rc::new(Nil5))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons5(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    println!("---- graph ----");
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    println!("==== LINKED LIST ====");
    let mut one = LLNode::new(1);
    let two = LLNode::new(2);

    one.set_next(two);

    println!("{:?}", one);
    println!("{}", one.data);
    println!("{:?}", one.next);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
