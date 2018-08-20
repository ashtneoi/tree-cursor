use prelude::*;
use std::marker::PhantomData;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TreeCursor<'n: 'f, 'f, N: 'n> {
    root: PhantomData<&'n N>,
    frozen: PhantomData<&'f ()>,
    stack: Vec<(*const N, usize)>,
}

impl<'n, N: 'n> TreeCursor<'n, 'n, N> {
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

    fn down_ptr_with<F>(&mut self, f: F) -> Option<*const N>
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

    pub fn down_with<F>(&mut self, f: F) -> bool
    where
        F: Fn(&'n N, usize) -> Option<&'n N>,
    {
        let maybe_new_ptr = self.down_ptr_with(f);
        if let &Some(new_ptr) = &maybe_new_ptr {
            self.stack.push((new_ptr, 0));
        }
        maybe_new_ptr.is_some()
    }

    pub fn split_below_with<'s, F>(&'s mut self, f: F)
        -> Option<TreeCursor<'n, 's, N>>
    where
        F: Fn(&'n N, usize) -> Option<&'n N>,
    {
        let new_ptr = self.down_ptr_with(f)?;
        Some(Self {
            root: PhantomData,
            frozen: PhantomData,
            stack: vec![(new_ptr, 0)],
        })
    }

    pub fn zero(&mut self) {
        self.top_mut().1 = 0;
    }

    pub fn up(&mut self) -> bool {
        if self.stack.len() == 1 {
            self.stack[0].1 = 0;
            false
        } else {
            self.stack.pop().unwrap();
            true
        }
    }

    pub fn split_above<'s>(&'s mut self) -> Option<TreeCursor<'n, 's, N>> {
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

    pub fn down(&mut self) -> bool {
        let maybe_new_ptr = self.down_ptr();
        if let &Some(new_ptr) = &maybe_new_ptr {
            self.stack.push((new_ptr, 0));
        }
        maybe_new_ptr.is_some()
    }

    pub fn split_below<'s>(&'s mut self) -> Option<TreeCursor<'n, 's, N>> {
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

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct TreeCursorMut<'n: 'f, 'f, N: 'n> {
    root: PhantomData<&'n mut N>,
    frozen: PhantomData<&'f ()>,
    stack: Vec<(*mut N, usize)>,
}

impl<'n, N: 'n> TreeCursorMut<'n, 'n, N> {
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

    fn down_ptr_with<F>(&mut self, f: F) -> Option<*mut N>
    where
        F: Fn(&'n mut N, usize) -> Option<&'n mut N>,
    {
        let idx = self.top().1;
        let here_ptr = self.get_mut() as *mut N;
        let new_ptr = f(unsafe { here_ptr.as_mut().unwrap() }, idx)? as *mut N;
        self.top_mut().1 += 1;
        Some(new_ptr)
    }

    pub fn down_with<F>(&mut self, f: F) -> bool
    where
        F: Fn(&'n mut N, usize) -> Option<&'n mut N>,
    {
        let maybe_new_ptr = self.down_ptr_with(f);
        if let &Some(new_ptr) = &maybe_new_ptr {
            self.stack.push((new_ptr, 0));
        }
        maybe_new_ptr.is_some()
    }

    pub fn split_below_with<'s, F>(&'s mut self, f: F)
        -> Option<TreeCursorMut<'n, 's, N>>
    where
        F: Fn(&'n mut N, usize) -> Option<&'n mut N>,
    {
        let new_ptr = self.down_ptr_with(f)?;
        Some(Self {
            root: PhantomData,
            frozen: PhantomData,
            stack: vec![(new_ptr, 0)],
        })
    }

    pub fn zero(&mut self) {
        self.top_mut().1 = 0;
    }

    pub fn up(&mut self) -> bool {
        if self.stack.len() == 1 {
            self.stack[0].1 = 0;
            false
        } else {
            self.stack.pop().unwrap();
            true
        }
    }

    pub fn split_above<'s>(&'s mut self) -> Option<TreeCursorMut<'n, 's, N>> {
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

    pub fn get(&self) -> &N {
        let here: *const N = self.top().0;
        (unsafe { here.as_ref() }).unwrap()
    }

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

impl<'n: 'f, 'f, N: 'n + TakeChild> TreeCursorMut<'n, 'f, N> {
    pub fn take_node<'s>(&'s mut self) -> Option<N> {
        if self.up() {
            let (here_ptr, here_idx) = self.stack.last_mut().unwrap();
            let here = unsafe { here_ptr.as_mut().unwrap() };
            assert!(*here_idx > 0);
            *here_idx -= 1;
            Some(here.take_child(*here_idx))
        } else {
            None
        }
    }
}

pub struct TreeCursorPos(Vec<usize>);

impl<'n: 'f, 'f, N: 'n + DownMut> TreeCursorMut<'n, 'f, N> {
    pub fn pos(&self) -> TreeCursorPos {
        TreeCursorPos(self.stack.iter().map(|&(_, idx)| idx).collect())
    }

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

    pub fn down(&mut self) -> bool {
        let maybe_new_ptr = self.down_ptr();
        if let &Some(new_ptr) = &maybe_new_ptr {
            self.stack.push((new_ptr, 0));
        }
        maybe_new_ptr.is_some()
    }

    pub fn split_below<'s>(
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
