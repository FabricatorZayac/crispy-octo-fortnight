use crate::nodes::SingleLinkNode;

pub struct LinkedList<T> {
    head: Option<Box<SingleLinkNode<T>>>,
}

pub trait Stack<T> {
    fn push(&mut self, e: T);
    fn peek(&self) -> Option<T>
        where T: Clone;
    fn peek_ref(&self) -> Option<&T>;
    fn pop(&mut self) -> Option<T>;
}

impl<T> Stack<T> for LinkedList<T> {
    fn push(&mut self, e: T) {
        if self.head.is_none() {
            self.head = Some(Box::new(SingleLinkNode {
                next: None,
                content: e,
            }));
        } else {
            let old_head = self.head.take();
            self.head = Some(Box::new(SingleLinkNode {
                next: old_head,
                content: e,
            }));
        }
    }

    fn peek(&self) -> Option<T>
        where T: Clone {
        self.head.as_ref().map(|head| head.content.clone())
    }

    fn peek_ref(&self) -> Option<&T> {
        self.head.as_ref().map(|head| &head.content)
    }

    fn pop(&mut self) -> Option<T> {
        let head = std::mem::take(&mut self.head);
        match head {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.content)
            }
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList::<T> { head: None }
    }
}
