use sscanf::sscanf;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Axis {
    X,
    Y,
    Z,
    Cube,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point3D {
    x: u32,
    y: u32,
    z: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Brick {
    start: Point3D,
    direction: Axis,
    size: u32,
}

#[derive(Debug)]
struct ParseBrickError;

struct BrickNode {
    supported_by: Vec<Point3D>,
    supporting: Vec<Point3D>,
    is_falling: bool,
}

impl FromStr for Brick {
    type Err = ParseBrickError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split("~");
        let (x0, y0, z0) = sscanf!(it.next().unwrap(), "{},{},{}", u32, u32, u32).unwrap();
        let (x1, y1, z1) = sscanf!(it.next().unwrap(), "{},{},{}", u32, u32, u32).unwrap();
        let dir: Axis = {
            if x0 != x1 {
                Axis::X
            } else if y0 != y1 {
                Axis::Y
            } else if z0 != z1 {
                Axis::Z
            } else {
                Axis::Cube
            }
        };
        return Ok(Brick {
            start: Point3D {
                x: x0,
                y: y0,
                z: z0,
            },
            direction: dir,
            size: match dir {
                Axis::X => x1 - x0,
                Axis::Y => y1 - y0,
                Axis::Z => z1 - z0,
                Axis::Cube => 0,
            },
        });
    }
}
impl Brick {
    fn can_support(&self, other: &Self) -> bool {
        let self_int_len: (u32, u32) = match self.direction {
            Axis::Cube | Axis::Z => (1, 1),
            Axis::X => (self.size + 1, 1),
            Axis::Y => (1, self.size + 1),
        };
        let other_int_len: (u32, u32) = match other.direction {
            Axis::Cube | Axis::Z => (1, 1),
            Axis::X => (other.size + 1, 1),
            Axis::Y => (1, other.size + 1),
        };
        return interval_meet_interval(
            self.start.x,
            self_int_len.0,
            other.start.x,
            other_int_len.0,
        ) && interval_meet_interval(
            self.start.y,
            self_int_len.1,
            other.start.y,
            other_int_len.1,
        );
    }
}

fn interval_meet_interval(a: u32, la: u32, b: u32, lb: u32) -> bool {
    return (a >= b && a < b + lb) || (b >= a && b < a + la);
}

fn main() {
    let mut all_bricks: Vec<Brick> = read_to_string("input/22")
        .unwrap()
        .lines()
        .map(|s| s.parse::<Brick>().unwrap())
        .collect();

    // sort bricks by z axis position
    all_bricks.sort_by_key(|b| b.start.z);
    let n = all_bricks.len();
    let mut brick_by_height: Vec<Vec<&Brick>> =
        vec![Vec::new(); all_bricks.iter().max_by_key(|b| b.start.z).unwrap().start.z as usize];
    let mut support_bricks: HashSet<Point3D> = HashSet::new();

    let mut brick_network: HashMap<Point3D, BrickNode> = HashMap::new();
    // Make bricks fall
    for b in all_bricks.iter_mut() {
        let mut current_supports: Vec<&Brick> = Vec::new();
        'outer: while b.start.z != 1 {
            for b2 in brick_by_height[(b.start.z - 2) as usize].iter() {
                // Can b2 support b?
                if b2.can_support(b) {
                    current_supports.push(b2);
                }
            }
            if !current_supports.is_empty() {
                break 'outer;
            }
            b.start.z -= 1;
        }
        if current_supports.len() == 1 {
            support_bricks.insert(current_supports[0].start);
        }
        brick_network.insert(
            b.start,
            BrickNode {
                supported_by: current_supports.iter().map(|b2| b2.start).collect(),
                supporting: Vec::new(),
                is_falling: false,
            },
        );
        for b2 in current_supports.iter() {
            brick_network
                .get_mut(&b2.start)
                .unwrap()
                .supporting
                .push(b.start);
        }
        brick_by_height[(b.start.z - 1) as usize].push(b);
        if b.direction == Axis::Z {
            for i in 1..=b.size {
                brick_by_height[(b.start.z + i - 1) as usize].push(b);
            }
        }
    }

    let mut sum = 0;
    for p in support_bricks.iter() {
        let node_p = brick_network.get_mut(&p).unwrap();

        node_p.is_falling = true;
        let mut may_fall = node_p.supporting.clone();

        while !may_fall.is_empty() {
            let mut new_may_fall = Vec::new();
            for p_mf in may_fall.iter() {
                let previous = brick_network.get(&p_mf).unwrap().supported_by.clone();
                let is_falling = previous
                    .iter()
                    .all(|p_prev| brick_network.get(p_prev).unwrap().is_falling);
                let a = brick_network.get_mut(&p_mf).unwrap();
                a.is_falling = is_falling;
                if is_falling {
                    new_may_fall.append(&mut a.supporting.clone());
                }
            }
            may_fall = new_may_fall;
        }
        sum += brick_network.iter_mut().fold(0, |acc, (_, v)| {
            if v.is_falling {
                v.is_falling = false;
                acc + 1
            } else {
                acc
            }
        }) - 1;
    }
    println!("Part 1: {}", n - support_bricks.len());
    println!("Part 2: {}", sum);
}
