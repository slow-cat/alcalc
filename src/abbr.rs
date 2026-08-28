#[allow(unused)]
pub use std::{
    collections::{btree_map::Entry, BTreeMap, BTreeSet, VecDeque},
    io::{self, Read},
};
#[allow(unused)]
pub trait InsertUnique<V> {
    fn insert_unique(self, value: V) -> Option<()>;
}

impl<K: Ord, V> InsertUnique<V> for Entry<'_, K, V> {
    fn insert_unique(self, value: V) -> Option<()> {
        match self {
            Entry::Vacant(e) => {
                e.insert(value);
                Some(())
            }
            Entry::Occupied(_) => None,
        }
    }
}
#[allow(unused)]
pub use tree_sitter::{Node, Parser, Tree};
#[allow(unused)]
pub type DataBase = BTreeMap<usize, (usize, BTreeSet<usize>)>;
#[allow(unused)]
pub type NUMBER = i32;
#[allow(unused)]
pub fn input() -> String {
    let mut buf = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .expect("Failed to read line.");
    buf
}
