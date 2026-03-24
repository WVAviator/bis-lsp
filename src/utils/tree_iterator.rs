use tree_sitter::{Node, TreeCursor};

pub struct TreeIterator<'a> {
    cursor: TreeCursor<'a>,
    done: bool,
}

impl<'a> TreeIterator<'a> {
    pub fn new(cursor: TreeCursor<'a>) -> Self {
        TreeIterator {
            cursor,
            done: false,
        }
    }
}

impl<'a> Iterator for TreeIterator<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let node = self.cursor.node();

        if self.cursor.goto_first_child() {
            return Some(node);
        }

        if self.cursor.goto_next_sibling() {
            return Some(node);
        }

        loop {
            if !self.cursor.goto_parent() {
                self.done = true;
                return Some(node);
            }
            if self.cursor.goto_next_sibling() {
                return Some(node);
            }
        }
    }
}
