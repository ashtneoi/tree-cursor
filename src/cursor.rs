use prelude::*;
use std::marker::PhantomData;

/// A cursor that holds a shared reference to its tree.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TreeCursor<'n, N: 'n> {
    root: PhantomData<&'n N>,
    stack: Vec<(*const N, usize)>,
}

impl<'n, N: 'n> TreeCursor<'n, N> {
    /// Creates a new `TreeCursor` starting at `root`.
    pub fn new(root: &'n N) -> Self {
        Self {
            root: PhantomData,
            stack: vec![(root as *const N, 0)],
        }
    }

    fn down_map_ptr<F>(&mut self, f: F) -> Option<*const N>
    where
        F: Fn(&'n N, usize) -> Option<&'n N>,
    {
        let idx = self.stack.last().unwrap().1;
        let here_ptr = self.get() as *const N;
        let new_ptr =
            f(unsafe { here_ptr.as_ref().unwrap() }, idx)? as *const N;
        self.stack.last_mut().unwrap().1 += 1;
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

    /// Like [`down_new`], except that it takes a closure like [`down_map`].
    ///
    /// [`down_new`]: TreeCursor::down_new
    /// [`down_map`]: TreeCursor::down_map
    pub fn down_map_new<F>(&mut self, f: F) -> Option<Self>
    where
        F: Fn(&'n N, usize) -> Option<&'n N>,
    {
        let new_ptr = self.down_map_ptr(f)?;
        Some(Self::new(unsafe { new_ptr.as_ref().unwrap() }))
    }

    /// Resets the active node's "next child" counter to 0.
    pub fn zero(&mut self) {
        self.stack.last_mut().unwrap().1 = 0;
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

    /// Like [`up`], except it also returns a new `TreeCursor` whose root is
    /// the old position. `self` is frozen until the new cursor goes out of
    /// scope.
    ///
    /// [`up`]: TreeCursor::up
    pub fn get_new(&mut self) -> Option<Self> {
        if self.stack.len() == 1 {
            self.stack[0].1 = 0;
            None
        } else {
            let (old_ptr, _) = self.stack.pop().unwrap();
            Some(Self::new(unsafe { old_ptr.as_ref().unwrap() }))
        }
    }

    /// Returns a shared reference to the active node.
    pub fn get(&self) -> &N {
        let here: *const N = self.stack.last().unwrap().0;
        unsafe { here.as_ref().unwrap() }
    }
}

impl<'n, N: 'n + Down> TreeCursor<'n, N> {
    fn down_ptr(&mut self) -> Option<*const N> {
        let idx = self.stack.last().unwrap().1;
        let new_ptr = self.get().down(idx)? as *const N;
        self.stack.last_mut().unwrap().1 += 1;
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
    pub fn down_new(&mut self) -> Option<Self> {
        let new_ptr = self.down_ptr()?;
        Some(Self::new(unsafe { new_ptr.as_ref().unwrap() }))
    }
}

/// A cursor that holds a mutable reference to its tree.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct TreeCursorMut<'n, N: 'n> {
    root: PhantomData<&'n mut N>,
    stack: Vec<(*mut N, usize)>,
}

impl<'n, N: 'n> TreeCursorMut<'n, N> {
    /// Creates a new `TreeCursorMut` starting at `root`.
    pub fn new(root: &'n mut N) -> Self {
        let root_ptr: *mut N = root;
        Self {
            root: PhantomData,
            stack: vec![(root_ptr, 0)],
        }
    }

    fn down_map_ptr<F>(&mut self, f: F) -> Option<*mut N>
    where
        F: Fn(&'n mut N, usize) -> Option<&'n mut N>,
    {
        let idx = self.stack.last().unwrap().1;
        let here_ptr = self.get_mut() as *mut N;
        let new_ptr = f(unsafe { here_ptr.as_mut().unwrap() }, idx)? as *mut N;
        self.stack.last_mut().unwrap().1 += 1;
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

    /// Like [`down_new`], except that it takes a closure like [`down_map`].
    ///
    /// [`down_new`]: TreeCursorMut::down_new
    /// [`down_map`]: TreeCursorMut::down_map
    pub fn down_map_new<F>(&mut self, f: F) -> Option<Self>
    where
        F: Fn(&'n mut N, usize) -> Option<&'n mut N>,
    {
        let new_ptr = self.down_map_ptr(f)?;
        Some(Self::new(unsafe { new_ptr.as_mut().unwrap() }))
    }

    /// Resets the active node's "next child" counter to 0.
    pub fn zero(&mut self) {
        self.stack.last_mut().unwrap().1 = 0;
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

    /// Like [`up`], except it also returns a new `TreeCursorMut` whose root is
    /// the old position. `self` is frozen until the new cursor goes out of
    /// scope.
    ///
    /// [`up`]: TreeCursorMut::up
    pub fn get_new(&mut self) -> Option<Self> {
        if self.stack.len() == 1 {
            self.stack[0].1 = 0;
            None
        } else {
            let (old_ptr, _) = self.stack.pop().unwrap();
            Some(Self::new(unsafe { old_ptr.as_mut().unwrap() }))
        }
    }

    /// Returns a shared reference to the active node.
    pub fn get(&self) -> &N {
        let here: *const N = self.stack.last().unwrap().0;
        (unsafe { here.as_ref() }).unwrap()
    }

    /// Returns a mutable reference to the active node.
    pub fn get_mut(&mut self) -> &mut N {
        let here = self.stack.last().unwrap().0;
        (unsafe { here.as_mut() }).unwrap()
    }
}

impl<'n, N: 'n + DownMut> TreeCursorMut<'n, N> {
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
    pub fn down_new(&mut self) -> Option<Self> {
        let new_ptr = self.down_ptr()?;
        Some(Self::new(unsafe { new_ptr.as_mut().unwrap() }))
    }
}
