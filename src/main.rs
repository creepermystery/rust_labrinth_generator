use rand::Rng;

#[derive(Clone)]
enum Cell {
    Wall,
    Corridor(usize),
}

impl Cell {
    fn get_corridor_id (&self) -> usize {
        match self {
            Cell::Corridor(id) => return *id,
            _ => 0,
        }
    }
}

struct Labrinth {
    height: usize,
    width: usize,
    matrix: Vec<Vec<Cell>>,
}

impl Labrinth {
    fn new (height: usize, width: usize) -> Labrinth {
        let mut labrinth = build_grid(height, width);

        loop {
            let walls_to_bore = identify_walls_to_bore(&labrinth);
            if walls_to_bore.len() == 0 {
                break
            }
            bore_path(&mut labrinth, &walls_to_bore);
        }
        doors(&mut labrinth);
        random_boring(&mut labrinth);
    
        labrinth
    }
}

fn build_grid (height: usize, width: usize) -> Labrinth {
    let mut grid: Vec<Vec<Cell>> = Vec::new();
    let full_wall = vec![Cell::Wall; height];
    let mut iterator = 1;

    for i in 0..width {
        if i%2 == 0 {
            grid.push(full_wall.clone());
        } else {
            grid.push(create_half_wall(height, &mut iterator));
        }
    }

    Labrinth {
        height,
        width,
        matrix: grid,
    }
}

fn create_half_wall (height: usize, iterator: &mut usize) -> Vec<Cell> {
    let mut result: Vec<Cell> = Vec::new();

    for j in 0..height {
        if j%2 == 0 {
            result.push(Cell::Wall);
        } else {
            result.push(Cell::Corridor(*iterator));
            *iterator += 1;
        }
    }

    result
}

fn random_boring (labrinth: &mut Labrinth) {
    let height = labrinth.height;
    let width = labrinth.width;
    let matrix = &mut labrinth.matrix;
    let id = matrix[1][1].get_corridor_id();

    for i in 0..width {
        for j in 0..height {
            if i == 0 || j == 0 || i == width-1 || j == height-1 {
                continue
            } else if i%2 == 0 && j%2 == 1 {
                if rand::thread_rng().gen_range(0..40) == 0 {
                    matrix[i][j] = Cell::Corridor(id);
                }
            } else if i%2 == 1 && j%2 == 0 {
                if rand::thread_rng().gen_range(0..40) == 0 {
                    matrix[i][j] = Cell::Corridor(id);
                }
            }
        }
    }
}

fn doors (labrinth: &mut Labrinth) {
    let matrix = &mut labrinth.matrix;
    let id = matrix[1][1].get_corridor_id();
    matrix[1][0] = Cell::Corridor(id);
    matrix[labrinth.width-2][labrinth.height-1] = Cell::Corridor(id);
}

fn identify_walls_to_bore (labrinth: &Labrinth) -> Vec<(usize, usize)> {
    let mut wall_coords: Vec<(usize, usize)> = Vec::new();

    for i in 1..labrinth.width-1 {
        for j in 1..labrinth.height-1 {
            if wall_is_valid(labrinth, (i, j)) {
                wall_coords.push((i, j))
            }
        }
    }

    wall_coords
}

fn wall_is_valid (labrinth: &Labrinth, coords: (usize, usize)) -> bool {
    let (x, y) = coords;

    if x == 0 || y == 0 || x == labrinth.width-1 || y == labrinth.height-1 {
        return false;
    }
    if x%2 == y%2 {
        return false
    }
    if x%2 == 0 { // If it's a left-right wall
        let left_cell = &labrinth.matrix[x-1][y];
        let right_cell = &labrinth.matrix[x+1][y];

        return left_cell.get_corridor_id() != right_cell.get_corridor_id()
    } else { // If it's a up-down wall
        let up_cell = &labrinth.matrix[x][y-1];
        let down_cell = &labrinth.matrix[x][y+1];

        return up_cell.get_corridor_id() != down_cell.get_corridor_id()
    }
}

fn bore_path (labrinth: &mut Labrinth, valid_walls_coords: &Vec<(usize, usize)>) {
    let matrix = &mut labrinth.matrix;
    let (x, y) = valid_walls_coords[rand::thread_rng().gen_range(0..valid_walls_coords.len())];
    
    if x%2 == 0 {
        let future_id = matrix[x-1][y].get_corridor_id();
        let previous_id = matrix[x+1][y].get_corridor_id();
        matrix[x][y] = Cell::Corridor(future_id);
        replace_number(labrinth, previous_id, future_id);
    } else {
        let future_id = matrix[x][y+1].get_corridor_id();
        let previous_id = matrix[x][y-1].get_corridor_id();
        matrix[x][y] = Cell::Corridor(future_id);
        replace_number(labrinth, previous_id, future_id);
    }
}

fn replace_number (labrinth: &mut Labrinth, previous_id: usize, future_id: usize) {
    for i in 1..labrinth.width-1 {
        for j in 1..labrinth.height-1 {
            let inspected_cell = &mut labrinth.matrix[i][j];
            if inspected_cell.get_corridor_id() == previous_id {
                *inspected_cell = Cell::Corridor(future_id);
            }
        }
    }
}

fn print_labrinth (labrinth: &Labrinth) {
    for i in 0..labrinth.width {
        for j in 0..labrinth.height {
            match labrinth.matrix[i][j] {
                Cell::Wall => print!("██"),
                Cell::Corridor(_) => print!("  "),
            }
        }
        println!("");
    }
}

fn print_corridor_id (labrinth: &Labrinth) {
    for i in 0..labrinth.width {
        for j in 0..labrinth.height {
            match labrinth.matrix[i][j] {
                Cell::Wall => print!("██"),
                Cell::Corridor(id) => print!("{} ", id),
            }
        }
        println!("");
    }
}

fn main () {
    const HEIGHT: usize = 65;
    const WIDTH: usize = 31;
    
    print_labrinth(&Labrinth::new(HEIGHT, WIDTH));

}