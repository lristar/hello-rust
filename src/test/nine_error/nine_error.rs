use rand::Error;
use std::fs;
use std::fs::File;
use std::io;
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
   let s = read_text_from_file( "hello.txt".to_string());
   match s {
     Ok(s) => {
         println!("content is {}",s)
     }
     Err(e) => {
         println!("error hahaha");
         println!("err is {:#?}",e);
     }
 }
}

// ? is must  have  
fn read_text_from_file(path: String) -> io::Result<String> {
        let mut f = File::open(path)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
     }



    


    // let mut str = String::new();
    // let  result =  f.read_to_string(&mut str).unwrap_or_else(|err| ErrorKind::NotFound) {

    //  }





//fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
//where
//    P: AsRef<Path>,
//{
//    let file = File::open(filename)?;
//    Ok(io::BufReader::new(file).lines())
//}
