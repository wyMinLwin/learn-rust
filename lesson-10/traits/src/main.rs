use aggregator::{Summary, Tweet, NewsArticle, notify};

fn main() {
    let tweet = Tweet {
        username: String::from("armin_arlert"),
        content: String::from(
            "Sasageyo! Sasageyo! Shinzou wo sasageyo! Subete no gisei wa ima kono toki no tame ni!",
        ),
        reply: false,
        retweet: false,
    };

    let news_article = NewsArticle {
        headline: String::from("The Attack Titan"),
        location: String::from("Paradis Island"),
        author: String::from("Hajime Isayama"),
        content: String::from(
            "The Attack Titan is a manga series written and illustrated by Hajime Isayama.",
        ),
    };

    println!("1 new article: {}", news_article.summarize());
    println!("1 new tweet: {}", tweet.summarize());

    notify(&news_article);
}
