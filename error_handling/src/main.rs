// READING CONTENT FROM A FILE
// 3 WAYS

use std::fs;
use std::io::{self, Read, Write};

fn main() {
    method_one();
    method_two();
    method_three();
}

// METHOD 1
fn method_one() {
    println!("-----METHOD 1-----");

    let readme: Result<fs::File, io::Error> =
        fs::File::open("/home/sanju/github/rust-programming-language/error_handling/README.md");

    let mut content: String = String::new();

    match readme {
        Ok(mut file) => {
            if let Err(err) = file.read_to_string(&mut content) {
                panic!("ERROR OCCURED: {:?}", err.kind());
            }

            println!("CONTENT READ FROM THE FILE \"README.md\": {content}");
        }
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                let readme = fs::File::create(
                    "/home/sanju/github/rust-programming-language/error_handling/README.md",
                );

                match readme {
                    Ok(mut file) => {
                        if let Err(err) = file.write_all(b"# CHAPTER 9") {
                            panic!(
                                "ERROR OCCURED WHILE WRITING INTO THE FILE: {:?}",
                                err.kind()
                            );
                        }

                        println!("WROTE INTO THE FILE \"README.md\" SUCCESSFULLY!");
                    }
                    Err(err) => {
                        panic!("ERROR OCCURED: {:?}", err.kind());
                    }
                }
            } else {
                panic!("ERROR OCCURED: {:?}", err.kind());
            }
        }
    }
    println!();
}

// METHOD 2
fn method_two() {
    println!("-----METHOD 2-----");

    let readme: Result<String, io::Error> =
        fs::read_to_string("/home/sanju/github/rust-programming-language/error_handling/README.md");

    match readme {
        Ok(content) => println!("CONTENT: {content}"),
        Err(err) => panic!("ERROR OCCURED: {:?}", err.kind()),
    }
    println!();
}

// METHOD 3
fn method_three() {
    println!("-----METHOD 3-----");

    let readme: Result<String, io::Error> =
        fs::read_to_string("/home/sanju/github/rust-programming-language/error_handling/README.md");

    let content = readme.unwrap_or_else(|err| {
        panic!("ERROR OCCURED: {:?}", err.kind());
    });

    println!("CONTENT: {content}");
}
