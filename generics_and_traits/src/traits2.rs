#![allow(dead_code)]

use indoc::formatdoc;

struct NewsArticle {
    author: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) {
        print!(
            "{}",
            formatdoc!(
                "
                Username: {}
                Content: {}
                ",
                self.username,
                self.content
            )
        );
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) {
        print!(
            "{}",
            formatdoc!(
                "
                Author: {}
                Content: {}
                ",
                self.author,
                self.content
            )
        );
    }
}

trait Summary {
    fn summarize(&self) {
        println!("Not implemented");
    }
}

fn main() {
    let t1 = Tweet {
        username: String::from("iamsanjusabu"),
        content: String::from("Penguins are the best"),
    };

    let na1 = NewsArticle {
        author: String::from("sanjusabu"),
        content: String::from("Penguins are the best"),
    };

    t1.summarize();
    na1.summarize();
}
