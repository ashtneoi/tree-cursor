use prelude::*;
use std::marker::PhantomData;

/// A cursor that holds a shared reference to its tree.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TreeCursor<'n: 'f, 'f, N: 'n> {
    root: PhantomData<&'n N>,
    frozen: PhantomData<&'f ()>,
    stack: Vec<(*const N, usize)>,
}

impl<'n, N: 'n> TreeCursor<'n, 'n, N> {
    /// Creates a new `TreeCursor` starting at `root`.
    pub fn new(root: &'n N) -> Self {
        Self {
            root: PhantomData,
            frozen: PhantomData,
            stack: vec![(root as *const N, 0)],
        }
    }
}

impl<'n: 'f, 'f, N: 'n> TreeCursor<'n, 'f, N> {
    fn top(&self) -> &(*const N, usize) {
        self.stack.last().unwrap()
    }

    fn top_mut(&mut self) -> &mut (*const N, usize) {
        self.stack.last_mut().unwrap()
    }

    fn down_map_ptr<F>(&mut self, f: F) -> Option<*const N>
    where
        F: Fn(&'n N, usize) -> Option<&'n N>,
    {
        let idx = self.top().1;
        let here_ptr = self.get() as *const N;
        let new_ptr =
            f(unsafe { here_ptr.as_ref().unwrap() }, idx)? as *const N;
        self.top_mut().1 += 1;
        Some(new_ptr)
    }

    /// Passes `f` the active node and the current value of the "next child"
    /// counter. If `f` returns a node, it's set as the active node, the old
    /// active node's "next child" counter is incremented, and this method
    /// returns true. Otherwise, this method returns false.
    pub fn down_map<F>(&mut self, f: F) -> bool
    where
        F: Fn(&'n N, usize) -> Option<&'n N>,
    {
        let maybe_new_ptr = self.down_map_ptr(f);
        if let &Some(new_ptr) = &maybe_new_ptr {
            self.stack.push((new_ptr, 0));
        }
        maybe_new_ptr.is_some()
    }

    /// Like [`down_take_cursor`], except that it takes a closure like
    /// [`down_map`].
    ///
    /// [`down_take_cursor`]: TreeCursor::down_take_cursor
    /// [`down_map`]: TreeCursor::down_map
    pub fn down_map_take_cursor<'s, F>(&'s mut self, f: F)
        -> Option<TreeCursor<'n, 's, N>>
    where
        F: Fn(&'n N, usize) -> Option<&'n N>,
    {
        let new_ptr = self.down_map_ptr(f)?;
        Some(Self {
            root: PhantomData,
            frozen: PhantomData,
            stack: vec![(new_ptr, 0)],
        })
    }

    /// Resets the active node's "next child" counter to 0.
    pub fn zero(&mut self) {
        self.top_mut().1 = 0;
    }

    /// Moves the cursor up one node. Returns true if there was a node to move
    /// to, and false otherwise. In both cases, the old active node's "next
    /// child" counter is reset, as if [`zero`] had been called.
    ///
    /// [`zero`]: TreeCursor::zero
    pub fn up(&mut self) -> bool {
        if self.stack.len() == 1 {
            self.stack[0].1 = 0;
            false
        } else {
            self.stack.pop().unwrap();
            true
        }
    }

    /// Takes the active node from this `TreeCursor` and returns a new
    /// `TreeCursor` at that position. `self` is frozen until the new cursor
    /// goes out of scope.
    pub fn take_cursor<'s>(&'s mut self) -> Option<TreeCursor<'n, 's, N>> {
        if self.stack.len() == 1 {
            None
        } else {
            let (old_ptr, old_idx) = self.stack.pop().unwrap();
            let old = unsafe { old_ptr.as_ref().unwrap() };
            Some(Self {
                root: PhantomData,
                frozen: PhantomData,
                stack: vec![(old, old_idx)],
            })
        }
    }

    /// Returns a shared reference to the active node.
    pub fn get(&self) -> &N {
        let here: *const N = self.top().0;
        unsafe { here.as_ref().unwrap() }
    }
}

impl<'n: 'f, 'f, N: 'n + Down> TreeCursor<'n, 'f, N> {
    fn down_ptr(&mut self) -> Option<*const N> {
        let idx = self.top().1;
        let new_ptr = self.get().down(idx)? as *const N;
        self.top_mut().1 += 1;
        Some(new_ptr)
    }

    /// Moves the cursor down one node. The node to move to is determined by
    /// calling [`Down::down`] on the active node and passing it the "next
    /// child" counter. Returns true and increments the old active node's
    /// "next child" counter if there was a node to move to, and returns false
    /// otherwise.
    pub fn down(&mut self) -> bool {
        let maybe_new_ptr = self.down_ptr();
        if let &Some(new_ptr) = &maybe_new_ptr {
            self.stack.push((new_ptr, 0));
        }
        maybe_new_ptr.is_some()
    }

    /// Like [`down`], except instead of moving the position of `self`, it
    /// returns a new `TreeCursor` whose root is the new position. `self` is
    /// frozen until the new cursor goes out of scope.
    ///
    /// [`down`]: TreeCursor::down
    pub fn down_take_cursor<'s>(&'s mut self) -> Option<TreeCursor<'n, 's, N>> {
        let new_ptr = self.down_ptr()?;
        Some(Self {
            root: PhantomData,
            frozen: PhantomData,
            stack: vec![(new_ptr, 0)],
        })
    }
}

impl<'n: 'f, 'f, N: 'n> From<TreeCursorMut<'n, 'f, N>>
    for TreeCursor<'n, 'f, N>
{
    fn from(mut cm: TreeCursorMut<'n, 'f, N>) -> Self {
        TreeCursor {
            root: PhantomData,
            frozen: PhantomData,
            stack: cm.stack.drain(..).map(|(p, n)| (p as *const N, n))
                .collect(), // TODO: speed this up
        }
    }
}

/// A cursor that holds a mutable reference to its tree.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct TreeCursorMut<'n: 'f, 'f, N: 'n> {
    root: PhantomData<&'n mut N>,
    frozen: PhantomData<&'f ()>,
    stack: Vec<(*mut N, usize)>,
}

impl<'n, N: 'n> TreeCursorMut<'n, 'n, N> {
    /// Creates a new `TreeCursorMut` starting at `root`.
    pub fn new(root: &'n mut N) -> Self {
        Self {
            root: PhantomData,
            frozen: PhantomData,
            stack: vec![(root as *mut N, 0)],
        }
    }
}

impl<'n: 'f, 'f, N: 'n> TreeCursorMut<'n, 'f, N> {
    fn top(&self) -> &(*mut N, usize) {
        self.stack.last().unwrap()
    }

    fn top_mut(&mut self) -> &mut (*mut N, usize) {
        self.stack.last_mut().unwrap()
    }

    fn down_map_ptr<F>(&mut self, f: F) -> Option<*mut N>
    where
        F: Fn(&'n mut N, usize) -> Option<&'n mut N>,
    {
        let idx = self.top().1;
        let here_ptr = self.get_mut() as *mut N;
        let new_ptr = f(unsafe { here_ptr.as_mut().unwrap() }, idx)? as *mut N;
        self.top_mut().1 += 1;
        Some(new_ptr)
    }

    /// Passes `f` the active node and the current value of the "next child"
    /// counter. If `f` returns a node, it's set as the active node, the old
    /// active node's "next child" counter is incremented, and this method
    /// returns true. Otherwise, this method returns false.
    pub fn down_map<F>(&mut self, f: F) -> bool
    where
        F: Fn(&'n mut N, usize) -> Option<&'n mut N>,
    {
        let maybe_new_ptr = self.down_map_ptr(f);
        if let &Some(new_ptr) = &maybe_new_ptr {
            self.stack.push((new_ptr, 0));
        }
        maybe_new_ptr.is_some()
    }

    /// Like [`down_take_cursor`], except that it takes a closure like
    /// [`down_map`].
    ///
    /// [`down_take_cursor`]: TreeCursorMut::down_take_cursor
    /// [`down_map`]: TreeCursorMut::down_map
    pub fn down_map_take_cursor<'s, F>(&'s mut self, f: F)
        -> Option<TreeCursorMut<'n, 's, N>>
    where
        F: Fn(&'n mut N, usize) -> Option<&'n mut N>,
    {
        let new_ptr = self.down_map_ptr(f)?;
        Some(Self {
            root: PhantomData,
            frozen: PhantomData,
            stack: vec![(new_ptr, 0)],
        })
    }

    /// Resets the active node's "next child" counter to 0.
    pub fn zero(&mut self) {
        self.top_mut().1 = 0;
    }

    /// Moves the cursor up one node. Returns true if there was a node to move
    /// to, and false otherwise. In both cases, the old active node's "next
    /// child" counter is reset, as if [`zero`] had been called.
    ///
    /// [`zero`]: TreeCursorMut::zero
    pub fn up(&mut self) -> bool {
        if self.stack.len() == 1 {
            self.stack[0].1 = 0;
            false
        } else {
            self.stack.pop().unwrap();
            true
        }
    }

    /// Takes the active node from this `TreeCursorMut` and returns a new
    /// `TreeCursorMut` at that position. `self` is frozen until the new cursor
    /// goes out of scope.
    pub fn take_cursor<'s>(&'s mut self) -> Option<TreeCursorMut<'n, 's, N>> {
        if self.stack.len() == 1 {
            None
        } else {
            let (old_ptr, old_idx) = self.stack.pop().unwrap();
            let old = unsafe { old_ptr.as_mut().unwrap() };
            Some(Self {
                root: PhantomData,
                frozen: PhantomData,
                stack: vec![(old, old_idx)],
            })
        }
    }

    /// Returns a shared reference to the active node.
    pub fn get(&self) -> &N {
        let here: *const N = self.top().0;
        (unsafe { here.as_ref() }).unwrap()
    }

    /// Returns a mutable reference to the active node.
    pub fn get_mut(&mut self) -> &mut N {
        let here = self.top().0;
        (unsafe { here.as_mut() }).unwrap()
    }

    pub fn as_cursor<'s>(&'s self) -> TreeCursor<'n, 's, N> {
        TreeCursor {
            root: PhantomData,
            frozen: PhantomData,
            stack: self.stack.iter()
                .map(|&(p, n)| (p as *const N, n)).collect(),
        }
    }
}

/// Stores a cursor's position at an earlier point in time.
pub struct TreeCursorPos(Vec<usize>);

impl<'n: 'f, 'f, N: 'n + DownMut> TreeCursorMut<'n, 'f, N> {
    /// Returns an opaque object that stores the current position of the cursor.
    /// Pass it to [`set_pos`] to restore that position.
    ///
    /// [`set_pos`]: TreeCursorMut::set_pos
    pub fn pos(&self) -> TreeCursorPos {
        TreeCursorPos(self.stack.iter().map(|&(_, idx)| idx).collect())
    }

    /// Moves the cursor to the given position, as long as tree mutation hasn't
    /// invalidated the position since it was retrieved.
    ///
    /// # Panics
    ///
    /// If the tree has changed such that the position is no longer valid, this
    /// method panics. However, since the position is stored using "next child"
    /// indices (not pointers), it remains valid as long as the tree has a node
    /// in that position, even if the node's value changes or it's replaced with
    /// another node. If this is a problem, you should track the position's
    /// validity yourself.
    ///
    /// [`pos`]: TreeCursorMut::pos
    pub fn set_pos(&mut self, pos: &TreeCursorPos) {
        self.stack.truncate(1);
        for &idx in pos.0.iter().rev().skip(1).rev() { // TODO: ugly
            self.top_mut().1 = idx - 1;
            if !self.down() {
                panic!("missing node in TreeCursorPos");
            }
        }
        let &idx = pos.0.last().unwrap();
        self.top_mut().1 = idx;
    }

    fn down_ptr(&mut self) -> Option<*mut N> {
        let idx = self.stack.last().unwrap().1;
        let new_ptr = self.get_mut().down_mut(idx)? as *mut N;
        self.stack.last_mut().unwrap().1 += 1;
        Some(new_ptr)
    }

    /// Moves the cursor down one node. The node to move to is determined by
    /// calling [`DownMut::down_mut`] on the active node and passing it the
    /// "next child" counter. Returns true and increments the old active node's
    /// "next child" counter if there was a node to move to, and returns false
    /// otherwise.
    pub fn down(&mut self) -> bool {
        let maybe_new_ptr = self.down_ptr();
        if let &Some(new_ptr) = &maybe_new_ptr {
            self.stack.push((new_ptr, 0));
        }
        maybe_new_ptr.is_some()
    }

    /// Like [`down`], except instead of moving the position of `self`, it
    /// returns a new `TreeCursorMut` whose root is the new position. `self` is
    /// frozen until the new cursor goes out of scope.
    ///
    /// [`down`]: TreeCursorMut::down
    pub fn down_take_cursor<'s>(
        &'s mut self
    ) -> Option<TreeCursorMut<'n, 's, N>> {
        let new_ptr = self.down_ptr()?;
        Some(Self {
            root: PhantomData,
            frozen: PhantomData,
            stack: vec![(new_ptr, 0)],
        })
    }
}
