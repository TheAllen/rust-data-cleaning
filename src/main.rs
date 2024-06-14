use chrono::NaiveDate;
use sentiment::sentiment::get_sentiment_map;
use serde::{Deserialize, Serialize};

mod convertors;
mod sentiment;

use convertors::{convertors::deserialize_date, review_sentiments::{check_sentiment, split_review_str}};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Review {
    #[serde(rename(deserialize = "Airline Name", serialize = "Airline Name"))]
    airline_name: String,
    #[serde(rename(deserialize = "Overall_Rating", serialize = "Overall Rating"))]
    overall_rating: String,
    #[serde(skip_deserializing)]
    sentiment: i8,
    #[serde(rename(deserialize = "Review_Title", serialize = "Review Title"))]
    review_title: String,
    #[serde(rename(deserialize = "Review Date", serialize = "Review Date"), deserialize_with = "deserialize_date")]
    review_date: NaiveDate,
    #[serde(rename(deserialize = "Verified", serialize = "Verified"))]
    verified: String,
    #[serde(rename(deserialize = "Review", serialize = "Review"))]
    review: String,
    #[serde(rename(deserialize = "Aircraft", serialize = "Aircraft"))]
    aircraft: String,
    #[serde(rename(deserialize = "Type Of Traveller"))]
    type_of_traveller: String,
    #[serde(rename(deserialize = "Seat Type"))]
    seat_type: String,
    #[serde(rename(deserialize = "Route"))]
    route: String,
    #[serde(rename(deserialize = "Date Flown"))]
    date_flown: String,
    #[serde(rename(deserialize = "Seat Comfort"))]
    seat_comfort: String,
    #[serde(rename(deserialize = "Cabin Staff Service"))]
    cabin_staff_service: String,
    #[serde(rename(deserialize = "Food & Beverages"))]
    food_and_beverages: String,
    #[serde(rename(deserialize = "Ground Service"))]
    ground_service: String,
    #[serde(rename(deserialize = "Inflight Entertainment"))]
    inflight_entertainment: String,
    #[serde(rename(deserialize = "Wifi & Connectivity"))]
    wifi_and_connectivity: String,
    #[serde(rename(deserialize = "Value For Money"))]
    value_and_money: String,
    #[serde(rename(deserialize = "Recommended"))]
    recommended: String
}

fn read_reviews_csv() -> Vec<Review>{
    // Read a CSV file
    let mut csv_reader = csv::Reader::from_path("Airline_review.csv").unwrap();
    let mut review_rows = csv_reader
        .deserialize()
        .map(|item| {
            item.ok().expect("Can not be unwrapped")
        })
        .collect::<Vec<Review>>();

    // println!("{:?}", review_rows);
    review_rows
}

async fn write_reviews_csv(reviews: &mut [Review]) {
    let mut csv_writer = csv::Writer::from_path("Airline_review_clean.csv").unwrap();
    let sentiment_map = get_sentiment_map().await.expect("Could not load sentiment map");
    for r in reviews.iter_mut() {

        r.review_title = trim_extra_quotes(&r.review_title);
        r.review = trim_extra_quotes(&r.review);

        // Checking sentiment
        let review_vec = split_review_str(&r.review);
        let sentiment_score = check_sentiment(review_vec, &sentiment_map);
        r.sentiment = sentiment_score;
        csv_writer.serialize(r);
    }

    csv_writer.flush();
}

/**
 * fn write_reviews_csv(reviews: &mut [Review]) -> Result<(), Box<dyn Error>> {
    let mut csv_writer = Writer::from_path("Airline_review_clean.csv")?;
    
    for r in reviews.iter_mut() {
        r.review = trim_extra_quotes(&r.review);
        csv_writer.serialize(r)?;
    }

    csv_writer.flush()?; // Ensure all data is written to the file
    Ok(())
}
 */


fn trim_extra_quotes(s: &str) -> String {
    let mut trimmed = s.trim();
    
    while trimmed.starts_with('\"') && trimmed.ends_with('\"') {
        trimmed = trimmed.strip_prefix('\"').unwrap_or(trimmed);
        trimmed = trimmed.strip_suffix('\"').unwrap_or(trimmed);
        trimmed = trimmed.trim();
    }
    
    while trimmed.starts_with('\'') && trimmed.ends_with('\'') {
        trimmed = trimmed.strip_prefix('\'').unwrap_or(trimmed);
        trimmed = trimmed.strip_suffix('\'').unwrap_or(trimmed);
        trimmed = trimmed.trim();
    }

    trimmed.to_owned()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("Airline Reviews");

    let mut data = read_reviews_csv();
    write_reviews_csv(&mut data).await;

    Ok(())
}



#[cfg(test)]
mod tests {

    use super::*;
    #[tokio::test]
    async fn test_csv() {
        let mut data = read_reviews_csv();
        write_reviews_csv(&mut data).await;
    }

    #[test]
    fn test_trim_extra_quote() {
        let extra = r#"""Something"""#;
        let s = trim_extra_quotes(extra);

        dbg!(s);
    }

}
