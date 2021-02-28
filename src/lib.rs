pub mod vec;

pub use vec::AssocListExt;

#[test]
fn test_entry() {
    use crate::AssocListExt;

    let mut v = vec![("a", 1), ("b", 2)];
    v.entry("c").or_insert(3);
    assert_eq!(v, vec![("a", 1), ("b", 2), ("c", 3)]);
    assert_eq!(v.entry("c").or_insert(4), &3);
}
