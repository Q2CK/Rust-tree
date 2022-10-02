struct Node {
    name: String,
    children: Vec<Node>,
    depth: u16
}

impl Node {
    fn new(new_name: &str) -> Node {
        Node {
            name: String::from(new_name),
            children: Vec::new(),
            depth: 0
        }
    }
    fn add_child(&mut self, mut new_node: Node) {
        new_node.depth = self.depth + 1;
        self.children.push(new_node);
    }
    fn traverse(&mut self, function: fn(&mut Node)) {
        function(self);
        for child in &mut self.children {
            Node::traverse(child, function);
        }
    }
}

fn main() {
    let mut tree = Node::new("root");

    tree.add_child(Node::new("bajo"));

    tree.children[0].add_child(Node::new("jajo"));
    tree.children[0].children[0].add_child(Node::new("beep"));
    tree.add_child(Node::new("lmao"));
    tree.children[1].add_child(Node::new("boop"));

    tree.traverse(|x: &mut Node| {
        for _ in 0..x.depth {
            print!(".");
        }
        x.name.push_str("_test");
        print!("{}\n", x.name);
    });

    tree.traverse(|x: &mut Node| {
        for _ in 0..x.depth {
            print!(".");
        }
        x.name.push_str("_test");
        print!("{}\n", x.name);
    });
}
