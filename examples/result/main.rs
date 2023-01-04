// fn panic_attack(){
//     panic!("some message");
// }

// fn main(){
//     panic_attack();
// }

// fn main(){
//     let v = vec![1, 2, 3, 4, 5];
//     println!("Access to an inaccessible item {}", v[5]);
// }

use std::fs::File; // standard library => File system => File object 
use std::io::ErrorKind

fn main(){
    // f : Resule<File, std::io::Error>
    let f = File::open("./examples/result/textfile2.txt");
    let s = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => println!("File not found - hello!");
        },
        _ => panic!("Problem with the file {:?}", error),
    };
    println!("{:?}",s);
}