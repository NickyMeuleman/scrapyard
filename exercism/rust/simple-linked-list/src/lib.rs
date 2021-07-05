use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        // traverse list, incrementing len every time there is a node in `next`
        // call `as_ref` to avoid taking ownership of the node, this leaves the `curr_node` intact when it changes
        // the borrow of `curr_node` is dropped, not the node itself
        let mut curr_node = self.head.as_ref();
        let mut count = 0;
        while let Some(node) = curr_node {
            count += 1;
            curr_node = node.next.as_ref();
        }
        count
    }

    pub fn push(&mut self, element: T) {
        // adds new node to the beginning of the list
        let new_node = Node {
            data: element,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        // removes head node `Option`, if it's a `Some` do things, otherwise return that `None`
        // If `Some`: sets `head` to `next` of the old head node, returns `data` of the old head node
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        // borrows `head` and returns borrow of that nodes `data`
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        // create a new list, push to it in the order `pop` on the old list returns data
        let mut list = Self::new();
        while let Some(elem) = self.pop() {
            list.push(elem);
        }
        list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        // create a new list, iterate over the given `IntoIterator` and push every item to the list
        iter.into_iter()
            .fold(Self::new(), |mut acc, item| {
                acc.push(item);
                acc
            })
    }
    // fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
    //     let mut list = Self::new();
    //     for item in iter {
    //         list.push(item)
    //     }
    //     list
    // }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        // create a new vector and push items to it in the order they are popped off the LinkedList
        // reverse the final vector, because this LinkedList pushes to the front and the test assumes it pushes to the back?
        let mut vec = Vec::new();
        while let Some(data) = self.pop() {
            vec.push(data);
        }
        vec.into_iter().rev().collect()
    }
    // fn into(self) -> Vec<T> {
    //     let mut vec: Vec<T> = Vec::new();
    //     let mut curr_node = self.head;
    //     while let Some(node) = curr_node {
    //         vec.push(node.data);
    //         curr_node = node.next;
    //     }
    //     vec.reverse();
    //     vec
    // }
}
