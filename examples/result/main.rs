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

// use std::fs::File; // standard library => File system => File object 
// use std::io::ErrorKind;

// fn main(){
//     // f : Resule<File, std::io::Error>
//     let filename : &str = "./examples/result/textfile2.txt";
//     let f = File::open(filename);
//     let _s = match f {
//         Ok(file) => file,
//         Err(error) =>
//             match error.kind() {
//                 ErrorKind::NotFound => match File::create(filename){
//                     Ok(fc) => fc,
//                     Err(_e) => panic!("Problem while creating the file")
//                 },
//             other_error =>
//                 panic!("Problem openening the file {:?}", other_error)
//             }
//         };
//     println!("Goodbye!");
// }


use std::fs::File; // standard library => File system => File object
use std::io::{ErrorKind, Read, Write};

fn main(){
    // f : Resule<File, std::io::Error>
    let filename : &str = "./examples/result/textfile.txt";
    let mut f = File::open(filename).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(filename).unwrap()
        } else {
            panic!("Problem while opening the file: {:?}", error)}
        }
    );
    let mut content = String::new();
    let mut content2 = String::new();
    f.read_to_string(&mut content);
    println!("File content: \n {:?}", content);
    f.write_all(b"Hello, world! Hello, world! Hello, world! Hello, world!");
    drop(f);
    let mut d = File::open(filename).unwrap();
    d.read_to_string(&mut content2);
    println!("New File content: \n {:?}", content2);
    //f.close();
    println!("=*= Goodbye! =*=");
}