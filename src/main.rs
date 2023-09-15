use std::collections::HashMap;
use std::collections::LinkedList;

fn main() {
    // create graph: adjcency list
    /*
    0: 1,2
    1:
    2: 3
    3:
    */
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let path: LinkedList<String> = LinkedList::new();

    graph.insert("0".to_string(), vec!["1".to_string(), "2".to_string()]);
    graph.insert("1".to_string(), vec![]);
    graph.insert("2".to_string(), vec!["3".to_string()]);
    graph.insert("3".to_string(), vec![]);

    // search graph to find if node a is reachable by node b
    //get start value
    let current = "0".to_string();
    let destination = "3".to_string();
    println!("starting point: {}", current);

    // visit the first node,
    // graph: is borrowed, dont want to return it to access it again
    // current: is borrowed, dont want to return it to access it again
    // path: is borrowed and mutable, should change, we are returning it
    // destination: is borrowed, dont want to return it to access it again
    let solution = visit(&graph, &current, path, &destination);

    println!("the solution is {:?}", solution)
}

// graph: is borrowed, dont want to return it to access it again
// current: is borrowed, dont want to return it to access it again
// path: is borrowed and mutable, should change, we are returning it
// destination: is borrowed, dont want to return it to access it again
// 'a lifetime of the return type is the same as the graph itself
fn visit<'a>(
    graph: &'a HashMap<String, Vec<String>>,
    node: &String,
    mut path: LinkedList<String>,
    destination: &String,
) -> LinkedList<String> {
    // borrowed pointer to the list of neighbors
    let neighbors: &Vec<String> = graph.get(node).unwrap();

    // clone the current node and add it to the path, path now owns it own copy
    path.push_back(node.clone());
    println!("path {:?}", path);

    // println!("added {}, to path to make: {:?}", node, path);

    //
    for neighbor in neighbors {
        path = visit(&graph, &neighbor, path, &destination);

        // println!("path when iterating over neighbors: {:?}", path);

        if path.len() > 0 {
            // println!("the last node in the path is {:?}: we are looking for {} <{}>", path.back().unwrap(), destination, path.back().unwrap() == destination);
            if path.back().unwrap() == destination {
                // println!("path contains destination: {:?}", path);
                return path;
            }
        }
    }
    // println!("destination:{} was not found in sub-tree, removing this branch: {:?}", destination, path);

    if path.back().unwrap() == destination {
        println!("path contains destination: {:?}", path);
        return path;
    }

    path.pop_back();
    return path;
}
