use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1])?;

    let map = CaveMap::parse(&input);

    // for c in map.caves.values() {
    //     for e in &c.edges {
    //         println!("{}-{}", c.id, e);
    //     }
    // }

    let count = map.count_paths();
    println!("There are {} paths through the map", count);

    Ok(())
}

struct CaveMap<'a> {
    caves: HashMap<&'a str, Cave<'a>>,
}

impl<'a> CaveMap<'a> {
    fn parse(input: &'a str) -> Self {
        let mut caves: HashMap<&'a str, Cave> = HashMap::new();

        for line in input.lines() {
            if let Some((a, b)) = line.split_once("-") {
                CaveMap::add_edge(&mut caves, a, b);
                CaveMap::add_edge(&mut caves, b, a);
            } else {
                panic!("Unexpected input line: '{}'", line);
            }
        }


        Self { caves: caves.clone() }
    }

    fn add_edge(caves: &mut HashMap<&'a str, Cave<'a>>, a: &'a str, b: &'a str) {
        if let Some(a_cave) = caves.get_mut(a) {
            a_cave.edges.push(b);
        } else {
            let mut a_cave = Cave::new(a);
            a_cave.edges.push(b);
            caves.insert(a, a_cave);
        }
    }
    
    fn count_paths(self: &Self) -> u64 {
        self.count_rec(&self.caves["start"], Vec::new())
    }

    fn count_rec(self: &Self, cave: &Cave<'a>, so_far: Vec<&'a str>) -> u64 {
        if cave.is_small() && so_far.contains(&cave.id) {
            if cave.id == "start" {
                return 0;
            }
            
            let mut sorted = so_far.clone();
            sorted.sort();
            let mut last = "";
            for p in sorted {
                if p.to_lowercase() == p && p == last {
                    return 0;
                }
                last = p;
            }
        }

        let mut path = so_far.clone();
        path.push(cave.id);
        if cave.id == "end" {
            print!("Found path: ");
            for e in &path {
                print!("{}->", e);
            }
            println!();
            return 1;
        }

        let mut count = 0;
        for e in &cave.edges {
            let n = &self.caves[e];
            count += self.count_rec(n, path.clone());
        }
        count
    }
}

#[derive(Clone)]
struct Cave<'a> {
    id: &'a str,
    edges: Vec<&'a str>,
}

impl<'a> Cave<'a> {
    fn new(id: &'a str) -> Self {
        Self {
            id,
            edges: Vec::new(),
        }
    }

    fn is_small(self: &Self) -> bool {
        self.id.to_lowercase() == self.id
    }
}

impl<'a> Eq for Cave<'a> {}

impl<'a> PartialEq for Cave<'a> {
    fn eq(self: &Self, rhs: &Self) -> bool {
        self.id == rhs.id
    }
}

impl<'a> std::hash::Hash for Cave<'a> {
    fn hash<H: core::hash::Hasher>(self: &Self, state: &mut H) {
        self.id.hash(state);
    }
}
