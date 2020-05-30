use std::mem;

struct Node {
    elem: i32,
    next: Link,
}

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            // this line, replaces the memory stored at self.head
            // and places an empty value for its place.
            // And then outputs the value it had to be placed into next.
            next: mem::replace(&mut self.head, Link::Empty),
        });

        // Set the new value into head.
        self.head = Link::More(new_node);
    }

    // Option can either return Some or None
    // Essentially saying, it CAN return a value but may not at times.
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here.
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

enum Link {
    Empty,
    More(Box<Node>),
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
            let mut l = List::new();
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
    }
}
