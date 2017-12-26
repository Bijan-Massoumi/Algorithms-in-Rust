#[cfg(test)]
mod linked_list {
    struct Node<T> {
        data: T,
        next: Option<Box<Node<T>>>,
        prev: Option<Box<Node<T>>>,
    }

    impl<T> Node<T> {
        fn new(data: T) -> Node<T> {
            Node {
                data,
                next: None,
                prev: None,
            }
        }
    }


    mod stack {
        use linked_list::Node;
        use std;

        struct Stack<T: std::fmt::Debug> {
            head: Option<Box<Node<T>>>,
        }


        impl<T: std::fmt::Debug> Stack<T> {
            fn new() -> Stack<T> {
                Stack {
                    head: None,
                }
            }

           /* fn print_nodes(head: &Option<Box<Node<T>>>) {
                match *head {
                    Some(ref p) => {
                        println!("{:?}",p.data);
                        Stack::print_nodes(head.next);
                    },
                }
            }*/

            fn push(&mut self, val: T) {
                self.head = Some(Box::new(Node {
                    data: val,
                    next: self.head.take(),
                    prev: None,
                }));
            }

            fn pop (&mut self) -> Option<T> {
                match self.head.take() {
                    None => None,
                    Some(mut head) => {
                        self.head = head.next.take();
                        Some(head.data)
                    }
                }
            }
            /*fn print_stack(&self) {
                Stack::print_nodes(&self.head);
            }*/

        }
        #[test]
        fn test_stack() {
            let mut test = Stack::<f32>::new();
            test.push(3.0);
            let love = test.pop();
            assert_eq!(love, Some(3.));
        }
    }
}
