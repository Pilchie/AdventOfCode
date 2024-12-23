use std::{collections::HashSet, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let graph = Graph::parse(&contents);
    let triangles = graph.find_triangles();

    let triangles_with_ts = triangles
        .iter()
        .filter(|c|c.iter().any(|v| v.starts_with('t')))
        .count();
    println!("There are {} connected components that contain a computer with a t", triangles_with_ts);
}

struct Graph<'a> {
    vertices: HashSet<&'a str>,
    edges: HashSet<(&'a str, &'a str)>,
}

impl<'a> Graph<'a> {
    fn parse(input: &'a str) -> Self {
        let mut vertices = HashSet::new();
        let mut edges = HashSet::new();
        for line in input.lines() {
            let (left, right) = line.split_once("-").unwrap();
            vertices.insert(left);
            vertices.insert(right);
            if left < right {
                edges.insert((left, right));
            } else {
                edges.insert((right, left));
            }
        }
        Self { vertices, edges }
    }

    fn find_triangles(&self) -> Vec<Vec<&'a str>> {
        let mut res = Vec::new();

        for (u, v) in &self.edges {
            for w in &self.vertices {
                if self.contains_edge(v, w) && self.contains_edge(w, u) {
                    res.push(vec![*u, *v, *w]);
                }
            }
        }
        res
    }

    fn contains_edge(&self, v1: &'a str, v2: &'a str) -> bool {
        self.edges.contains(&(v1, v1)) || self.edges.contains(&(v2, v1))
    }
}