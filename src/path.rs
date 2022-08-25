use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Node {
    dests: HashMap<String, usize>,
}

impl Node {
    fn new() -> Node {
        Node {
            dests: HashMap::new(),
        }
    }

    fn find_shortest(&self, visited: &HashSet<String>) -> Option<(String, usize)> {
        let mut ret: Option<(String, usize)> = None;

        for (dest, len) in self.dests.iter() {
            if visited.contains(dest) {
                continue;
            }

            if let Some(r) = &ret {
                if len < &r.1 {
                    ret = Some((dest.clone(), *len))
                }
            } else {
                ret = Some((dest.clone(), *len))
            }
        }

        ret
    }

    fn find_longest(&self, visited: &HashSet<String>) -> Option<(String, usize)> {
        let mut ret: Option<(String, usize)> = None;

        for (dest, len) in self.dests.iter() {
            if visited.contains(dest) {
                continue;
            }

            if let Some(r) = &ret {
                if len > &r.1 {
                    ret = Some((dest.clone(), *len))
                }
            } else {
                ret = Some((dest.clone(), *len))
            }
        }

        ret
    }
}

fn insert_node(nodes: &mut HashMap<String, Node>, orig: &str, dest: &str, len: &str) {
    if !nodes.contains_key(orig) {
        nodes.insert(orig.to_string(), Node::new());
    }

    if let Some(node) = nodes.get_mut(orig) {
        node.dests
            .insert(dest.to_string(), len.parse::<usize>().unwrap());
    }
}

fn find_shortest_recursive(
    nodes: &HashMap<String, Node>,
    curr: &Node,
    visited: &mut HashSet<String>,
) -> usize {
    if let Some((dest, len)) = curr.find_shortest(visited) {
        visited.insert(dest.to_string());
        let curr = nodes.get_key_value(&dest).unwrap().1;
        return len + find_shortest_recursive(nodes, curr, visited);
    }
    0
}

fn find_longest_recursive(
    nodes: &HashMap<String, Node>,
    curr: &Node,
    visited: &mut HashSet<String>,
) -> usize {
    if let Some((dest, len)) = curr.find_longest(visited) {
        visited.insert(dest.to_string());
        let curr = nodes.get_key_value(&dest).unwrap().1;
        return len + find_longest_recursive(nodes, curr, visited);
    }
    0
}

pub fn find_shortest_path(input: &str) -> usize {
    let mut nodes: HashMap<String, Node> = HashMap::new();

    for line in input.lines() {
        let input: Vec<&str> = line.split(' ').collect();

        match input[..] {
            [orig, "to", dest, "=", len] => {
                insert_node(&mut nodes, orig, dest, len);
                insert_node(&mut nodes, dest, orig, len);
            }
            _ => unreachable!(),
        };
    }

    let mut min = std::usize::MAX;
    for (key, node) in nodes.iter() {
        let mut visited: HashSet<String> = HashSet::new();
        visited.insert(key.to_string());

        let curr_min = find_shortest_recursive(&nodes, node, &mut visited);

        if curr_min < min {
            min = curr_min;
        }
    }
    min
}

pub fn find_longest_path(input: &str) -> usize {
    let mut nodes: HashMap<String, Node> = HashMap::new();

    for line in input.lines() {
        let input: Vec<&str> = line.split(' ').collect();

        match input[..] {
            [orig, "to", dest, "=", len] => {
                insert_node(&mut nodes, orig, dest, len);
                insert_node(&mut nodes, dest, orig, len);
            }
            _ => unreachable!(),
        };
    }

    let mut max = std::usize::MIN;
    for (key, node) in nodes.iter() {
        let mut visited: HashSet<String> = HashSet::new();
        visited.insert(key.to_string());

        let curr_max = find_longest_recursive(&nodes, node, &mut visited);

        if curr_max > max {
            max = curr_max;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest() {
        let input = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";

        assert_eq!(605, find_shortest_path(input));
    }

    #[test]
    fn test_longest() {
        let input = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";

        assert_eq!(982, find_longest_path(input));
    }
}
