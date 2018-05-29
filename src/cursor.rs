use prelude::*;
use std::marker::PhantomData;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TreeCursor<'n, N: 'n> {
    root: PhantomData<&'n N>,
    stack: Vec<(*const N, usize)>,
}

impl<'n, N: 'n> TreeCursor<'n, N> {
    pub fn new(root: &'n N) -> Self {
        Self {
            root: PhantomData,
            stack: vec![(root as *const N, 0)],
        }
    }

    fn down_map_ptr<F>(&mut self, f: F) -> Option<*const N>
    where
        F: Fn(&N, usize) -> Option<&N>,
    {
        let idx = self.stack.last().unwrap().1;
        let new_ptr = f(self.get(), idx)? as *const N;
        self.stack.last_mut().unwrap().1 += 1;
        Some(new_ptr)
    }

    pub fn down_map<F>(&mut self, f: F) -> bool
    where
        F: Fn(&N, usize) -> Option<&N>,
    {
        let maybe_new_ptr = self.down_map_ptr(f);
        if let &Some(new_ptr) = &maybe_new_ptr {
            self.stack.push((new_ptr, 0));
        }
        maybe_new_ptr.is_some()
    }

    pub fn down_map_new<F>(&mut self, f: F) -> Option<Self>
    where
        F: Fn(&N, usize) -> Option<&N>,
    {
        let new_ptr = self.down_map_ptr(f)?;
        Some(Self::new(unsafe { new_ptr.as_ref().unwrap() }))
    }

    pub fn zero(&mut self) {
        self.stack.last_mut().unwrap().1 = 0;
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

    pub fn down(&mut self) -> bool {
        let maybe_new_ptr = self.down_ptr();
        if let &Some(new_ptr) = &maybe_new_ptr {
            self.stack.push((new_ptr, 0));
        }
        maybe_new_ptr.is_some()
    }

    pub fn down_new(&mut self) -> Option<Self> {
        let new_ptr = self.down_ptr()?;
        Some(Self::new(unsafe { new_ptr.as_ref().unwrap() }))
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct TreeCursorMut<'n, N: 'n> {
    root: PhantomData<&'n mut N>,
    stack: Vec<(*mut N, usize)>,
}

impl<'n, N: 'n + DownMut> TreeCursorMut<'n, N> {
    pub fn new(root: &'n mut N) -> Self {
        let root_ptr: *mut N = root;
        Self {
            root: PhantomData,
            stack: vec![(root_ptr, 0)],
        }
    }

    fn down_map_ptr<F>(&mut self, f: F) -> Option<*mut N>
    where
        F: Fn(&mut N, usize) -> Option<&mut N>,
    {
        let idx = self.stack.last().unwrap().1;
        let new_ptr = f(self.get_mut(), idx)? as *mut N;
        self.stack.last_mut().unwrap().1 += 1;
        Some(new_ptr)
    }

    pub fn down_map<F>(&mut self, f: F) -> bool
    where
        F: Fn(&mut N, usize) -> Option<&mut N>,
    {
        let maybe_new_ptr = self.down_map_ptr(f);
        if let &Some(new_ptr) = &maybe_new_ptr {
            self.stack.push((new_ptr, 0));
        }
        maybe_new_ptr.is_some()
    }

    pub fn down_map_new<F>(&mut self, f: F) -> Option<Self>
    where
        F: Fn(&mut N, usize) -> Option<&mut N>,
    {
        let new_ptr = self.down_map_ptr(f)?;
        Some(Self::new(unsafe { new_ptr.as_mut().unwrap() }))
    }

    pub fn zero(&mut self) {
        self.stack.last_mut().unwrap().1 = 0;
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

    pub fn get(&self) -> &N {
        let here: *const N = self.stack.last().unwrap().0;
        (unsafe { here.as_ref() }).unwrap()
    }

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

    pub fn down(&mut self) -> bool {
        let maybe_new_ptr = self.down_ptr();
        if let &Some(new_ptr) = &maybe_new_ptr {
            self.stack.push((new_ptr, 0));
        }
        maybe_new_ptr.is_some()
    }

    pub fn down_new(&mut self) -> Option<Self> {
        let new_ptr = self.down_ptr()?;
        Some(Self::new(unsafe { new_ptr.as_mut().unwrap() }))
    }
}
