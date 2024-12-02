use core::{
    ptr::null_mut,
    sync::atomic::{AtomicPtr, Ordering},
};

pub struct Collection<E> {
    head: AtomicPtr<CollectionNode<E>>,
}

impl<E> Collection<E> {
    pub const fn new() -> Self {
        Self {
            head: AtomicPtr::new(null_mut()),
        }
    }

    pub unsafe fn iter(&self) -> impl Iterator<Item = &'static E>
    where
        E: 'static,
    {
        CollectionIter {
            ptr: self.head.load(Ordering::Relaxed),
        }
    }

    pub fn push(&self, node: &mut CollectionNode<E>) {
        let next = self.head.swap(
            node as *const CollectionNode<E> as *mut CollectionNode<E>,
            Ordering::Relaxed,
        );
        node.next = next;
    }
}

pub struct CollectionNode<E> {
    elem: E,
    next: *const CollectionNode<E>,
}

impl<E> CollectionNode<E> {
    pub const fn new(elem: E) -> Self {
        Self {
            elem,
            next: null_mut(),
        }
    }
}

pub struct CollectionIter<E> {
    ptr: *const CollectionNode<E>,
}

impl<E> Iterator for CollectionIter<E>
where
    E: 'static,
{
    type Item = &'static E;
    fn next(&mut self) -> Option<Self::Item> {
        //SAFETY: it is assumed that Collection will not be modified for the rest of the program.
        let Some(CollectionNode { elem, next }) = (unsafe { self.ptr.as_ref() }) else {
            return None;
        };
        self.ptr = *next;
        Some(elem)
    }
}
