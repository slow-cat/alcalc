use crate::abbr::*;
#[allow(unused)]
use crate::create_tree::create_tree;
#[allow(unused)]
fn create_db(nodes: &Vec<Node>, var2line: &BTreeMap<String, usize>, source: &str) -> DataBase {
    let mut db: DataBase = BTreeMap::new();
    for (i, &node) in nodes.iter().enumerate() {
        let mut dep: BTreeSet<usize> = BTreeSet::new();
        let mut indep: BTreeSet<usize> = BTreeSet::new();
        match node.kind() {
            "EXPR" => {
                db.entry(i).or_default().0 += count_id(node, &mut dep, var2line, source);
            }
            "LET" => {
                //a
                db.entry(i).or_default().0 += count_id(
                    node.child_by_field_name("value").unwrap(),
                    &mut dep,
                    var2line,
                    source,
                );
            }
            "DEF" => {
                let _ = count_id(
                    node.child_by_field_name("args").unwrap(),
                    &mut indep,
                    var2line,
                    source,
                );
                let _ = count_id(
                    node.child_by_field_name("body").unwrap(),
                    &mut dep,
                    var2line,
                    source,
                );
                dep = dep.difference(&indep).cloned().collect();
                db.entry(i).or_default().0 += dep.len();
            }
            _ => {
                unreachable!()
            }
        }
        for d in dep {
            db.entry(d).or_default().1.insert(i);
        }
        eprintln!(
            "{} {:?}",
            node.kind(),
            node.children(&mut node.walk())
                .map(|n| n.kind())
                .collect::<Vec<_>>()
        );
    }
    db
}
fn count_id(
    node: Node,
    dep: &mut BTreeSet<usize>,
    var2line: &BTreeMap<String, usize>,
    source: &str,
) -> usize {
    let mut cur = node.walk();
    let mut count = 0usize;
    if node.kind() == "id" {
        let s = &source[node.byte_range()];
        if var2line.contains_key(s) && !dep.contains(&var2line.get(s).unwrap()) {
            count += 1;
            dep.insert(var2line[&source[node.byte_range()]]);
        }
    }
    for chi in node.children(&mut cur) {
        count += count_id(chi, dep, var2line, source);
    }
    count
}

#[allow(unused)]
pub fn dep_order(nodes: &Vec<Node>, src: &str) -> Vec<usize> {
    /*文の列をいれると依存換気から処理順を出力する。インクリメンタルとかはしない*/
    let var2line = nodes
        .iter()
        .enumerate()
        .filter_map(|(i, node)| {
            node.child_by_field_name("name")
                .map(|name| (i, src[name.byte_range()].to_string()))
        })
        .fold(BTreeMap::new(), |mut h, (i, e)| {
            h.entry(e).or_insert(i);
            h
        });
    let mut db = create_db(&nodes, &var2line, src);
    /*列->(依存数,影響与える先) */
    let mut order = Vec::new();
    let mut queue = VecDeque::new();
    for row in 0..nodes.len() {
        if db[&row].0 == 0 {
            queue.push_back(row);
        }
    }
    while let Some(row) = queue.pop_front() {
        order.push(row);
        let targets = db.remove(&row).unwrap().1;
        for t in targets.iter() {
            let p = &mut db.get_mut(t).unwrap().0;
            *p -= 1;
            if *p == 0 {
                queue.push_back(*t);
            }
        }
    }
    order
}
#[test]
fn show_order() {
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
    let orders = dep_order(&nodes, source);
    println!("{source}");
    println!("{orders:?}");
}
