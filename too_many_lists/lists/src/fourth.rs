// Disclaimer: this chapter is basically a demonstration that this is a very bad idea.
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    prev: Link<T>,
    next: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T>(Option<Ref<'a, Node<T>>>);

impl<T> Node<T> {
    // this seems weird to me, why not do this as Link::new instead to remove the wrapping of Self?
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            elem: elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    // An easy way for us to validate if our methods make sense
    // is if we maintain the following invariant:
    // each node should have exactly two pointers to it.
    // Each node in the middle of the list is pointed at by its predecessor and successor,
    // while the nodes on the ends are pointed to by the list itself
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        // new node needs +2 links, everything else should be +0
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone()); // +1 new_head
                new_head.borrow_mut().next = Some(old_head); // +1 old_head
                self.head = Some(new_head); // -1 old_head, +1 new_head
            }
            None => {
                // empty list, the new node is the head and tail of the list
                self.tail = Some(new_head.clone()); // +1 new_head
                self.head = Some(new_head); // +1 new_head
            }
        }
    }

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                // Rc::clone() is same as calling .clone()
                old_tail.borrow_mut().next = Some(Rc::clone(&new_tail));
                new_tail.borrow_mut().prev = Some(Rc::clone(&old_tail));
                self.tail = Some(new_tail);
            }
            None => {
                // empty list, new node is head and tail
                self.head = Some(Rc::clone(&new_tail));
                self.tail = Some(new_tail);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        // the head (called old_head) needs to go -2 before this method completes
        self.head.take().map(|old_head| {
            // -1 old_head
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    // -1 new_head
                    new_head.borrow_mut().prev.take(); // -1 old_head
                    self.head = Some(new_head); // +1 new_head
                                                // total: -2 old, +0 new
                }
                None => {
                    // empty the list
                    self.tail.take(); // -1 old_head
                }
            }
            // the Result<T> can't be unwrap()ped because Node<T> doesn't
            // implement Debug, we work around it by converting the Result, to an Option with ok()
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        // the tail at the start of this function needs to go -2 in strong_reference count
        self.tail.take().map(|old_tail| {
            // -1 old_tail
            match old_tail.borrow_mut().prev.take() {
                // -1 new_tail
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take(); // -1 old_tail
                    self.tail = Some(new_tail); // +1 new_tail
                }
                None => {
                    // the list just became empty, clean up the strong_reference it has
                    self.head.take(); // -1 old_tail
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        // the Ref from borrow() needs to stay alive for the reference to be valid
        // so we return it, otherwise it would be dropped when this function ends
        self.head.as_ref().map(|node| {
            // map() the Ref<Node<T>> into a Ref<T>
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter(self.head.as_ref().map(|head| head.borrow()))
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // pop till None, deallocate all the things!
        while self.pop_front().is_some() {}
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    // there is no type Item = T; here? I'd like to use Self::Item in the return-type
    // apparently, that works because DoubleEndedIterator inherits from Iterator so the type Item is already defined.
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

// impl<'a, T> Iterator for Iter<'a, T> {
//     // I AM NOT SMART ENOUGH FOR THIS, my head hurts
//     type Item = Ref<'a, T>;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.take().map(|node_ref| {
//             let (next, elem) = Ref::map_split(node_ref, |node| {
//                 (&node.next, &node.elem)
//             });
//             // the deeper we traverse the list, the more RefCells wrap the thing we want, so the next line throws a big fat error
//             self.0 = if next.is_some(){Some(Ref::map(next, |next| &**next.as_ref().unwrap()))}else {None};
//             elem
//         })
//     }
// }

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        // Check empty list behaves right
        assert_eq!(list.pop_back(), None);

        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        // Refs don't implement comparisons, womp, womp
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(&*list.peek_front().unwrap(), &3);
        // this also works
        // assert_eq!(*list.peek_front().unwrap(), 3);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }
}
