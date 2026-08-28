use crate::abbr::*;
type Function<'a> = (Node<'a>, Vec<String>);
#[derive(Clone)]
pub enum Operand<'a> {
    Fun(Function<'a>),
    Num(NUMBER),
}
impl<'a> Operand<'a> {
    fn fun(&self) -> &Function<'a> {
        match self {
            Self::Fun(x) => x,
            _ => panic!("expected function"),
        }
    }
    fn num(&self) -> NUMBER {
        match self {
            Self::Num(x) => *x,
            _ => panic!("expected NUMBER"),
        }
    }
}
pub fn eval<'a>(node: Node<'a>, src: &str, look: &mut BTreeMap<String, Operand<'a>>) {
    match node.kind() {
        "EXPR" => expr_state(node, src, look),
        "LET" => let_state(node, src, look),
        "DEF" => def_statement(node, src, look),
        _ => unreachable!(),
    }
}
#[allow(unused)]
fn expr_state(node: Node, src: &str, look: &BTreeMap<String, Operand>) {
    let ex = node
        // .child_by_field_name("expr")
        .child(0)
        .expect(&format!("cannot find expr field:{}", node.to_sexp()).to_string());
    println!("{}", expr(ex, src, look));
}
#[allow(unused)]
fn expr(node: Node, src: &str, look: &BTreeMap<String, Operand>) -> NUMBER {
    match node.kind() {
        "num" => return src[node.byte_range()].parse::<NUMBER>().expect("not usize"),
        "id" => {
            if let Some(value) = look.get(&src[node.byte_range()]) {
                return value.num();
            } else {
                panic!("no id found")
            }
        }
        "call" => call_fn(node, src, look),
        "parenthesized" => return expr(node.child(0).unwrap(), src, look),
        "binary" => {
            let left_node = node.child(0).unwrap();
            let right_node = node.child(2).unwrap();
            let left = expr(left_node, src, look);
            let right = expr(right_node, src, look);
            let operator = node.child(1).unwrap().kind();
            match operator {
                "*" => return left * right,
                "/" => return left / right,
                "+" => return left + right,
                "-" => return left - right,
                _ => unreachable!(),
            }
        }
        "unary" => {
            let operator = node.child(0).unwrap().kind();
            let operand = expr(node.child(1).unwrap(), src, look);
            if operator == "-" {
                return (0 as NUMBER).saturating_sub(operand);
            } else {
                return operand;
            }
        }
        "expr" => {
            let ch = node.child(0).unwrap_or_else(|| {
                panic!(
                    "cannot find child_0:{} {}",
                    &src[node.byte_range()],
                    node.to_sexp()
                )
            });
            return expr(ch, src, look);
        }
        e => unreachable!("{e} [{}]", &src[node.byte_range()]),
    }
}
#[allow(unused)]
fn def_statement<'a>(node: Node<'a>, src: &str, look: &mut BTreeMap<String, Operand<'a>>) {
    let name = node
        .child_by_field_name("name")
        .map(|name_node| src[name_node.byte_range()].to_string())
        .expect("can't find name field");
    let args = node.child_by_field_name("args").expect("no args");
    let parameters = args
        .children_by_field_name("parameter", &mut args.walk())
        .map(|chnode| src[chnode.byte_range()].to_string())
        .collect();
    look.entry(name.clone())
        .insert_unique(Operand::Fun((
            node.child_by_field_name("body").expect("not body"),
            parameters,
        )))
        .unwrap_or_else(|| panic!("{} already exists", name));
}
#[allow(unused)]
fn call_fn(node: Node, src: &str, look: &BTreeMap<String, Operand>) -> NUMBER {
    // id args
    // look args 分離の必要がない　直接結合で問題ない
    let fun = {
        let funode = node
            .child_by_field_name("called")
            .expect("not found function in this node");
        look.get(&src[funode.byte_range()])
            .map(|val| val.fun())
            .unwrap()
    };
    let stack = {
        let args = node.child_by_field_name("args").unwrap();
        args.children_by_field_name("parameter", &mut args.walk())
            .map(|par| expr(par, src, look))
            .collect::<Vec<_>>()
    };
    let mut look_args = look.clone();
    for (i, k) in fun.1.iter().enumerate() {
        look_args
            .entry(k.clone())
            .insert_entry(Operand::Num(stack[i]));
    }
    expr(fun.0, src, &look_args)
}
#[allow(unused)]
fn let_state(node: Node, src: &str, look: &mut BTreeMap<String, Operand>) {
    let name = node
        .child_by_field_name("name")
        .map(|name_node| src[name_node.byte_range()].to_string())
        .expect("can't find name field");
    let value = expr(
        node.child_by_field_name("value")
            .expect("can't find value field"),
        src,
        look,
    );
    look.entry(name.clone())
        .insert_unique(Operand::Num(value))
        .unwrap_or_else(|| panic!("{} already exists", name));
}
