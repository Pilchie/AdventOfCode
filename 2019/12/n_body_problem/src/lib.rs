#[derive(Debug)]
struct Vector3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Clone for Vector3D {
    fn clone(&self) -> Vector3D {
        Vector3D { ..*self }
    }
}

impl Copy for Vector3D {}

impl std::ops::Add for Vector3D {
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


#[derive(Debug)]
pub struct Moon {
    position: Vector3D,
    velocity: Vector3D,
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
    fn new(x: i32, y: i32, z: i32) -> Moon {
        Moon {
            position: Vector3D { x, y, z },
            velocity: Vector3D { x: 0, y: 0, z: 0 },
        }
    }

    fn update_position(&mut self) {
        self.position = self.position + self.velocity;
    }
}

pub fn step(moons: &mut Vec<Moon>) {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_moons(actual: &[Moon], expected: &[Moon]) {
        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert_eq!(expected[i], actual[i]);
        }
    }

    #[test]
    fn first_ten_steps() {
        let mut moons = vec![ Moon::new(-1, 0, 2), Moon::new(2, -10, -7), Moon::new(4, -8, 8), Moon::new(3, 5, -1), ];

        assert_moons(
            &moons,
            &vec![
                Moon { position: Vector3D { x: -1, y:   0, z:  2 }, velocity: Vector3D { x: 0, y: 0, z: 0 }, },
                Moon { position: Vector3D { x:  2, y: -10, z: -7 }, velocity: Vector3D { x: 0, y: 0, z: 0 }, },
                Moon { position: Vector3D { x:  4, y:  -8, z:  8 }, velocity: Vector3D { x: 0, y: 0, z: 0 }, },
                Moon { position: Vector3D { x:  3, y:   5, z: -1 }, velocity: Vector3D { x: 0, y: 0, z: 0 }, },
            ],
        );

        step(&mut moons);
        assert_moons(
            &moons,
            &vec![
                Moon { position: Vector3D { x: 2, y: -1, z:  1 }, velocity: Vector3D { x:  3, y: -1, z: -1 }, },
                Moon { position: Vector3D { x: 3, y: -7, z: -4 }, velocity: Vector3D { x:  1, y:  3, z:  3 }, },
                Moon { position: Vector3D { x: 1, y: -7, z:  5 }, velocity: Vector3D { x: -3, y:  1, z: -3 }, },
                Moon { position: Vector3D { x: 2, y:  2, z:  0 }, velocity: Vector3D { x: -1, y: -3, z:  1 }, },
            ],
        );

        step(&mut moons);
        assert_moons(
            &moons,
            &vec![
                Moon { position: Vector3D { x: 5, y: -3, z: -1 }, velocity: Vector3D { x:  3, y: -2, z: -2 }, },
                Moon { position: Vector3D { x: 1, y: -2, z:  2 }, velocity: Vector3D { x: -2, y:  5, z:  6 }, },
                Moon { position: Vector3D { x: 1, y: -4, z: -1 }, velocity: Vector3D { x:  0, y:  3, z: -6 }, },
                Moon { position: Vector3D { x: 1, y: -4, z:  2 }, velocity: Vector3D { x: -1, y: -6, z:  2 }, },
            ],
        );

        step(&mut moons);
        assert_moons(
            &moons,
            &vec![
                Moon { position: Vector3D { x: 5, y: -6, z: -1 }, velocity: Vector3D { x:  0, y: -3, z:  0 }, },
                Moon { position: Vector3D { x: 0, y:  0, z:  6 }, velocity: Vector3D { x: -1, y:  2, z:  4 }, },
                Moon { position: Vector3D { x: 2, y:  1, z: -5 }, velocity: Vector3D { x:  1, y:  5, z: -4 }, },
                Moon { position: Vector3D { x: 1, y: -8, z:  2 }, velocity: Vector3D { x:  0, y: -4, z:  0 }, },
            ],
        );

        step(&mut moons);
        assert_moons(
            &moons,
            &vec![
                Moon { position: Vector3D { x: 2, y: -8, z:  0 }, velocity: Vector3D { x: -3, y: -2, z:  1 }, },
                Moon { position: Vector3D { x: 2, y:  1, z:  7 }, velocity: Vector3D { x:  2, y:  1, z:  1 }, },
                Moon { position: Vector3D { x: 2, y:  3, z: -6 }, velocity: Vector3D { x:  0, y:  2, z: -1 }, },
                Moon { position: Vector3D { x: 2, y: -9, z:  1 }, velocity: Vector3D { x:  1, y: -1, z: -1 }, },
            ],
        );

        step(&mut moons);
        assert_moons(
            &moons,
            &vec![
                Moon { position: Vector3D { x: -1, y: -9, z:  2 }, velocity: Vector3D { x: -3, y: -1, z:  2 }, },
                Moon { position: Vector3D { x:  4, y:  1, z:  5 }, velocity: Vector3D { x:  2, y:  0, z: -2 }, },
                Moon { position: Vector3D { x:  2, y:  2, z: -4 }, velocity: Vector3D { x:  0, y: -1, z:  2 }, },
                Moon { position: Vector3D { x:  3, y: -7, z: -1 }, velocity: Vector3D { x:  1, y:  2, z: -2 }, },
            ],
        );

        step(&mut moons);
        assert_moons(
            &moons,
            &vec![
                Moon { position: Vector3D { x: -1, y: -7, z:  3 }, velocity: Vector3D { x:  0, y:  2, z:  1 }, },
                Moon { position: Vector3D { x:  3, y:  0, z:  0 }, velocity: Vector3D { x: -1, y: -1, z: -5 }, },
                Moon { position: Vector3D { x:  3, y: -2, z:  1 }, velocity: Vector3D { x:  1, y: -4, z:  5 }, },
                Moon { position: Vector3D { x:  3, y: -4, z: -2 }, velocity: Vector3D { x:  0, y:  3, z: -1 }, },
            ],
        );

        step(&mut moons);
        assert_moons(
            &moons,
            &vec![
                Moon { position: Vector3D { x: 2, y: -2, z:  1 }, velocity: Vector3D { x:  3, y:  5, z: -2 }, },
                Moon { position: Vector3D { x: 1, y: -4, z: -4 }, velocity: Vector3D { x: -2, y: -4, z: -4 }, },
                Moon { position: Vector3D { x: 3, y: -7, z:  5 }, velocity: Vector3D { x:  0, y: -5, z:  4 }, },
                Moon { position: Vector3D { x: 2, y:  0, z:  0 }, velocity: Vector3D { x: -1, y:  4, z:  2 }, },
            ],
        );

        step(&mut moons);
        assert_moons(
            &moons,
            &vec![
                Moon { position: Vector3D { x: 5, y:  2, z: -2 }, velocity: Vector3D { x:  3, y:  4, z: -3 }, },
                Moon { position: Vector3D { x: 2, y: -7, z: -5 }, velocity: Vector3D { x:  1, y: -3, z: -1 }, },
                Moon { position: Vector3D { x: 0, y: -9, z:  6 }, velocity: Vector3D { x: -3, y: -2, z:  1 }, },
                Moon { position: Vector3D { x: 1, y:  1, z:  3 }, velocity: Vector3D { x: -1, y:  1, z:  3 }, },
            ],
        );

        step(&mut moons);
        assert_moons(
            &moons,
            &vec![
                Moon { position: Vector3D { x: 5, y:  3, z: -4 }, velocity: Vector3D { x: 0, y:  1, z: -2 }, },
                Moon { position: Vector3D { x: 2, y: -9, z: -3 }, velocity: Vector3D { x: 0, y: -2, z:  2 }, },
                Moon { position: Vector3D { x: 0, y: -8, z:  4 }, velocity: Vector3D { x: 0, y:  1, z: -2 }, },
                Moon { position: Vector3D { x: 1, y:  1, z:  5 }, velocity: Vector3D { x: 0, y:  0, z:  2 }, },
            ],
        );

        step(&mut moons);
        assert_moons(
            &moons,
            &vec![
                Moon { position: Vector3D { x: 2, y:  1, z: -3 }, velocity: Vector3D { x: -3, y: -2, z:  1 }, },
                Moon { position: Vector3D { x: 1, y: -8, z:  0 }, velocity: Vector3D { x: -1, y:  1, z:  3 }, },
                Moon { position: Vector3D { x: 3, y: -6, z:  1 }, velocity: Vector3D { x:  3, y:  2, z: -3 }, },
                Moon { position: Vector3D { x: 2, y:  0, z:  4 }, velocity: Vector3D { x:  1, y: -1, z: -1 }, },
            ],
        );
    }
}
