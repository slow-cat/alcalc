#[allow(unused)]
use crate::abbr::*;
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
