use rand::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

#[test]
fn test_hahaha() {
    //    let f = File::open("hello.txt");
    //
    //    let f = match f {
    //        Ok(file) => file,
    //        Err(error) => match error.kind() {
    //            ErrorKind::NotFound => match File::create("hello.txt") {
    //                Ok(fc) => fc,
    //                Err(e) => panic!("Problem creating the file: {:?}", e),
    //            },
    //            other_error => panic!("Problem opening the file: {:?}", other_error),
    //        },
    //    };
    //    let f = File::open("hello.txt").unwrap_or_else(|error| {
    //        if error.kind() == ErrorKind::NotFound {
    //            File::create("hello.txt").unwrap_or_else(|error| {
    //                panic!("Problem creating the file: {:?}", error);
    //            })
    //        } else {
    //            panic!("Problem opening the file: {:?}", error);
    //        }
    //    });
    //    println!("file is {:#?}", f);
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => {
            panic!("Problem opening the file: {:?}", e)
        }
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(s) => {
            println!("s:{}", s);
        }
        Err(e) => {
            panic!("Problem read the file: {:?}", e)
        }
    }
}

//fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
//where
//    P: AsRef<Path>,
//{
//    let file = File::open(filename)?;
//    Ok(io::BufReader::new(file).lines())
//}
