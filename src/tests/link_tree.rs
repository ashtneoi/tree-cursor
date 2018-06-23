use cursor::{TreeCursor, TreeCursorMut};
use std::collections::HashMap;

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
        let mut c2 = c.down_map_new(d).unwrap();
        let mut cm = cm.down_map_new(md).unwrap();
        assert_eq!(&c2.get().target, "bar");
        assert_eq!(&cm.get().target, "bar");
        {
            let c3 = c2.down_map_new(d).unwrap();
            let cm = cm.down_map_new(md).unwrap();
            assert_eq!(&c3.get().target, "fuzz");
            assert_eq!(&cm.get().target, "fuzz");
        }
        assert_eq!(&c2.get().target, "bar");
        assert_eq!(&cm.get().target, "bar");
        assert!(c2.down_map_new(d).is_none());
        assert!(cm.down_map_new(md).is_none());
    }
    assert_eq!(&c.get().target, "foo");
    assert_eq!(&cm.get().target, "foo");
    assert!(c.down_map_new(d).is_none());
    assert!(cm.down_map_new(md).is_none());
}
