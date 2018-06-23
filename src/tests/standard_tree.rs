use cursor::{SetPosError, TreeCursor, TreeCursorMut};
use prelude::*;
use std::ptr;

#[derive(Clone)]
struct X {
    v: Vec<X>,
}

impl Down for X {
    fn down(&self, idx: usize) -> Option<&Self> {
        self.v.get(idx)
    }
}

impl DownMut for X {
    fn down_mut(&mut self, idx: usize) -> Option<&mut Self> {
        self.v.get_mut(idx)
    }
}

fn xx(v: Vec<X>) -> X {
    X { v }
}

fn x() -> X {
    X { v: vec![] }
}

#[test]
fn full_traversal() {
    fn go(t: &mut X) -> usize {
        let mut down_count = 0;
        {
            let mut up_count = 0;
            let mut c = TreeCursor::new(t);
            loop {
                while c.down() { down_count += 1; }
                if !c.up() { break; }
                up_count += 1;
            }
            assert!(c.down());
            assert_eq!(down_count, up_count);
        }

        let mut down_count_mut = 0;
        {
            let mut up_count_mut = 0;
            let mut c = TreeCursorMut::new(t);
            loop {
                while c.down() { down_count_mut += 1; }
                if !c.up() { break; }
                up_count_mut += 1;
            }
            assert!(c.down());
            assert_eq!(down_count_mut, up_count_mut);
        }

        assert_eq!(down_count, down_count_mut);

        down_count
    }

    {
        let t = x();
        let mut c = TreeCursor::new(&t);
        assert!(!c.down());
        assert!(!c.up());
        assert!(!c.down());
    }
    assert_eq!(
        go(
            &mut xx(vec![
                x(),
            ])
        ),
        1
    );
    assert_eq!(
        go(
            &mut xx(vec![
                x(),
                x(),
            ])
        ),
        2
    );
    assert_eq!(
        go(
            &mut xx(vec![
                xx(vec![
                    x()
                ]),
                x(),
            ])
        ),
        3
    );
    assert_eq!(
        go(
            &mut xx(vec![
                xx(vec![
                    x()
                ]),
                xx(vec![
                    x()
                ]),
            ])
        ),
        4
    );
}

#[test]
fn get() {
    let t = xx(vec![
        xx(vec![
            x(),
            x(),
        ]),
        x(),
    ]);
    let mut mt = t.clone();
    let mut mt2 = t.clone();

    let mut c = TreeCursor::new(&t);
    let mut c2;
    let mut cm = TreeCursorMut::new(&mut mt);
    let mut cm2 = TreeCursorMut::new(&mut mt2);
    assert!(ptr::eq(c.get(), &t));
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 2);
    assert_eq!(cm2.get().v.len(), 2);

    assert!(c.down());
    assert!(cm.down());
    assert!(cm2.down());
    assert!(ptr::eq(c.get(), &t.v[0]));
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 2);
    assert_eq!(cm2.get().v.len(), 2);

    assert!(c.down());
    assert!(cm.down());
    assert!(cm2.down());
    assert!(ptr::eq(c.get(), &t.v[0].v[0]));
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 0);
    assert_eq!(cm2.get().v.len(), 0);

    assert!(!c.down());
    assert!(!cm.down());
    assert!(!cm2.down());
    assert!(ptr::eq(c.get(), &t.v[0].v[0]));
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 0);
    assert_eq!(cm2.get().v.len(), 0);

    assert!(c.up());
    assert!(cm.up());
    assert!(cm2.up());
    assert!(ptr::eq(c.get(), &t.v[0]));
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 2);
    assert_eq!(cm2.get().v.len(), 2);

    c2 = TreeCursor::from(cm2);

    assert!(c.down());
    assert!(c2.down());
    assert!(cm.down());
    assert!(ptr::eq(c.get(), &t.v[0].v[1]));
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 0);
    assert_eq!(c2.get().v.len(), 0);

    let cm_pos = cm.pos();

    assert!(!c.down());
    assert!(!c2.down());
    assert!(!cm.down());
    assert!(ptr::eq(c.get(), &t.v[0].v[1]));
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 0);
    assert_eq!(c2.get().v.len(), 0);

    assert!(c.up());
    assert!(c2.up());
    assert!(cm.up());
    assert!(ptr::eq(c.get(), &t.v[0]));
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 2);
    assert_eq!(c2.get().v.len(), 2);

    assert!(!c.down());
    assert!(!c2.down());
    assert!(!cm.down());
    assert!(ptr::eq(c.get(), &t.v[0]));
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 2);
    assert_eq!(c2.get().v.len(), 2);

    assert!(c.up());
    assert!(c2.up());
    assert!(cm.up());
    assert!(ptr::eq(c.get(), &t));
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 2);
    assert_eq!(c2.get().v.len(), 2);

    assert!(c.down());
    assert!(c2.down());
    assert!(cm.down());
    assert!(ptr::eq(c.get(), &t.v[1]));
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 0);
    assert_eq!(c2.get().v.len(), 0);

    assert!(!c.down());
    assert!(!c2.down());
    assert!(!cm.down());
    assert!(ptr::eq(c.get(), &t.v[1]));
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 0);
    assert_eq!(c2.get().v.len(), 0);

    assert!(c.up());
    assert!(c2.up());
    assert!(cm.up());
    assert!(ptr::eq(c.get(), &t));
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 2);
    assert_eq!(c2.get().v.len(), 2);

    assert!(!c.up());
    assert!(!c2.up());
    assert!(!cm.up());
    assert!(ptr::eq(c.get(), &t));
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 2);
    assert_eq!(c2.get().v.len(), 2);

    cm.set_pos(&cm_pos).unwrap();

    assert!(!cm.down());
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 0);

    assert!(cm.up());
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 2);

    assert!(!cm.down());
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 2);

    assert!(cm.up());
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 2);

    assert!(cm.down());
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 0);

    assert!(!cm.down());
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 0);

    assert!(cm.up());
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 2);

    assert!(!cm.up());
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 2);
}

#[test]
fn scoped_down() {
    let t = xx(vec![
        xx(vec![
            x(),
            xx(vec![
                x(),
            ]),
            x(),
        ]),
    ]);
    let mut mt = t.clone();

    let mut c = TreeCursor::new(&t);
    let mut cm = TreeCursorMut::new(&mut mt);

    {
        let mut c = c.down_new().unwrap();
        let mut cm = cm.down_new().unwrap();
        assert!(!c.up());
        assert!(!cm.up());
        {
            let mut c = c.down_new().unwrap();
            let mut cm = cm.down_new().unwrap();
            assert!(!c.up());
            assert!(!cm.up());
            assert!(c.down_new().is_none());
            assert!(cm.down_new().is_none());
            assert!(!c.up());
            assert!(!cm.up());
        }
        {
            let mut c = c.down_new().unwrap();
            let mut cm = cm.down_new().unwrap();
            assert!(!c.up());
            assert!(!cm.up());
            {
                let mut c = c.down_new().unwrap();
                let mut cm = cm.down_new().unwrap();
                assert!(!c.up());
                assert!(!cm.up());
                assert!(c.down_new().is_none());
                assert!(cm.down_new().is_none());
                assert!(!c.up());
                assert!(!cm.up());
            }
            assert!(c.down_new().is_none());
            assert!(cm.down_new().is_none());
            assert!(!c.up());
            assert!(!cm.up());
            assert!(c.down());
            assert!(cm.down());
        }
        {
            let mut c = c.down_new().unwrap();
            let mut cm = cm.down_new().unwrap();
            assert!(c.down_new().is_none());
            assert!(cm.down_new().is_none());
        }
        assert!(c.down_new().is_none());
        assert!(cm.down_new().is_none());
        assert!(!c.up());
        assert!(!cm.up());
        assert!(c.down());
        assert!(cm.down());
    }
    assert!(c.down_new().is_none());
    assert!(cm.down_new().is_none());
    assert!(!c.up());
    assert!(!cm.up());
    assert!(c.down());
    assert!(cm.down());
}

#[test]
fn take() {
    let t = xx(vec![
        xx(vec![
            x(),
            xx(vec![
                x(),
            ]),
            x(),
        ]),
    ]);
    let mut mt = t.clone();

    let mut c = TreeCursor::new(&t);
    let mut cm = TreeCursorMut::new(&mut mt);

    {
        assert!(c.down());
        assert!(cm.down());
        let mut c = c.take().unwrap();
        let mut cm = cm.take().unwrap();
        assert!(!c.up());
        assert!(!cm.up());
        {
            assert!(c.down());
            assert!(cm.down());
            let mut c = c.take().unwrap();
            let mut cm = cm.take().unwrap();
            assert!(!c.up());
            assert!(!cm.up());
            assert!(!c.down());
            assert!(!cm.down());
            assert!(!c.up());
            assert!(!cm.up());
        }
        {
            assert!(c.down());
            assert!(cm.down());
            let mut c = c.take().unwrap();
            let mut cm = cm.take().unwrap();
            assert!(!c.up());
            assert!(!cm.up());
            {
                assert!(c.down());
                assert!(cm.down());
                let mut c = c.take().unwrap();
                let mut cm = cm.take().unwrap();
                assert!(!c.up());
                assert!(!cm.up());
                assert!(!c.down());
                assert!(!cm.down());
                assert!(!c.up());
                assert!(!cm.up());
            }
            assert!(!c.down());
            assert!(!cm.down());
            assert!(!c.up());
            assert!(!cm.up());
            assert!(c.down());
            assert!(cm.down());
        }
        {
            assert!(c.down());
            assert!(cm.down());
            let mut c = c.take().unwrap();
            let mut cm = cm.take().unwrap();
            assert!(!c.down());
            assert!(!cm.down());
        }
        assert!(!c.down());
        assert!(!cm.down());
        assert!(!c.up());
        assert!(!cm.up());
        assert!(c.down());
        assert!(cm.down());
    }
    assert!(!c.down());
    assert!(!cm.down());
    assert!(!c.up());
    assert!(!cm.up());
    assert!(c.down());
    assert!(cm.down());
}

#[test]
fn iter_down() {
    let t = xx(vec![
        xx(vec![
            x(),
        ]),
        xx(vec![
            x(),
            x(),
        ]),
        xx(vec![
            x(),
            x(),
            x(),
        ]),
    ]);

    let mut c = TreeCursor::new(&t);

    let mut leaf_count = 0;
    while let Some(mut c) = c.down_new() {
        while let Some(_) = c.down_new() {
            leaf_count += 1;
        }
    }
    assert_eq!(leaf_count, 6);
}

#[test]
fn set_pos_error() {
    let mut t = xx(vec![
        xx(vec![
            x(),
        ]),
        x(),
        xx(vec![
            x(),
            x(),
        ]),
    ]);

    let mut cm = TreeCursorMut::new(&mut t);
    assert_eq!(cm.get().v.len(), 3);

    assert!(cm.down());
    assert_eq!(cm.get().v.len(), 1);

    assert!(cm.up());
    assert_eq!(cm.get().v.len(), 3);

    assert!(cm.down());
    assert_eq!(cm.get().v.len(), 0);

    assert!(cm.up());
    assert_eq!(cm.get().v.len(), 3);

    assert!(cm.down());
    assert_eq!(cm.get().v.len(), 2);

    let p = cm.pos();

    cm.set_pos(&p).unwrap();
    assert_eq!(cm.get().v.len(), 2);

    assert!(cm.up());
    assert_eq!(cm.get().v.len(), 3);

    // set_pos() is idempotent.
    cm.set_pos(&p).unwrap();
    cm.set_pos(&p).unwrap();
    assert_eq!(cm.get().v.len(), 2);

    assert!(cm.up());
    assert_eq!(cm.get().v.len(), 3);

    cm.get_mut().v.pop().unwrap();

    assert_eq!(cm.set_pos(&p).unwrap_err(), SetPosError::MissingNode);

    cm.get_mut().v.pop().unwrap();
    cm.get_mut().v.push(x());

    assert_eq!(cm.set_pos(&p).unwrap_err(), SetPosError::MissingNode);
}
