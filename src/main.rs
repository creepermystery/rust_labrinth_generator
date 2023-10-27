use rand::Rng;

fn print_labrinth(labrinth:&Vec<Vec<usize>>) {
    let labrinth_size = labrinth.len();
    for i in 0..labrinth_size {
        for j in 0..labrinth_size {
            if labrinth[j][i] == 1 {
                print!("█");
            }
            else {
                print!(" ")
            }
        }
        println!("");
    }   
}

fn grid_builder (grid_size:usize) -> Vec<Vec<usize>> {

    let internal_size: usize = grid_size-2;

    let mut grid: Vec<Vec<usize>> = Vec::new();
    let full_collum: Vec<usize> = vec![1; grid_size];

    grid.push(full_collum.clone());

    let mut iterator = 1; // Not 0 because first pass would be a 1 -> wall
    for i in 0..internal_size {
        let mut collum: Vec<usize> = Vec::new();
        if i%2 == 1 {
            grid.push(full_collum.clone());
        }
        else {
            for j in 0..grid_size {
                if j%2 == 0 {
                    collum.push(1);
                }
                else {
                    iterator += 1;
                    collum.push(iterator);
                }
            }
            grid.push(collum);
        }
    }
    grid.push(full_collum);

    return grid
}

fn labrinth_bore (labrinth:&mut Vec<Vec<usize>>) -> &mut Vec<Vec<usize>> {
    let labrinth_size = labrinth.len();

    while labrinth_still_boring(labrinth) {
        let mut valid_walls_coords: Vec<Vec<usize>> = Vec::new();

        for i in 1..labrinth_size-1 {
            for j in 1..labrinth_size-1 {
                let wall_coords = vec![i, j];
                if i%2 == 0 && j%2 == 1 {
                    let left_cell = labrinth[i-1][j];
                    let right_cell = labrinth[i+1][j];
                    if left_cell != right_cell {
                        valid_walls_coords.push(wall_coords);
                    }
                }
                else if i%2 == 1 && j%2 == 0 {
                    let up_cell = labrinth[i][j-1];
                    let down_cell = labrinth[i][j+1];
                    if up_cell != down_cell {
                        valid_walls_coords.push(wall_coords);
                    }
                }
            }
        }

        let chosen_wall = rand::thread_rng().gen_range(0..valid_walls_coords.len());
        let chosen_wall_coords = valid_walls_coords[chosen_wall].clone();
        valid_walls_coords.remove(chosen_wall);

        if chosen_wall_coords[0]%2 == 0 && chosen_wall_coords[1]%2 == 1 {
            let left_number = labrinth[chosen_wall_coords[0]-1][chosen_wall_coords[1]];
            let right_number = labrinth[chosen_wall_coords[0]+1][chosen_wall_coords[1]];
            let _ = std::mem::replace(&mut labrinth[chosen_wall_coords[0]][chosen_wall_coords[1]], left_number);
            replace_number(labrinth, left_number, right_number);
        }
        else if chosen_wall_coords[0]%2 == 1 && chosen_wall_coords[1]%2 == 0 {
            let up_number = labrinth[chosen_wall_coords[0]][chosen_wall_coords[1]-1];
            let down_number = labrinth[chosen_wall_coords[0]][chosen_wall_coords[1]+1];
            let _ = std::mem::replace(&mut labrinth[chosen_wall_coords[0]][chosen_wall_coords[1]], up_number);
            replace_number(labrinth, up_number, down_number);
        }
    }
    return labrinth
}

fn replace_number (labrinth:&mut Vec<Vec<usize>>, number_to_keep:usize, number_to_replace:usize) -> &mut Vec<Vec<usize>>{
    let labrinth_size = labrinth.len()-1;
    for i in 1..labrinth_size {
        for j in 1..labrinth_size {
            if labrinth[i][j] == number_to_replace {
                let _ = std::mem::replace(&mut labrinth[i][j], number_to_keep);
            }
        }
    }
    return labrinth
}

fn labrinth_still_boring (labrinth:&Vec<Vec<usize>>) -> bool {
    let labrinth_size = labrinth.len()-1;
    let mut labrinth_contents = Vec::new();

    for i in 1..labrinth_size {
        for j in 1..labrinth_size {
            if !labrinth_contents.contains(&labrinth[i][j]) {
                labrinth_contents.push(labrinth[i][j]);
            }
        }
    }
    if labrinth_contents.len() > 2 {
        return true
    }
    else {
        return false
    }
}

fn doors(labrinth:&mut Vec<Vec<usize>>) -> &mut Vec<Vec<usize>> {
    let labrinth_size = labrinth.len();
    let inner_value = labrinth[1][1];
    let _ = std::mem::replace(&mut labrinth[0][1], inner_value);
    let _ = std::mem::replace(&mut labrinth[labrinth_size-1][labrinth_size-2], inner_value);
    return labrinth
}

fn main() {
    
    const LABRINTH_SIZE: usize = 91; // MUST be odd and greater than 4 for the program to function properly

    let mut base_grid = grid_builder(LABRINTH_SIZE);
    let labrinth = labrinth_bore(&mut base_grid);
    let labrinth = doors(labrinth);

    print_labrinth(labrinth);
}