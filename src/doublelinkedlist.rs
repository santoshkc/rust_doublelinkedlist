//mod doublelinkedlist

use std::cell::*;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    element: T,
    previous: Link<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    fn create(element: T) -> Rc<RefCell<Node<T>>> {
        let node = Self {
            element: element,
            previous: None,
            next: None,
        };
        Rc::new(RefCell::new(node))
    }
}

struct DoubleLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> DoubleLinkedList<T> {
    fn new() -> Self {
        DoubleLinkedList {
            head: None,
            tail: None,
        }
    }

    fn insert_at_front(&mut self, element: T) {
        let new_node = Node::create(element);

        let previous_head = self.head.take();

        match previous_head {
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            Some(value) => {
                value.borrow_mut().previous = Some(new_node.clone());
                new_node.borrow_mut().next = Some(value);

                self.head = Some(new_node);
            }
        }
    }

    fn remove_at_front(&mut self) -> Option<T> {
        let element = self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().previous.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }

            // make sure we have rc strong as 1 before try_unwrap
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().element
        });

        element
    }

    fn peek_at_front(&self) -> Option<Ref<T>> {
        let front = self.head.as_ref().map(|n| {
            let b = n.borrow();
            let r = Ref::map(b, |t| &t.element);
            r
        });
        front
    }

    fn insert_at_end(&mut self, element: T) {
        let new_node = Node::create(element);

        let previous_tail = self.tail.take();

        match previous_tail {
            None => {
                self.tail.replace(new_node.clone());
                self.head.replace(new_node);
            }
            Some(value) => {
                value.borrow_mut().next.replace(new_node.clone());
                new_node.borrow_mut().previous.replace(value.clone());

                self.tail = Some(new_node.clone());
            }
        }
    }

    fn remove_at_end(&mut self) -> Option<T> {

        self.tail.take().map(|old_tail| {

            match old_tail.borrow_mut().previous.take() {

                Some(new_tail) => {

                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                },
                None => {
                    self.head.take();
                }
            }
            // make sure we have strong rc count = 1 before unwrapping here
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().element
        })
    }

    fn peek_at_end(&self) -> Option<Ref<T>> {
        let end = self.tail.as_ref().map(|n| {
            let b = n.borrow();
            Ref::map(b, |t| &t.element)
        });
        end
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_list_empty() {
        let list = DoubleLinkedList::<i32>::new();

        assert!(list.head.is_none());
        assert!(list.tail.is_none());
    }

    #[test]
    fn insert_at_front_test() {
        let element = 10;
        let another_element = 100;
        let mut list = DoubleLinkedList::<i32>::new();
        list.insert_at_front(element);
        list.insert_at_front(another_element);

        let front = list.peek_at_front();

        let x = *front.unwrap();

        assert_eq!(another_element, x);
    }

    #[test]
    fn insert_at_back_test() {
        let element = 10;
        let another_element = 100;
        let mut list = DoubleLinkedList::new();
        list.insert_at_end(element);
        list.insert_at_end(another_element);

        let end = list.peek_at_end();

        let x = *end.unwrap();

        assert_eq!(another_element, x);
    }

    #[test]
    fn remove_at_front_test() {
        let mut list = DoubleLinkedList::<i32>::new();

        let front = list.remove_at_front();
        assert!(front.is_none());

        let element = 10;
        list.insert_at_front(element);

        let another_element = 20;
        list.insert_at_end(another_element);

        let front = list.remove_at_front();
        let item = front.unwrap();

        assert_eq!(element, item);

        let front = list.remove_at_front();
        let item = front.unwrap();

        assert_eq!(another_element, item);
    }


    #[test]
    fn remove_at_end_test() {
        let mut list = DoubleLinkedList::<i32>::new();

        let end = list.remove_at_end();
        assert!(end.is_none());

        let element = 10;
        list.insert_at_front(element);

        let another_element = 20;
        list.insert_at_end(another_element);

        let end = list.remove_at_end();

        let item = end.unwrap();

        assert_eq!(another_element, item);

        let end = list.remove_at_front();
        let item = end.unwrap();

        assert_eq!(element, item);

    }
}
