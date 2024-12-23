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
    println!("There are {} triangles that contain a computer with a t", triangles_with_ts);

    let mut maximal_cliques = graph.find_maximal_cliques();
    maximal_cliques.sort_by(|v1, v2| v1.len().cmp(&v2.len()));
    let mut lan_party = maximal_cliques.last().unwrap().clone();
    lan_party.sort();
    println!("The password is {}", lan_party.join(","));
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

    // Use the BronKerbosch algorithm
    // from https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
    fn find_maximal_cliques(&self) -> Vec<Vec<&'a str>> {
        let mut res = Vec::new();
        let p = self.vertices.clone();
        let r = HashSet::new();
        let x = HashSet::new();

        self.find_maximal_cliques_recursive(&r, &p, &x, &mut res);
        res
    }

     fn find_maximal_cliques_recursive(&self, r: &HashSet<&'a str>, p: &HashSet<&'a str>, x: &HashSet<&'a str>, res: &mut Vec<Vec<&'a str>>) {
        if p.is_empty() && x.is_empty() {
            let mut v = Vec::new();
            for n in r {
                v.push(*n);
            }
            res.push(v);
        } else {
            let mut p = p.clone();
            let mut x = x.clone();
            let mut removed = HashSet::new();
            for v in p.clone() {
                if removed.contains(v) {
                    continue;
                }
                let mut new_r = r.clone();
                new_r.insert(v);
                let neighbors_of_v = self.find_neighbors(v);
                let new_p = p.intersection(&neighbors_of_v).map(|i| *i).collect();
                let new_x = p.intersection(&neighbors_of_v).map(|i| *i).collect();

                self.find_maximal_cliques_recursive(&new_r, &new_p, &new_x, res);
                p.remove(v);
                removed.insert(v);
                x.insert(v);
            }
        }
    }

    fn find_neighbors(&self, vertex: &'a str) -> HashSet<&'a str> {
        let mut res = HashSet::new();
        for (v1, v2) in &self.edges {
            if vertex == *v1 {
                res.insert(*v2);
            } else if vertex == *v2 {
                res.insert(*v1);
            }
        }

        res
    }

    fn contains_edge(&self, v1: &'a str, v2: &'a str) -> bool {
        self.edges.contains(&(v1, v1)) || self.edges.contains(&(v2, v1))
    }
}