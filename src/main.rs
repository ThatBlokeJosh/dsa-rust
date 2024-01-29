mod llist;
mod bst;
use llist::List;
use bst::Node;

fn main() {
    // Binary Search Tree
    let mut t = Node::new(10);
    t.insert(55); // Inserts value into BST
    t.search(10); // Returns true if the value is in the BST
    t.all(); // Prints all values

    // Linked List
    let mut l: List<i32> = List::new();
    l.push(23); // Push value to the end
    l.push(56);
    l.pop(); // Remove last value
    l.print(); // Print all values
}
