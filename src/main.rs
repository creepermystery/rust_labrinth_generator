fn main() {

    fn print_labrinth(labrinth: Vec<Vec<u8>>, labrinth_size: usize) {
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
    
    const LABRINTH_SIZE: usize = 11; // MUST be odd and greater than 4 for the program to function correctly
    const INTERNAL_SIZE: usize = LABRINTH_SIZE-2;

    let mut labrinth: Vec<Vec<u8>> = Vec::new();
    let full_collum: Vec<u8> = Vec::from([1u8; LABRINTH_SIZE]);

    labrinth.push(full_collum.clone());

    for i in 0..INTERNAL_SIZE {
        let mut collum: Vec<u8> = Vec::new();
        if i%2 == 1 {
            labrinth.push(full_collum.clone());
        }
        else {
            for l in 0..LABRINTH_SIZE {
                if l%2 == 0 {
                    collum.push(1);
                }
                else {
                    collum.push(0);
                }
            }
            labrinth.push(collum);
        }
    }
    labrinth.push(full_collum);

    print_labrinth(labrinth, LABRINTH_SIZE);

    
}