# assoc

[<img alt="docs.rs" src="https://docs.rs/assoc/badge.svg">](https://docs.rs/assoc)

Treat vectors like association lists.

## Examples

```rust
use assoc::AssocListExt;

let mut map = vec![("a", 1), ("b", 2)];
map.entry("c").or_insert(3);
assert_eq!(map.get(&"c"), Some(&3));
assert_eq!(map.entry("c").or_insert(4), &3);
```
