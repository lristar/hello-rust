use rand::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::num::ParseIntError;

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
    let s = read_text_from_file("hello.txt".to_string());
    match s {
        Ok(s) => {
            println!("content is {}", s)
        }
        Err(e) => {
            println!("error hahaha");
            println!("err is {:#?}", e);
        }
    }
}

// ? is must  have  Result<T>
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

// 使用？解开option

struct Person {
    job: Option<Job>,
}
#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    fn work_phone_area_code(&self) -> Option<u8> {
        // 判断job是不是none，如果不是找phone_number,如果phone_number不为none,则找到area_code属性返回
        self.job?.phone_number?.area_code
    }
}

#[test]
fn test_option_judge() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 15,
            }),
        }),
    };
    assert_eq!(p.work_phone_area_code(), Some(61));
}

// 设置bie'ming
type AliasedResult<T> = Result<T, ParseIntError>;

fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| second_number * first_number)
    })
}

fn printf(result: AliasedResult<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

#[test]
fn test_parse() {
    printf(multiply("1", "2"));
}
