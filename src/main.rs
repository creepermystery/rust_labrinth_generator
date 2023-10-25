use rand::Rng;

fn print_labrinth(labrinth: Vec<Vec<usize>>) {
    let labrinth_size = labrinth.len();
    for k in 0..labrinth_size {
        for j in 0..labrinth_size {
            if labrinth[j][k] == 1 {
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
            for l in 0..grid_size {
                if l%2 == 0 {
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

fn labrinth_bore (grid:Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let labrinth_size = grid.len();
    while labrinth_finished(grid) == 0 {
        let x = rand::thread_rng::gen_range(0..((labrinth_size-1)/2-1));
        let y = rand::thread_rng::gen_range(0..((labrinth_size-1)/2-1));
        let wall_x_position = x*2;
        let wall_y_position = y*2;
        if grid[wall_x_position][wall_y_position] == 1 {
            // bore_wall()
            // unicise numbers()
        }
    }
}

fn labrinth_finished (labrinth:Vec<Vec<usize>>) -> bool {
    let labrinth_size = labrinth.len()
    for i in 0..labrinth_size {

    }
}

fn main() {
    
    const LABRINTH_SIZE: usize = 11; // MUST be odd and greater than 4 for the program to function correctly

    let base_grid = grid_builder(LABRINTH_SIZE);
    
    print_labrinth(base_grid);
}