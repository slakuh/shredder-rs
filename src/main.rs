mod constants;
mod shredder;
use std::env;
use std::io;
use std::path::Path;
use shredder::Shred;


fn main() {
    let arguments = env::args().skip(1);//.len() > 0
    if arguments.len() == 0 {
        return;
    }
    println!("\n\n !!! PROGRAM ĆE NEPOVRATNO OBRISATI DATOTEKE !!!\n");
    println!(" Pritisnite ENTER za brisanje");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    println!("\n Briše...\n");

    let mut shred = Shred::new();//
    let mut is_error = false;
    for argument in arguments {
        let result = shred.shred(Path::new(&argument));
        match result {
            Err(e) => {
                println!(" Error: {}\n {}\n",
                    e,
                    &argument,
                );
                is_error = true;
            }
            _ => (),
        }
    }

    if is_error {
        println!("\n Pritisnite ENTER za izlazak.");
        io::stdin().read_line(&mut input).unwrap();
    }
}
