// Include a library from a different repository : ../traits_lib
use traits_lib::{Summary, Tweet, NewsArticle}; // Use external library "traits_lib" from an other repository ../traits_lib


// // Include a library from the same repository : src/lib.rs
// pub mod lib;
// use lib::{Summary, Tweet, NewsArticle}; // Use external library "traits_lib" from an other repository ../traits_lib

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Is Buildroot > Yocto"),
        location: String::from("France"),
        author: String::from("Leo"),
        content: String::from("afjzahfjkdzefhkjzehfjkzhkjfhjkhfkjhkjfhsdkjfhkj"),
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new news article: {}", article.summarize());
}