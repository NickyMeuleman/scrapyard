use std::rc::Rc;
use crate::List::{Cons, Nil};
use crate::List2::{Cons as Cons2, Nil as Nil2};
use crate::List3::{Cons as Cons3, Nil as Nil3};
use std::ops::Deref;

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
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

