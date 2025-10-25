
use std::cell::{RefCell, Ref};
use std::rc::{Rc, Weak};

type RcRefCell<T> = Rc<RefCell<T>>; 
type WeakRefCell<T> = Weak<RefCell<T>>; 

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    parent: RefCell<Weak<Node<T>>>,
    children: RefCell<Vec<Rc<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Rc<Node<T>> {
        Rc::new(Node {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }

    pub fn add_child(parent: &Rc<Node<T>>, child: Rc<Node<T>>) {
        *child.parent.borrow_mut() = Rc::downgrade(parent);
        parent.children.borrow_mut().push(child);
    }

    pub fn children(&'_ self) -> Ref<'_, Vec<Rc<Node<T>>>> {
        self.children.borrow()
    }

    pub fn parent(&self) -> Option<Rc<Node<T>>> {
        self.parent.borrow().upgrade()
    }
}

impl<T> std::ops::Deref for Node<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> std::ops::DerefMut for Node<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
