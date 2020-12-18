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
            for x in self.minx..self.maxx+1 {
                let res = match self.is_active(x, y) {
                    true => "#",
                    false => ".",
                };
                write!(f, "{}", res)?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

pub struct Cube {
    z_planes: HashMap<isize, Plane>,
    minx: isize,
    miny: isize,
    minz: isize,
    maxx: isize,
    maxy: isize,
    maxz: isize,
}

impl Cube {
    pub fn parse(input: &str) -> Self {
        let mut zmap = HashMap::new();
        let p = Plane::parse(input);
        let minx = p.minx;
        let miny = p.miny;
        let maxx = p.maxx;
        let maxy = p.maxy;
        zmap.insert(0, p);
        Cube {
            z_planes: zmap,
            minx: minx,
            miny: miny,
            minz: 0,
            maxx: maxx,
            maxy: maxy,
            maxz: 0,
        }
    }

    pub fn create_from(zmap: HashMap<isize, Plane>) -> Self {
        let minx = zmap.iter().map(|(_, p)| p.minx).min().unwrap();
        let miny = zmap.iter().map(|(_, p)| p.miny).min().unwrap();
        let minz = *zmap.iter().map(|(z, _)| z).min().unwrap();
        let maxx = zmap.iter().map(|(_, p)| p.maxx).max().unwrap();
        let maxy = zmap.iter().map(|(_, p)| p.maxy).max().unwrap();
        let maxz = *zmap.iter().map(|(z, _)| z).max().unwrap();
        Cube {
            z_planes: zmap,
            minx: minx,
            miny: miny,
            minz: minz,
            maxx: maxx,
            maxy: maxy,
            maxz: maxz,
        }
    }

    pub fn active_count(&self) -> usize {
        self.z_planes.iter().map(|(_, p)| p.active_count()).sum()
    }

    pub fn cycle3d(&self) -> Self {
        let mut zmap = HashMap::new();
        for z in self.minz-1..self.maxz + 2 {
            let mut ymap = HashMap::new();
            for y in self.miny-1..self.maxy + 2 {
                let mut xs = HashSet::new();
                for x in self.minx..self.maxx + 2 {
                    let adj = self.active_adjacent3d(x, y, z);
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

        Cube{
            z_planes: zmap,
            minx: self.minx-1,
            miny: self.miny-1,
            minz: self.minz-1,
            maxx: self.maxx+1,
            maxy: self.maxy+1,
            maxz: self.maxz+1,
        }
    }
    
    fn active_adjacent3d(&self, x: isize, y: isize, z: isize) -> usize {
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

impl std::fmt::Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for z in self.minz..self.maxz+1 {
            writeln!(f, "z={}", z)?;
            writeln!(f, "{}", self.z_planes[&z])?;
        }
        Ok(())
    }
}

struct Dimension {
    cubes: HashMap<isize, Cube>,
    minx: isize,
    miny: isize,
    minz: isize,
    minw: isize,
    maxx: isize,
    maxy: isize,
    maxz: isize,
    maxw: isize,
}

impl Dimension {
    pub fn parse(input: &str) -> Self {
        let mut cubes = HashMap::new();
        let c = Cube::parse(input);
        let minx = c.minx;
        let miny = c.miny;
        let minz = c.minz;
        let maxx = c.maxx;
        let maxy = c.maxy;
        let maxz = c.maxz;

        cubes.insert(0, c);
        Self {
            cubes: cubes,
            minx: minx,
            miny: miny,
            minz: minz,
            minw: 0,
            maxx: maxx,
            maxy: maxy,
            maxz: maxz,
            maxw: 0,
        }
    }

    pub fn active_count(&self) -> usize {
        self.cubes.iter().map(|(_, c)| c.active_count()).sum()
    }

    pub fn cycle(&self) -> Self {
        let mut cubes = HashMap::new();
        for w in self.minw-1..self.maxw + 2 {
            let mut zmap = HashMap::new();
            for z in self.minz-1..self.maxz + 2 {
                let mut ymap = HashMap::new();
                for y in self.miny-1..self.maxy + 2 {
                    let mut xs = HashSet::new();
                    for x in self.minx..self.maxx + 2 {
                        let adj = self.active_adjacent(x, y, z, w);
                        let res = match self.is_active(x, y, z, w) {
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
            cubes.insert(w, Cube::create_from(zmap));
        }

        Self {
            cubes: cubes,
            minx: self.minx-1,
            miny: self.miny-1,
            minz: self.minz-1,
            minw: self.minw-1,
            maxx: self.maxx+1,
            maxy: self.maxy+1,
            maxz: self.maxz+1,
            maxw: self.maxw+1,
        }
    }

    fn active_adjacent(&self, x: isize, y: isize, z: isize, w: isize) -> usize {
        let mut count = 0;
        for ww in w-1..w+2 {
            for zz in z-1..z+2 {
                for yy in y-1..y+2 {
                    for xx in x-1..x+2 {
                        if self.is_active(xx, yy, zz, ww) {
                            count += 1;
                        }
                    }
                }
            }
        }

        if self.is_active(x, y, z, w) {
            count -= 1;
        }

        count
    }

    fn is_active(&self, x: isize, y: isize, z: isize, w: isize) -> bool {
        match self.cubes.get(&w) {
            Some(c) => c.is_active(x, y, z),
            None => false,
        }
    }
}

impl std::fmt::Display for Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for w in self.minw..self.maxw+1 {
            writeln!(f, "w = {}, {}", w, self.cubes[&w])?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests_part1 {
    use super::*;

    #[test]
    fn test() {
        let mut state = Cube::parse(
".#.
..#
###",
        );

        for i in 0..6 {
            state = state.cycle3d();
            println!("------- After step {} -----------", i);
            println!("{}", state);
        }

        assert_eq!(112, state.active_count())
    }
}

#[cfg(test)]
mod tests_part2 {
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
            println!("Bounds: x:{}-{}, y:{}-{}, z:{}-{}, w:{}-{}", state.minx, state.maxx, state.miny, state.maxy, state.minz, state.maxz, state.minw, state.maxw);
            println!("{}", state);
        }

        assert_eq!(848, state.active_count())
    }
}
