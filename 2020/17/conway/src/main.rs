use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let mut state = Dimension::parse(
"#.#####.
#..##...
.##..#..
#.##.###
.#.#.#..
#.##..#.
#####..#
..#.#.##",
    );

    for i in 0..6 {
        state = state.cycle();
        println!("------- After step {} -----------", i);
        println!("{}", state);
    }

    println!("After boot, there are {} cycles", state.active_count());
}

#[derive(Clone)]
pub struct Plane {
    y_points: HashMap<isize, HashSet<isize>>,
    minx: isize,
    miny: isize,
    maxx: isize,
    maxy: isize,
}

impl Plane {
    pub fn parse(input: &str) -> Self {
        let mut map = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            let mut hashset = HashSet::new();
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    hashset.insert(x as isize);
                }
            }
            map.insert(y as isize, hashset);
        }
        let (minx, miny, maxx, maxy) = Self::get_mins_maxes(&map);

        Self {
            y_points: map,
            minx: minx,
            miny: miny,
            maxx: maxx,
            maxy: maxy,
        }
    }

    pub fn create_from(map: HashMap<isize, HashSet<isize>>) -> Self {
        let (minx, miny, maxx, maxy) = Self::get_mins_maxes(&map);
        Self {
            y_points: map,
            minx: minx,
            miny: miny,
            maxx: maxx,
            maxy: maxy,
        }
    }

    fn get_mins_maxes(map: &HashMap<isize, HashSet<isize>>) -> (isize, isize, isize, isize) {
        let mut minx = isize::MAX;
        let mut miny = isize::MAX;
        let mut maxx = isize::MIN;
        let mut maxy = isize::MIN;
        for (y, xs) in map {
            if *y > maxy { maxy = *y; }
            if *y < miny { miny = *y; }

            for x in xs {
                if *x > maxx { maxx = *x; }
                if *x < minx { minx = *x; }
            }
        }

        (minx, miny, maxx, maxy)
    }

    pub fn active_count(&self) -> usize {
        self.y_points.iter().map(|(_, xs)| xs.len()).sum()
    }

    pub fn is_active(&self, x: isize, y: isize) -> bool {
        match self.y_points.get(&y) {
            Some(xs) => match xs.get(&x) {
                Some(_) => true,
                None => false,
            },
            None => false,
        }
    }
}

impl std::fmt::Display for Plane {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for y in self.miny..self.maxy+1 {
            if let Some(xs) = self.y_points.get(&y) {
                for x in self.minx..self.maxx+1 {
                    let c = match xs.get(&x) {
                        Some(_) => "#",
                        None => ".",
                    };
                    write!(f, "{}", c)?;
                }

            } else {
                for _ in self.minx..self.maxx+1 {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

pub struct Dimension {
    z_planes: HashMap<isize, Plane>,
    minx: isize,
    miny: isize,
    minz: isize,
    maxx: isize,
    maxy: isize,
    maxz: isize,
}

impl Dimension {
    pub fn parse(input: &str) -> Self {
        let mut zmap = HashMap::new();
        let p = Plane::parse(input);
        let minx = p.minx;
        let miny = p.miny;
        let maxx = p.maxx;
        let maxy = p.maxy;
        zmap.insert(0, p);
        Self {
            z_planes: zmap,
            minx: minx,
            miny: miny,
            minz: 0,
            maxx: maxx,
            maxy: maxy,
            maxz: 0,
        }
    }

    pub fn active_count(&self) -> usize {
        self.z_planes.iter().map(|(_, p)| p.active_count()).sum()
    }

    pub fn cycle(&self) -> Self {
        let mut zmap = HashMap::new();
        for z in self.minz-1..self.maxz + 2 {
            let mut ymap = HashMap::new();
            for y in self.miny-1..self.maxy + 2 {
                let mut xs = HashSet::new();
                for x in self.minx..self.maxx + 2 {
                    let adj = self.active_adjacent(x, y, z);
                    let res = match self.is_active(x, y, z) {
                        true => adj == 2 || adj == 3,
                        false => adj == 3,
                    };

                    if res { 
                        xs.insert(x);
                    }
                }
                ymap.insert(y, xs);
            }
            zmap.insert(z, Plane::create_from(ymap));
        }

        Self{
            z_planes: zmap,
            minx: self.minx-1,
            miny: self.miny-1,
            minz: self.minz-1,
            maxx: self.maxx+1,
            maxy: self.maxy+1,
            maxz: self.maxz+1,
        }
    }
    
    fn active_adjacent(&self, x: isize, y: isize, z: isize) -> usize {
        let mut count = 0;
        for zz in z-1..z+2 {
            for yy in y-1..y+2 {
                for xx in x-1..x+2 {
                    if self.is_active(xx, yy, zz) {
                        count += 1;
                    }
                }
            }
        }

        if self.is_active(x, y, z) {
            count -= 1;
        }
        count
    }

    fn is_active(&self, x: isize, y: isize, z: isize) -> bool {
        match self.z_planes.get(&z) {
            Some(p) => p.is_active(x, y),
            None => false
        }
    }
}

impl std::fmt::Display for Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for (z, plane) in &self.z_planes {
            writeln!(f, "z={}", z)?;
            write!(f, "{}", plane)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests_part1 {
    use super::*;

    #[test]
    fn test() {
        let mut state = Dimension::parse(
".#.
..#
###",
        );

        for i in 0..6 {
            state = state.cycle();
            println!("------- After step {} -----------", i);
            println!("{}", state);
        }

        assert_eq!(112, state.active_count())
    }
}
