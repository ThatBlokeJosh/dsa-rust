mod bst;

fn main() {
    let mut t = bst::Node::new(10);
    t.insert(9);
    t.insert(11);
    t.insert(13);
    t.insert(12);
    t.all()
}
