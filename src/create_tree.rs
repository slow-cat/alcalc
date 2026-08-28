#[allow(unused)]
use crate::abbr::*;
#[allow(unused)]
use std::fs::File;
#[allow(unused)]
use std::fs::read_to_string;
use std::io::Write;
#[allow(unused)]
pub fn create_tree(src: &str) -> Tree {
    let mut parser = {
        let mut parser = Parser::new();
        let language = tree_sitter_alg::LANGUAGE;
        parser
            .set_language(&language.into())
            .expect("Error loading Alg parser");
        parser
    };
    let tree = parser.parse(src, None).unwrap();
    tree
}
#[test]
fn show_node() {
    let source = r#"
        1+1;
        let x = 10;
        let y=x+2;
        fn add(a,b)=a+b;
        add(y,3);
        x;"#;
    let tree = create_tree(source);
    let root = tree.root_node();
    let nodes = root.children(&mut root.walk()).collect::<Vec<Node>>();
    for &node in &nodes {
        println!(
            "{} {:?}",
            node.kind(),
            node.children(&mut node.walk())
                .map(|e| e.kind())
                .collect::<Vec<_>>()
        )
    }
    println!(
        "{} {:?}",
        nodes[0].child(0).unwrap().child(0).unwrap(),
        nodes[0]
            .child(0)
            .unwrap()
            .child(0)
            .unwrap()
            .children(&mut nodes[0].child(0).unwrap().child(0).unwrap().walk())
            .collect::<Vec<_>>()
    );
    println!("{}", root.to_sexp());
}
#[allow(unused)]
fn write_namedot(node: Node, out: &mut String, next_id: &mut usize, src: &str) -> usize {
    let id = *next_id;
    *next_id += 1;
    out.push_str(&format!("tree_{id} [label=\"{}\"];\n", node.kind()));
    if node.child_count() == 0 {
        let child_id = *next_id;
        out.push_str(&format!(
            "tree_{child_id} [shape=box, label=\"{}\"];\n",
            &src[node.byte_range()]
        ));
        out.push_str(&format!("tree_{id} -> tree_{child_id};\n"));
        *next_id += 1;
    } else {
        for child_node in node.children(&mut node.walk()) {
            let child_id = write_namedot(child_node, out, next_id, src);
            out.push_str(&format!("tree_{id} -> tree_{child_id};\n"));
        }
    }
    id
}
#[allow(unused)]
fn namedot(node: Node, file: &'_ str, src: &str) {
    let mut dot = String::from("digraph tree {\n");
    let mut next_id = 0;
    write_namedot(node, &mut dot, &mut next_id, src);
    dot.push_str("}\n");
    File::create(file)
        .unwrap()
        .write_all(dot.as_bytes())
        .unwrap();
}
#[test]
fn create_dot_file() {
    let source = read_to_string("./test.calc").unwrap();
    let tree = create_tree(&source);
    let f = File::create("./test.dot").unwrap();
    tree.print_dot_graph(&f);
    namedot(tree.root_node(), "./test2.dot", &source);
}
