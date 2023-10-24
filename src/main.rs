fn main() {

    fn print_labrinth(labrinth: Vec<Vec<u8>>, labrinth_size: usize) {
        for k in 0..labrinth_size-1 {
            for j in 0..labrinth_size-1 {
                print!("{}", labrinth[j][k]);
                print!(" ");
            }
            println!("");
        }

        println!("{}", labrinth[0][0]);        
    }
    
    const LABRINTH_SIZE: usize = 11; // MUST be odd and greater than 4 for the program to function correctly
    const INTERNAL_SIZE: usize = LABRINTH_SIZE-2;

    let mut labrinth: Vec<Vec<u8>> = Vec::new();
    let end_collums: Vec<u8> = Vec::from([1u8; LABRINTH_SIZE]);

    labrinth.push(end_collums.clone());

    for i in 0..INTERNAL_SIZE {
        let mut collum: Vec<u8> = Vec::new();
        if i%2 == 0 {
            collum.push(1);
        }
        else {
            collum.push(0);
        }
        labrinth.push(collum);
    }
    labrinth.push(end_collums);

    print_labrinth(labrinth, LABRINTH_SIZE);
}