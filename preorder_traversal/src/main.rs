struct Node {
    children: Vec<Node>,
    name: Option<String>,
}

fn preorder_traversal(node: &Node) {
    if let Some(name) = &node.name {
        print!("{} ", name);
    }
    for child in &node.children {
        preorder_traversal(child);
    }
}

fn parse_newick(newick: &str) -> Node {
    let mut stack = Vec::new();
    let mut current = Node {
        children: Vec::new(),
        name: None,
    };

    for c in newick.chars() {
        match c {
            '(' => {
                stack.push(current);
                current = Node {
                    children: Vec::new(),
                    name: None,
                };
            }
            ')' => {
                let parent = stack.pop().unwrap();
                parent.children.push(current);
                current = parent;
            }
            ',' => {
                let mut parent = stack.pop().unwrap();
                parent.children.push(current);
                stack.push(parent);
                current = Node {
                    children: Vec::new(),
                    name: None,
                };
            }
            ':' => {
                let mut name = String::new();
                while let Some(c) = newick.chars().next() {
                    if c == ',' || c == ')' {
                        break;
                    }
                    name.push(c);
                    newick.remove(0);
                }
                current.name = Some(name);
            }
            _ => {}
        }
    }

    current
}

fn main() {
    let newick = "((A:1,B:2):3,(C:4,D:5):6):7;";
    let root = parse_newick(newick);
    preorder_traversal(&root);
}

