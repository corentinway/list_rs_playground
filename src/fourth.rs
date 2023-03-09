use std::{
    borrow::Borrow,
    cell::{Ref, RefCell, RefMut},
    fmt::Debug,
    rc::Rc,
};

pub struct List<T>
where
    T: Debug,
{
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T: Debug> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}
impl<T> Node<T>
where
    T: Debug,
{
    pub fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            next: None,
            prev: None,
        }))
    }
}

impl<T> Default for List<T>
where
    T: Debug,
{
    fn default() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
}
impl<T> List<T>
where
    T: Debug,
{
    pub fn new() -> Self {
        List::default()
    }
    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                // Doing
                // self.head           new_head              old_head
                //          next-->           next-->
                //            None <--prev            <--prev
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }
    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }

            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            let elem = Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem;
            println!("pop_back elem :{:?}", &elem);
            elem
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        let head_ref: Option<&Rc<RefCell<Node<T>>>> = self.head.as_ref();
        let borrowed_node: Option<&RefCell<Node<T>>> = head_ref.map(|node| node.borrow());

        let borrowed_node: Option<Ref<Node<T>>> = borrowed_node.map(|node| node.borrow());

        let borrowed_node: Option<Ref<T>> =
            borrowed_node.map(|node| Ref::map(node, |node| &node.elem));

        borrowed_node
    }
    pub fn peek_back(&self) -> Option<Ref<T>> {
        let tail_ref: Option<&Rc<RefCell<Node<T>>>> = self.tail.as_ref();
        let borrowed_node: Option<&RefCell<Node<T>>> = tail_ref.map(|node| node.borrow());
        let borrowed_node: Option<Ref<Node<T>>> = borrowed_node.map(|node| node.borrow());
        let borrowed_node: Option<Ref<T>> =
            borrowed_node.map(|node| Ref::map(node, |node| &node.elem));
        borrowed_node
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        let head_ref: Option<&Rc<RefCell<Node<T>>>> = self.head.as_ref();
        let borrowed_node: Option<RefMut<Node<T>>> = head_ref.map(|node| node.borrow_mut());

        let borrowed_node: Option<RefMut<T>> =
            borrowed_node.map(|node| RefMut::map(node, |node| &mut node.elem));

        borrowed_node
    }
    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        let tail_ref: Option<&Rc<RefCell<Node<T>>>> = self.tail.as_ref();
        let borrowed_node: Option<RefMut<Node<T>>> = tail_ref.map(|node| node.borrow_mut());

        let borrowed_node: Option<RefMut<T>> =
            borrowed_node.map(|node| RefMut::map(node, |node| &mut node.elem));

        borrowed_node
    }
}

impl<T> Drop for List<T>
where
    T: Debug,
{
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

pub struct IntoIter<T>(List<T>)
where
    T: Debug;

impl<T> List<T>
where
    T: Debug,
{
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T>
where
    T: Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T>
where
    T: Debug,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

pub struct Iter<'a, T>(Option<Ref<'a, Node<T>>>)
where
    T: Debug;

impl<T> List<T>
where
    T: Debug,
{
    pub fn iter(&self) -> Iter<T> {
        let head_ref: Option<&Rc<RefCell<Node<T>>>> = self.head.as_ref();
        let borrowed_node: Option<&RefCell<Node<T>>> = head_ref.map(|node| node.borrow());

        let borrowed_node: Option<Ref<Node<T>>> = borrowed_node.map(|node| node.borrow());
        Iter(borrowed_node)
    }
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Debug,
{
    type Item = Ref<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let parts: Option<(Ref<Option<Rc<RefCell<Node<T>>>>>, Ref<T>)> =
            self.0.take().map(|node_ref| {
                Ref::map_split(node_ref, |node| (&node.next, &node.elem))
                // // type Link<T> = Option<Rc<RefCell<Node<T>>>>;
                // let borrowed_node: &Link<T> = &*next;
                // let borrowed_node: Option<&RefCell<Node<T>>> =
                //     borrowed_node.as_ref().map(|node| node.borrow());

                // let borrowed_node: Option<Ref<Node<T>>> = borrowed_node.map(|node| node.borrow());

                // // let borrowed_node: Option<Ref<T>> =
                // //     borrowed_node.map(|node| Ref::map(node, |node| &node.elem));

                // // self.0 : Option<Ref<'a, Node<T>>>
                // self.0 = borrowed_node;

                // elem
            });

        if let Some((next, elem)) = parts {
            let borrowed_node: &Link<T> = &*next;
            let borrowed_node: Option<&RefCell<Node<T>>> =
                borrowed_node.as_ref().map(|node| node.borrow());
            let borrowed_node: Option<Ref<Node<T>>> = borrowed_node.map(|node| node.borrow());
            self.0 = borrowed_node;

            return Some(elem);
        }

        None
    }
}

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

        assert_eq!(list.pop_back(), None);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        list.push_back(4);
        list.push_back(5);

        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(*list.peek_front().unwrap(), 3);
        assert_eq!(&*list.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(*list.peek_back().unwrap(), 1);
        assert_eq!(&*list.peek_back_mut().unwrap(), &mut 1);
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
    }
}
