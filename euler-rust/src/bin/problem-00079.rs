use euler_rust::utils::files;
use euler_rust::utils::parse;
use std::collections::HashSet;

fn main() {
    let passcode = shortest_passcode_file("resources/0079_keylog.txt");
    println!("{}", passcode);
}

fn shortest_passcode_file(file_path: &str) -> String {
    shortest_passcode(files::read_file(file_path).as_str())
}

// The login attempts describe directed paths between the digits. The
// rest of the question reduces to finding a topological sort of this
// (hopefully) directed acyclic graph. We implement Kahn's algorithm.
fn shortest_passcode(text: &str) -> String {
    // To keep track of incoming edges, so that we can find candidate
    // vertices to insert into our topological sort order.
    let mut indegrees = vec![0u8; 10];
    // Adjacency list.
    let mut edges: Vec<HashSet<u8>> = vec![HashSet::new(); 10];
    // A set of all vertices in the graph (needed since we're modeling
    // the previous data structures as vectors and not maps).
    let mut vertices: HashSet<u8> = HashSet::new();
    build_graph(text, &mut edges, &mut indegrees, &mut vertices);
    // Initial set of vertices with zero indegrees. Will be updated as
    // we start removing edges from the graph.
    let mut no_incoming: HashSet<u8> = indegrees.iter().enumerate().filter(|(_, &d)| d == 0).map(|(v, _)| v as u8).collect();
    let mut top_sort: Vec<u8> = Vec::new();
    while !no_incoming.is_empty() {
	let u = no_incoming.iter().next().unwrap().clone();
	no_incoming.remove(&u);
	if !vertices.contains(&u) { continue; }
	top_sort.push(u);
	for &v in edges[u as usize].iter() {
	    indegrees[v as usize] -= 1;
	    if indegrees[v as usize] == 0 {
		no_incoming.insert(v);
	    }
	}
    }
    parse::parse_slice_as_string(top_sort.as_slice())
}

fn build_graph(text: &str, edges: &mut Vec<HashSet<u8>>, indegrees: &mut Vec<u8>, vertices: &mut HashSet<u8>) {
    for line in text.lines() {
	let path: Vec<u8> = line.chars().into_iter().map(|c| (c as u8) - 48).collect();
	for (&u, &v) in path.iter().zip(path.iter().skip(1)) {
	    vertices.insert(u);
	    vertices.insert(v);
	    // This check is necessary because some edges appear
	    // multiple times in our text, and we only need to
	    // increase the indegree for a vertex once per incoming
	    // edge.
	    if edges[u as usize].insert(v) {
		indegrees[v as usize] += 1;
	    }
	}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_passcode() {
	let text: &str = "\
	317\n\
	578\n\
	532\n\
	127\
	";
	assert_eq!(shortest_passcode(text).as_str(), "531278");
    }
}
