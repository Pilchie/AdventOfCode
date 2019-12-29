extern crate num_integer;

use num_integer::lcm;
use std::ops::Add;

#[derive(Debug)]
pub struct Vector3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Clone for Vector3D {
    fn clone(&self) -> Vector3D {
        Vector3D { ..*self }
    }
}

impl Copy for Vector3D {}

impl Add for Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl PartialEq for Vector3D {
    fn eq(&self, rhs: &Vector3D) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == rhs.z
    }
}

impl Vector3D {
    fn sum_of_abs_values(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(Debug)]
pub struct Moon {
    pub position: Vector3D,
    pub velocity: Vector3D,
}

impl PartialEq for Moon {
    fn eq(&self, rhs: &Moon) -> bool {
        self.position == rhs.position && self.velocity == rhs.velocity
    }
}

impl Clone for Moon {
    fn clone(&self) -> Moon {
        Moon { ..*self }
    }
}

impl Moon {
    pub fn new(x: i32, y: i32, z: i32) -> Moon {
        Moon {
            position: Vector3D { x, y, z },
            velocity: Vector3D { x: 0, y: 0, z: 0 },
        }
    }

    fn update_position(&mut self) {
        self.position = self.position + self.velocity;
    }

    pub fn potential_energy(&self) -> i32 {
        self.position.sum_of_abs_values()
    }

    pub fn kinetic_energy(&self) -> i32 {
        self.velocity.sum_of_abs_values()
    }

    pub fn total_energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }
}

pub fn step_other(moons: &mut [Moon]) {
    let len = moons.len();
    for i1 in 0..len {
        for i2 in (i1 + 1)..len {
            let m1 = moons[i1].clone();
            let m2 = moons[i2].clone();
            let (m1u, m2u) = update_velocity(&m1, &m2);
            moons[i1] = m1u;
            moons[i2] = m2u;
        }
    }

    for m in moons {
        m.update_position();
    }
}

fn get_updated_velocities(p1: i32, p2: i32, v1: i32, v2: i32) -> (i32, i32) {
    if p1 > p2 {
        (v1 - 1, v2 + 1)
    } else if p1 < p2 {
        (v1 + 1, v2 - 1)
    } else {
        (v1, v2)
    }
}

fn update_velocity(m1: &Moon, m2: &Moon) -> (Moon, Moon) {
    let (vx1, vx2) = get_updated_velocities(m1.position.x, m2.position.x, m1.velocity.x, m2.velocity.x);
    let (vy1, vy2) = get_updated_velocities(m1.position.y, m2.position.y, m1.velocity.y, m2.velocity.y);
    let (vz1, vz2) = get_updated_velocities(m1.position.z, m2.position.z, m1.velocity.z, m2.velocity.z);
    (
        Moon { position: m1.position, velocity: Vector3D { x: vx1, y: vy1, z: vz1 } },
        Moon { position: m2.position, velocity: Vector3D { x: vx2, y: vy2, z: vz2 } },
    )
}

fn sequence_equal<T: Eq>(lhs: &[T], rhs: &[T]) -> bool {
    if lhs.len() != rhs.len() {
        return false;
    }

    for i in 0..lhs.len() {
        if lhs[i] != rhs[i] {
            return false;
        }
    }

    true
}

pub fn step_one(p: &mut [i32], v: &mut [i32]) {
    for i in 0..p.len() {
        for j in 0..p.len() {
            if p[i] > p[j] {
                v[i] -= 1;
            } else if p[i] < p[j] {
                v[i] += 1;
            }
        }
    }

    for i in 0..p.len() {
        p[i] += v[i];
    }
}

pub fn find_repeat(moons: &mut[Moon; 4]) -> u128 {
    let mut px: Vec<i32> = moons.iter().map(|m| m.position.x).collect();
    let mut py: Vec<i32> = moons.iter().map(|m| m.position.y).collect();
    let mut pz: Vec<i32> = moons.iter().map(|m| m.position.z).collect();
    let mut vx: Vec<i32> = moons.iter().map(|m| m.velocity.x).collect();
    let mut vy: Vec<i32> = moons.iter().map(|m| m.velocity.y).collect();
    let mut vz: Vec<i32> = moons.iter().map(|m| m.velocity.z).collect();

    let pxi = px.clone();
    let pyi = py.clone();
    let pzi = pz.clone();
    let vxi = vx.clone();
    let vyi = vy.clone();
    let vzi = vz.clone();

    let x = first_repeat_one(&mut px, &mut vx, &pxi, &vxi);
    let y = first_repeat_one(&mut py, &mut vy, &pyi, &vyi);
    let z = first_repeat_one(&mut pz, &mut vz, &pzi, &vzi);

    println!("x: {}, y: {}, z: {}", x, y, z);
    lcm(x, lcm(y, z))
}

fn first_repeat_one(p: &mut [i32], v: &mut [i32], pi: &[i32], vi: &[i32]) -> u128 {
    let mut i = 1;
    loop {
        step_one(p, v);
        if sequence_equal(&pi, &p) && sequence_equal(&vi, &v) {
            println!("pi: {:?}, p: {:?}", &pi, &p);
            println!("vi: {:?}, v: {:?}", &vi, &v);
            return i;
        }

        i += 1;
    }
}