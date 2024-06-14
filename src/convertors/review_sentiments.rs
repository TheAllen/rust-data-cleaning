use std::collections::HashMap;

use crate::sentiment::{self, sentiment::get_sentiment_map};


pub fn split_review_str(review: &str) -> Vec<String> {
    review.split(" ")
        .into_iter()
        .map(|word| {
            word.to_ascii_lowercase()
        })
        .collect()
}

pub fn check_sentiment(
    review_vec: Vec<String>, 
    sentiment_map: &HashMap<String, i8>
) -> i8 {
    // let sentiment_map = get_sentiment_map().await.expect("Could not load sentiment map");

    let mut sentiment_score = 0;
    let mut count: i8 = 0;
    for word in review_vec.iter() {
        if sentiment_map.contains_key(word) {
            count += 1;
            sentiment_score += sentiment_map.get(word).unwrap();
        }
    }

    if count == 0 { 0 } else { sentiment_score / count }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_split_review_string() {
        let sentence = "Moroni to Moheli. Turned out to be a pretty decent airline. Online booking worked well, checkin and boarding was fine and the plane looked well maintained. Its a very short flight - just 20 minutes or so so i didn't expect much but they still managed to hand our a bottle of water and some biscuits which i though was very nice. Both flights on time.";
        dbg!(split_review_str(sentence));
    }

    #[tokio::test]
    async fn test_check_sentiment() {
        let sentence = "Moroni to Moheli. Turned out to be a pretty decent airline. Online booking worked well, checkin and boarding was fine and the plane looked well maintained. Its a very short flight - just 20 minutes or so so i didn't expect much but they still managed to hand our a bottle of water and some biscuits which i though was very nice. Both flights on time.";
        let sentiment_map = get_sentiment_map().await.unwrap();
        let review_vec = split_review_str(&sentence);
        dbg!(check_sentiment(review_vec, &sentiment_map));

    }

}