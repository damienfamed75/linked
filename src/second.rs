struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct List<T> {
    head: Link<T>,
}

// Link is now an Option instead of its own enum.
type Link<T> = Option<Box<Node<T>>>;

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            // Take the value of head, and replace it with a None value.
            next: self.head.take(),
        });

        // Set the new value into head.
        self.head = Some(new_node);
    }

    // Option can either return Some or None
    // Essentially saying, it CAN return a value but may not at times.
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        // map takes by value, so we first take by reference and then map.
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        // map takes by value, so we first take by reference and then map.
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // Take the value of head, and replace it with a None value.
        let mut cur_link = self.head.take();
        // let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
            // boxed_node goes out of scope and gets dropped here.
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

// cfg(test) prevents the compiler from warning when use super::List;
#[cfg(test)]
mod test {
    use super::List;

    // Mark this function as a testing function.
    #[test]
    fn basic() {
        {
            // Given an empty list
            let mut l: List<i32> = List::new();
            {
                // When popping
                assert_eq!(l.pop(), None);
            }
        }

        {
            // Given a populate list.
            let mut l = List::new();
            // Populate the list.
            l.push(1);
            l.push(2);
            l.push(3);
            {
                // When popping
                assert_eq!(l.pop(), Some(3));
                assert_eq!(l.pop(), Some(2));
                assert_eq!(l.pop(), Some(1));
                // Exhausted list pop.
                assert_eq!(l.pop(), None);
            }
        }

        {
            let mut l = List::new();
            l.push(1);

            {
                assert_eq!(l.peek(), Some(&1));
                assert_eq!(l.pop(), Some(1));
            }
        }
    }
}
