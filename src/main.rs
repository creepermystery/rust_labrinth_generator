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
    size: usize,
    matrix: Vec<Vec<Cell>>,
}

fn build_labrinth (size: usize) -> Labrinth {
    let new_labrinth = build_grid(size);
    new_labrinth
}

fn build_grid (size: usize) -> Labrinth {
    let mut grid: Vec<Vec<Cell>> = Vec::new();
    let full_wall = vec![Cell::Wall; size];
    let mut iterator = 1;

    for i in 0..size {
        if i%2 == 0 {
            grid.push(full_wall.clone());
        } else {
            grid.push(create_half_wall(size, &mut iterator));
        }
    }

    Labrinth {
        size,
        matrix: grid,
    }
}

fn create_half_wall (size: usize, iterator: &mut usize) -> Vec<Cell> {
    let mut result: Vec<Cell> = Vec::new();

    for i in 0..size {
        if i%2 == 0 {
            result.push(Cell::Wall);
        } else {
            result.push(Cell::Corridor(*iterator));
            *iterator += 1;
        }
    }

    result
}

fn bore_labrinth (mut labrinth: Labrinth) -> Labrinth {
    loop {
        let walls_to_bore = identify_walls_to_bore(&labrinth);
        if walls_to_bore.len() == 0 {
            break
        }
        bore_path(&mut labrinth, &walls_to_bore);
    }
    doors(&mut labrinth);
    // random_boring();

    labrinth
}

fn doors (labrinth: &mut Labrinth) {
    let size = labrinth.size-1;
    let matrix = &mut labrinth.matrix;
    let id = matrix[1][1].get_corridor_id();
    matrix[1][0] = Cell::Corridor(id);
    matrix[size-1][size] = Cell::Corridor(id);
}

fn identify_walls_to_bore (labrinth: &Labrinth) -> Vec<(usize, usize)> {
    let size = labrinth.size;
    let mut wall_coords: Vec<(usize, usize)> = Vec::new();

    for i in 1..size-1 {
        for j in 1..size-1 {
            if wall_is_valid(labrinth, (i, j)) {
                wall_coords.push((i, j))
            }
        }
    }

    wall_coords
}

fn wall_is_valid (labrinth: &Labrinth, coords: (usize, usize)) -> bool {
    let size = labrinth.size-1;
    let (x, y) = coords;

    if x == 0 || y == 0 || x == size || y == size {
        return false;
    }
    if x%2 == y%2 {
        return false
    }
    if x%2 == 0 { // If it's a left-right wall
        let left_cell = &labrinth.matrix[x-1][y];
        let right_cell = &labrinth.matrix[x+1][y];

        return same_corridor_id(left_cell, right_cell)
    } else { // If it's a up-down wall
        let up_cell = &labrinth.matrix[x][y-1];
        let down_cell = &labrinth.matrix[x][y+1];

        return same_corridor_id(up_cell, down_cell)
    }
}

fn same_corridor_id (cell_a: &Cell, cell_b: &Cell) -> bool {
    return cell_a.get_corridor_id() != cell_b.get_corridor_id();
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
    for i in 1..labrinth.size-1 {
        for j in 1..labrinth.size-1 {
            let inspected_cell = &mut labrinth.matrix[i][j];
            if inspected_cell.get_corridor_id() == previous_id {
                *inspected_cell = Cell::Corridor(future_id);
            }
        }
    }
}

fn print_labrinth (labrinth: &Labrinth) {
    let size = labrinth.size;

    for i in 0..size {
        for j in 0..size {
            match labrinth.matrix[i][j] {
                Cell::Wall => print!("██"),
                Cell::Corridor(_) => print!("  "),
            }
        }
        println!("");
    }
}

fn print_corridor_id (labrinth: &Labrinth) {
    let size = labrinth.size;
    
    for i in 0..size {
        for j in 0..size {
            match labrinth.matrix[i][j] {
                Cell::Wall => print!("██"),
                Cell::Corridor(id) => print!("{} ", id),
            }
        }
        println!("");
    }
}

fn main () {
    const SIZE: usize = 85;
    
    print_labrinth(&bore_labrinth(build_labrinth(SIZE)));
}