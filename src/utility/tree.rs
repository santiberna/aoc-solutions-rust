use std::{marker::PhantomData, ptr::NonNull};

pub struct Node<T> {
    pub value: T,
    pub children: Vec<Box<Node<T>>>,
}

pub struct Zipper<'a, T> {
    current: NonNull<Node<T>>,
    parent: Option<Box<Zipper<'a, T>>>,
    marker: PhantomData<&'a mut Node<T>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            children: vec![],
        }
    }

    fn add_child(&mut self, child: Self) {
        self.children.push(Box::new(child));
    }

    fn remove_child(&mut self, index: usize) -> Self {
        *self.children.swap_remove(index)
    }
}

impl<'a, T> Zipper<'a, T> {
    fn from_root(root: &'a mut Node<T>) -> Self {
        Self {
            current: NonNull::from(root),
            parent: None,
            marker: PhantomData,
        }
    }

    fn get_mut(&mut self) -> &'a mut Node<T> {
        unsafe { self.current.as_mut() }
    }

    fn get(&self) -> &'a Node<T>
    {
        
    }

    fn up(self) -> Option<Self> {
        self.parent.map(|b| *b)
    }

    fn down(mut self, index: usize) -> Option<Self> {
        let current = self.get_mut();

        current.children.get_mut(index).map(|child| Self {
            current: NonNull::from(child.as_mut()),
            parent: Some(Box::new(self)),
            marker: PhantomData,
        })
    }

    fn root(self) -> Self {
        match self.parent {
            Some(p) => p.root(),
            None => self,
        }
    }
}
