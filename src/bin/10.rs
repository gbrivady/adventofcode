use std::fs::read_to_string;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Pipe {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Pipe {
    const START: Pipe = Pipe {
        up: true,
        down: true,
        left: true,
        right: true,
    };
    const VERTICAL: Pipe = Pipe {
        up: true,
        down: true,
        left: false,
        right: false,
    };
    const HORIZONTAL: Pipe = Pipe {
        up: false,
        down: false,
        left: true,
        right: true,
    };
    const NEBEND: Pipe = Pipe {
        up: true,
        down: false,
        left: false,
        right: true,
    };
    const NWBEND: Pipe = Pipe {
        up: true,
        down: false,
        left: true,
        right: false,
    };
    const SWBEND: Pipe = Pipe {
        up: false,
        down: true,
        left: true,
        right: false,
    };
    const SEBEND: Pipe = Pipe {
        up: false,
        down: true,
        left: false,
        right: true,
    };
    const GROUND: Pipe = Pipe {
        up: false,
        down: false,
        left: false,
        right: false,
    };
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '|' => Pipe::VERTICAL,
            '-' => Pipe::HORIZONTAL,
            'L' => Pipe::NEBEND,
            'J' => Pipe::NWBEND,
            '7' => Pipe::SWBEND,
            'F' => Pipe::SEBEND,
            'S' => Pipe::START,
            _ => Pipe::GROUND,
        }
    }
}

fn get_successor(
    cur_pos: (usize, usize),
    old_pos: (usize, usize),
    grid: &Vec<Vec<Pipe>>,
) -> (usize, usize) {
    //check up
    let (i, j) = cur_pos;

    if i > 0 {
        if grid[i][j].up && grid[i - 1][j].down && (i - 1, j) != old_pos {
            return (i - 1, j);
        }
    }
    if i < grid.len() - 1 {
        if grid[i][j].down && grid[i + 1][j].up && (i + 1, j) != old_pos {
            return (i + 1, j);
        }
    }
    if j > 0 {
        if grid[i][j].left && grid[i][j - 1].right && (i, j - 1) != old_pos {
            return (i, j - 1);
        }
    }
    if j < grid[0].len() - 1 {
        if grid[i][j].right && grid[i][j + 1].left && (i, j + 1) != old_pos {
            return (i, j + 1);
        }
    }
    panic!("One neighbor should match, found none at cell {} {}", i, j);
}

fn color(i: usize, j: usize, grid_color: &mut Vec<Vec<i8>>) {
    if grid_color[i][j] != 0 {
        return;
    }
    grid_color[i][j] = 1;
    if i > 0 {
        color(i - 1, j, grid_color);
    }
    if i < grid_color.len() - 1 {
        color(i + 1, j, grid_color);
    }
    if j > 0 {
        color(i, j - 1, grid_color);
    }
    if j < grid_color[0].len() - 1 {
        color(i, j + 1, grid_color);
    }
}

fn color_in_dir(dir: Direction, i: usize, j: usize, grid_color: &mut Vec<Vec<i8>>) {
    match dir {
        Direction::Up => {
            if i > 0 {
                color(i - 1, j, grid_color);
            }
        }
        Direction::Down => {
            if i < grid_color.len() - 1 {
                color(i + 1, j, grid_color);
            }
        }
        Direction::Left => {
            if j > 0 {
                color(i, j - 1, grid_color);
            }
        }
        Direction::Right => {
            if j < grid_color[0].len() - 1 {
                color(i, j + 1, grid_color);
            }
        }
    };
}

fn main() {
    let mut grid: Vec<Vec<Pipe>> = Vec::new();
    for line in read_to_string("input/10").unwrap().lines() {
        grid.push(line.chars().map(|c| Pipe::from(c)).collect())
    }
    let n = grid.len();
    let m = grid[0].len();

    // Find starting position
    let mut i = 0;
    let mut j = 0;

    '_outer: while i < n {
        j = 0;
        while j < m {
            if grid[i][j] == Pipe::START {
                break '_outer;
            }
            j += 1;
        }
        i += 1;
    }

    let mut loop_nodes: Vec<(usize, usize)> = Vec::new();
    loop_nodes.push((i, j));

    let (mut i_next, mut j_next) = get_successor((i, j), (usize::MAX, usize::MAX), &grid);
    while grid[i_next][j_next] != Pipe::START {
        let temp = (i_next, j_next);
        loop_nodes.push((i_next, j_next));
        (i_next, j_next) = get_successor((i_next, j_next), (i, j), &grid);
        (i, j) = temp;
    }
    println!("Part 1: {}", loop_nodes.len() / 2);

    let loop_color = -1;
    let mut grid_coloring: Vec<Vec<i8>> = vec![vec![0; m]; n];

    for (i, j) in loop_nodes.iter() {
        grid_coloring[*i][*j] = loop_color;
    }
    //if a point is NOT inside the loop, there is some way for it to reach the border of the grid.
    // i.e. if I paint starting at the borders of the grid, I WILL reach all outside points, only left is to count
    // whats not colored
    //then I need to find SOME kind of "|" or '-' accessible from the border

    // Cast rays starting at the top, going down, looking for a '-' within the loop
    i = 0;
    j = 0;
    'outer: while j < m {
        i = 0;
        j += 1;
        while i < n {
            if grid_coloring[i][j] == -1 {
                if grid[i][j] == Pipe::HORIZONTAL {
                    break 'outer;
                }
                break;
            }
            i += 1;
        }
    }

    let mut direction = Direction::Up;

    let index_start = loop_nodes.iter().position(|&x| x == (i, j)).unwrap();
    let mut index = (index_start + 1) % loop_nodes.len();
    while index != index_start {
        let (i, j) = loop_nodes[index];
        match grid[i][j] {
            //In my input, Start pipe is a vertical one
            Pipe::HORIZONTAL | Pipe::VERTICAL | Pipe::START => {
                color_in_dir(direction, i, j, &mut grid_coloring);
            }
            Pipe::NWBEND | Pipe::SEBEND => match direction {
                Direction::Up => {
                    color_in_dir(direction, i, j, &mut grid_coloring);
                    direction = Direction::Left;
                    color_in_dir(direction, i, j, &mut grid_coloring);
                }
                Direction::Down => {
                    color_in_dir(direction, i, j, &mut grid_coloring);
                    direction = Direction::Right;
                    color_in_dir(direction, i, j, &mut grid_coloring);
                }
                Direction::Left => {
                    color_in_dir(direction, i, j, &mut grid_coloring);
                    direction = Direction::Up;
                    color_in_dir(direction, i, j, &mut grid_coloring);
                }
                Direction::Right => {
                    color_in_dir(direction, i, j, &mut grid_coloring);
                    direction = Direction::Down;
                    color_in_dir(direction, i, j, &mut grid_coloring);
                }
            },
            Pipe::NEBEND | Pipe::SWBEND => match direction {
                Direction::Up => {
                    color_in_dir(direction, i, j, &mut grid_coloring);
                    direction = Direction::Right;
                    color_in_dir(direction, i, j, &mut grid_coloring);
                }
                Direction::Down => {
                    color_in_dir(direction, i, j, &mut grid_coloring);
                    direction = Direction::Left;
                    color_in_dir(direction, i, j, &mut grid_coloring);
                }
                Direction::Left => {
                    color_in_dir(direction, i, j, &mut grid_coloring);
                    direction = Direction::Down;
                    color_in_dir(direction, i, j, &mut grid_coloring);
                }
                Direction::Right => {
                    color_in_dir(direction, i, j, &mut grid_coloring);
                    direction = Direction::Up;
                    color_in_dir(direction, i, j, &mut grid_coloring);
                }
            },
            _ => unreachable!(),
        }
        index = (index + 1) % loop_nodes.len();
    }

    println!(
        "Part 2: {}",
        grid_coloring.iter().fold(0, |acc, line| {
            acc + line
                .iter()
                .fold(0, |acc, cell| if *cell == 0 { acc + 1 } else { acc })
        })
    );
}
