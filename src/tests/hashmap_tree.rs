use cursor::{TreeCursor, TreeCursorMut};
use std::collections::HashMap;

#[derive(Clone)]
struct HashMapNode {
    x: usize,
    m: HashMap<String, HashMapNode>,
}

fn n(x: usize) -> HashMapNode {
    nn(x, vec![])
}

fn nn(x: usize, children: Vec<(&str, HashMapNode)>) -> HashMapNode {
    let mut h = HashMapNode { x, m: HashMap::new() };
    for (name, child) in children {
        h.m.insert(name.to_string(), child);
    }
    h
}

#[test]
fn down_map() {
    let t = nn(0, vec![
        ("foo", n(1)),
        ("bar", nn(2, vec![
            ("rue", n(3)),
        ])),
    ]);
    let mut mt = t.clone();

    let mut c = TreeCursor::new(&t);
    let mut cm = TreeCursorMut::new(&mut mt);
    assert_eq!(c.get().x, 0);
    assert_eq!(cm.get().x, 0);

    assert!(c.down_with(|n, _| n.m.get("foo")));
    assert!(cm.down_with(|n, _| n.m.get_mut("foo")));
    assert_eq!(c.get().x, 1);
    assert_eq!(cm.get().x, 1);

    assert!(c.up());
    assert!(cm.up());
    assert_eq!(c.get().x, 0);
    assert_eq!(cm.get().x, 0);

    assert!(c.down_with(|n, _| n.m.get("bar")));
    assert!(cm.down_with(|n, _| n.m.get_mut("bar")));
    assert_eq!(c.get().x, 2);
    assert_eq!(cm.get().x, 2);

    assert!(c.down_with(|n, _| n.m.get("rue")));
    assert!(cm.down_with(|n, _| n.m.get_mut("rue")));
    assert_eq!(c.get().x, 3);
    assert_eq!(cm.get().x, 3);

    assert!(c.up());
    assert!(cm.up());
    assert_eq!(c.get().x, 2);
    assert_eq!(cm.get().x, 2);

    assert!(c.up());
    assert!(cm.up());
    assert_eq!(c.get().x, 0);
    assert_eq!(cm.get().x, 0);

    assert!(!c.up());
    assert!(!cm.up());
    assert_eq!(c.get().x, 0);
    assert_eq!(cm.get().x, 0);

    assert!(c.down_with(|n, _| n.m.get("foo")));
    assert!(cm.down_with(|n, _| n.m.get_mut("foo")));
    assert_eq!(c.get().x, 1);
    assert_eq!(cm.get().x, 1);
}
