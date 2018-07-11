//! # Summary
//!
//! This crate provides a simple, non-intrusive tree cursor that supports node
//! mutation without [`Cell`]/[`RefCell`].
//!
//! # Soundness
//!
//! **In a nutshell: I don't know if this crate is sound. I *think* it is, but
//! you should still check it yourself before using it.**
//!
//! The current implementation uses unsafe code to work around the borrow
//! checker's strictness about lifetimes. So far, I haven't formally proven
//! that it's memory-safe, especially in the presence of malicious code. I've
//! done my best to write adversarial tests, and I'll continue poking at corner
//! cases until I'm personally satisfied that it's sound, but it's not likely
//! I'll get around to formally proving it until I better understand the borrow
//! checker.
//!
//! # Concepts
//!
//! For the purposes of this crate...
//! - a tree has exactly one root node (no tree is empty)
//! - each node has zero or more children
//! - "up" is toward the root
//! - "down" is away from the root
//! - at any point in time, a cursor has a *position*, and the node at that
//!   position is considered *active*
//!
//! # Guided tour
//!
//! Let's look at a simple tree and how you might use a cursor to traverse it.
//!
//! ```
//! use tree_cursor::cursor::TreeCursor;
//! use tree_cursor::prelude::*;
//!
//! struct Node(&'static str, Vec<Node>);
//!
//! // This trait impl is used by TreeCursor::down to determine the next child
//! // to visit. You can create a TreeCursor for something that doesn't
//! // implement Down; you just won't be able to call TreeCursor::down.
//! impl Down for Node {
//!     fn down(&self, idx: usize) -> Option<&Self> {
//!         // idx starts at 0 when we visit this node going downward and
//!         // increments every time we visit it going upward. This allows us
//!         // to visit each child in order by going back up to this node after
//!         // each one.
//!         self.1.get(idx)
//!     }
//! }
//!
//! let foobar = Node("foo", vec![
//!     Node("bar", vec![]),
//!     Node("zup", vec![]),
//! ]);
//! // foobar is a tree; its root, "foo", has two children, named "bar" and
//! // "zup".
//!
//! let mut cur = TreeCursor::new(&foobar);
//! assert_eq!(cur.get().0, "foo"); // The cursor starts at the root, "foo".
//! // TreeCursor::get() returns a reference to the active node.
//!
//! assert!(cur.down());
//! // TreeCursor::down returns true if the position actually moved.
//! assert_eq!(cur.get().0, "bar");
//!
//! assert!(!cur.down());
//! // "bar" has no children so we can't move the cursor down.
//!
//! assert!(cur.up());
//! assert_eq!(cur.get().0, "foo"); // The cursor is at the root again.
//!
//! assert!(cur.down());
//! assert_eq!(cur.get().0, "zup");
//!
//! assert!(cur.up());
//! assert_eq!(cur.get().0, "foo"); // Back at the root.
//!
//! assert!(!cur.down()); // No more children to visit.
//! assert!(!cur.up()); // The root has no parent.
//! ```
//!
//! When the cursor is created, its initial position is at the root of the
//! tree. [`down`] and [`up`] are two methods for repositioning the cursor.
//! There are several methods besides [`down`] for moving down a tree;
//! typically [`down`] is used when fully traversing a tree, as we just saw.
//! Here are two ways you might do a full traversal if you don't know the
//! tree's exact shape ahead of time:
//!
//! ```
//! # use tree_cursor::cursor::TreeCursor;
//! # use tree_cursor::prelude::*;
//! #
//! # struct Node(&'static str, Vec<Node>);
//! #
//! # impl Down for Node {
//! #     fn down(&self, idx: usize) -> Option<&Self> {
//! #         self.1.get(idx)
//! #     }
//! # }
//! #
//! # fn process_node(_: &Node) { }
//! #
//! fn process_tree_pre_order(root: &Node) {
//!     let mut cur = TreeCursor::new(root);
//!     'outer: loop {
//!         process_node(cur.get());
//!         while !cur.down() {
//!             if !cur.up() { break 'outer; }
//!         }
//!     }
//! }
//!
//! fn process_tree_post_order(root: &Node) {
//!     let mut cur = TreeCursor::new(root);
//!     loop {
//!         while cur.down() { }
//!         process_node(cur.get());
//!         if !cur.up() { break; }
//!     }
//! }
//! ```
//!
//! When you need more complex behavior or when there's no particular order to
//! a node's children, you can use the [`down_map`] method instead, passing it
//! a closure that determines the next child to visit.
//!
//! # Mutability and node references
//!
//! [`TreeCursor`] is the immutable version of the tree cursor, meaning it
//! holds a shared reference to the tree, preventing you from modifying the
//! tree until the cursor goes out of scope. If you need to modify the tree,
//! use [`TreeCursorMut`] instead, which gives you access to a mutable
//! reference to the active node.
//!
//! [`Cell`]: std::cell::Cell
//! [`RefCell`]: std::cell::RefCell
//!
//! [`TreeCursor`]: cursor::TreeCursor
//! [`TreeCursorMut`]: cursor::TreeCursorMut
//! [`down`]: cursor::TreeCursor::down
//! [`down_map`]: cursor::TreeCursor::down_map
//! [`up`]: cursor::TreeCursor::up
//! [`get`]: cursor::TreeCursor::get
//! [`get_mut`]: cursor::TreeCursorMut::get_mut

pub mod cursor;

pub mod prelude {
    pub use super::{Down, DownMut};
}

#[cfg(test)]
mod tests;

pub trait Down {
    /// See [`TreeCursor::down`].
    ///
    /// [`TreeCursor::down`]: cursor::TreeCursor::down
    fn down(&self, idx: usize) -> Option<&Self>;
}

pub trait DownMut {
    /// See [`TreeCursorMut::down`].
    ///
    /// [`TreeCursorMut::down`]: cursor::TreeCursorMut::down
    fn down_mut(&mut self, idx: usize) -> Option<&mut Self>;
}
