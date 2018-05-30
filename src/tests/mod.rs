use cursor::{TreeCursor, TreeCursorMut};
use prelude::*;
use std::collections::HashMap;
use std::ptr;

mod hashmap_tree;

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

    let mut c = TreeCursor::new(&t);
    let mut cm = TreeCursorMut::new(&mut mt);
    assert!(ptr::eq(c.get(), &t));
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 2);

    assert!(c.down());
    assert!(cm.down());
    assert!(ptr::eq(c.get(), &t.v[0]));
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 2);

    assert!(c.down());
    assert!(cm.down());
    assert!(ptr::eq(c.get(), &t.v[0].v[0]));
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 0);

    assert!(!c.down());
    assert!(!cm.down());
    assert!(ptr::eq(c.get(), &t.v[0].v[0]));
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 0);

    assert!(c.up());
    assert!(cm.up());
    assert!(ptr::eq(c.get(), &t.v[0]));
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 2);

    assert!(c.down());
    assert!(cm.down());
    assert!(ptr::eq(c.get(), &t.v[0].v[1]));
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 0);

    assert!(!c.down());
    assert!(!cm.down());
    assert!(ptr::eq(c.get(), &t.v[0].v[1]));
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 0);

    assert!(c.up());
    assert!(cm.up());
    assert!(ptr::eq(c.get(), &t.v[0]));
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 2);

    assert!(!c.down());
    assert!(!cm.down());
    assert!(ptr::eq(c.get(), &t.v[0]));
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 2);

    assert!(c.up());
    assert!(cm.up());
    assert!(ptr::eq(c.get(), &t));
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 2);

    assert!(c.down());
    assert!(cm.down());
    assert!(ptr::eq(c.get(), &t.v[1]));
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 0);

    assert!(!c.down());
    assert!(!cm.down());
    assert!(ptr::eq(c.get(), &t.v[1]));
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 0);

    assert!(c.up());
    assert!(cm.up());
    assert!(ptr::eq(c.get(), &t));
    assert!(ptr::eq(cm.get() as *const X, cm.get_mut() as *const X));
    assert_eq!(cm.get().v.len(), 2);

    assert!(!c.up());
    assert!(!cm.up());
    assert!(ptr::eq(c.get(), &t));
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

    let mut c = TreeCursor::new(&t);

    {
        let mut c = c.down_new().unwrap();
        assert!(!c.up());
        {
            let mut c = c.down_new().unwrap();
            assert!(!c.up());
            assert!(c.down_new().is_none());
            assert!(!c.up());
        }
        {
            let mut c = c.down_new().unwrap();
            assert!(!c.up());
            {
                let mut c = c.down_new().unwrap();
                assert!(!c.up());
                assert!(c.down_new().is_none());
                assert!(!c.up());
            }
            assert!(c.down_new().is_none());
            assert!(!c.up());
            assert!(c.down());
        }
        {
            let mut c = c.down_new().unwrap();
            assert!(c.down_new().is_none());
        }
        assert!(c.down_new().is_none());
        assert!(!c.up());
        assert!(c.down());
    }
    assert!(c.down_new().is_none());
    assert!(!c.up());
    assert!(c.down());
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

#[derive(Clone)]
struct LinkNode {
    target: String,
}

fn n(target: &str) -> LinkNode {
    LinkNode { target: target.to_string() }
}

fn build_link_map<'t>(
    nn: &'t [(&str, LinkNode)],
) -> HashMap<String, &'t LinkNode> {
    let mut m = HashMap::new();
    for &(name, ref link) in nn {
        assert!(m.insert(name.to_string(), link).is_none());
    }
    m
}

fn build_mut_link_map<'t>(
    nn: &'t mut [(&str, LinkNode)],
) -> HashMap<String, *mut LinkNode> {
    let mut m = HashMap::new();
    for &mut (name, ref mut link) in nn {
        assert!(m.insert(name.to_string(), link as *mut LinkNode).is_none());
    }
    m
}

#[test]
fn link_tree() {
    let root = n("foo");
    let nn = vec![
        ("foo", n("bar")),
        ("bar", n("fuzz")),
        ("fuzz", n("bar")),
    ];
    let mut mroot = root.clone();
    let mut mnn = nn.clone();

    let lm = build_link_map(&nn);
    let mlm = build_mut_link_map(&mut mnn);

    let d = |n: &LinkNode, idx: usize| {
        if idx == 0 {
            Some(lm[&n.target])
        } else {
            None
        }
    };
    let md = |n: &mut LinkNode, idx: usize| {
        if idx == 0 {
            Some(unsafe { mlm[&n.target].as_mut().unwrap() })
        } else {
            None
        }
    };

    let mut c = TreeCursor::new(&root);
    let mut cm = TreeCursorMut::new(&mut mroot);
    assert_eq!(&c.get().target, "foo");
    assert_eq!(&cm.get().target, "foo");

    assert!(c.down_map(d));
    assert!(cm.down_map(md));
    assert_eq!(&c.get().target, "bar");
    assert_eq!(&cm.get().target, "bar");

    assert!(c.down_map(d));
    assert!(cm.down_map(md));
    assert_eq!(&c.get().target, "fuzz");
    assert_eq!(&cm.get().target, "fuzz");

    assert!(c.down_map(d));
    assert!(cm.down_map(md));
    assert_eq!(&c.get().target, "bar");
    assert_eq!(&cm.get().target, "bar");

    assert!(c.down_map(d));
    assert!(cm.down_map(md));
    assert_eq!(&c.get().target, "fuzz");
    assert_eq!(&cm.get().target, "fuzz");

    assert!(c.up());
    assert!(cm.up());
    assert_eq!(&c.get().target, "bar");
    assert_eq!(&cm.get().target, "bar");
    assert!(!c.down_map(d));
    assert!(!cm.down_map(md));
    assert_eq!(&c.get().target, "bar");
    assert_eq!(&cm.get().target, "bar");

    assert!(c.up());
    assert!(cm.up());
    assert_eq!(&c.get().target, "fuzz");
    assert_eq!(&cm.get().target, "fuzz");
    assert!(!c.down_map(d));
    assert!(!cm.down_map(md));
    assert_eq!(&c.get().target, "fuzz");
    assert_eq!(&cm.get().target, "fuzz");

    assert!(c.up());
    assert!(cm.up());
    assert_eq!(&c.get().target, "bar");
    assert_eq!(&cm.get().target, "bar");
    assert!(!c.down_map(d));
    assert!(!cm.down_map(md));
    assert_eq!(&c.get().target, "bar");
    assert_eq!(&cm.get().target, "bar");

    assert!(c.up());
    assert!(cm.up());
    assert_eq!(&c.get().target, "foo");
    assert_eq!(&cm.get().target, "foo");
    assert!(!c.down_map(d));
    assert!(!cm.down_map(md));
    assert_eq!(&c.get().target, "foo");
    assert_eq!(&cm.get().target, "foo");

    assert!(!c.up());
    assert!(!cm.up());
    assert_eq!(&c.get().target, "foo");
    assert_eq!(&cm.get().target, "foo");

    assert!(c.down_map(d));
    assert!(cm.down_map(md));
    assert_eq!(&c.get().target, "bar");
    assert_eq!(&cm.get().target, "bar");
}

#[test]
fn link_tree_scoped_down() {
    let root = n("foo");
    let nn = vec![
        ("foo", n("bar")),
        ("bar", n("fuzz")),
        ("fuzz", n("bar")),
    ];
    let mut mroot = root.clone();
    let mut mnn = nn.clone();

    let lm = build_link_map(&nn);
    let mlm = build_mut_link_map(&mut mnn);

    let d = |n: &LinkNode, idx: usize| {
        if idx == 0 {
            Some(lm[&n.target])
        } else {
            None
        }
    };
    let md = |n: &mut LinkNode, idx: usize| {
        if idx == 0 {
            Some(unsafe { mlm[&n.target].as_mut().unwrap() })
        } else {
            None
        }
    };

    let mut c = TreeCursor::new(&root);
    let mut cm = TreeCursorMut::new(&mut mroot);
    assert_eq!(&c.get().target, "foo");
    assert_eq!(&cm.get().target, "foo");
    {
        let mut c = c.down_map_new(d).unwrap();
        let mut cm = cm.down_map_new(md).unwrap();
        assert_eq!(&c.get().target, "bar");
        assert_eq!(&cm.get().target, "bar");
        {
            let c = c.down_map_new(d).unwrap();
            let cm = cm.down_map_new(md).unwrap();
            assert_eq!(&c.get().target, "fuzz");
            assert_eq!(&cm.get().target, "fuzz");
        }
        assert_eq!(&c.get().target, "bar");
        assert_eq!(&cm.get().target, "bar");
        assert!(c.down_map_new(d).is_none());
        assert!(cm.down_map_new(md).is_none());
    }
    assert_eq!(&c.get().target, "foo");
    assert_eq!(&cm.get().target, "foo");
    assert!(c.down_map_new(d).is_none());
    assert!(cm.down_map_new(md).is_none());
}
