mod abbr;
mod create_tree;
mod dep_order;
mod eval;
use abbr::*;
use create_tree::*;
use dep_order::*;
use eval::*;
#[allow(unused)]
fn main() {
    let source = input();
    let tree = create_tree(&source);
    let root = tree.root_node();
    let nodes = root.children(&mut root.walk()).collect::<Vec<Node>>();
    let order = dep_order(&nodes, &source);
    let mut look = BTreeMap::new();
    for i in order {
        let node = nodes[i];
        eprintln!("{}", &source[node.byte_range()]);
        eval(node, &source, &mut look);
    }
}
