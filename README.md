# CSV Data Cleaning Application in Rust

This project is a Rust-based application designed to clean and transform CSV data files. The application reads CSV files containing airline reviews, processes the data to clean it up, adds sentiment analysis to reviews, and writes the transformed data to a new CSV file. The project utilizes the serde library for serialization and deserialization of CSV data and includes a simple sentiment analysis feature.

## Features

- **CSV Data Processing**: Reads data from an input CSV file, processes it to clean and normalize the data, and outputs the cleaned data to a new CSV file.
- **Sentiment Analysis**: Adds a sentiment score to each review based on the content of the review text.
- **Data Transformation**: Transforms the CSV data to a more usable format, changing the structure and ensuring consistency.
- **Output CSV**: The application writes the processed data to an output CSV file with the added sentiment score.

## Example Data
Original Data Format
```bash
Airline Name	Overall Rating	Review Title	Review Date	Verified	Review	Aircraft	Type Of Traveller	Seat Type	Route	Date Flown	Seat Comfort	Cabin Staff Service	Food & Beverages	Ground Service	Inflight Entertainment	Wifi & Connectivity	Value For Money	Recommended
AB Aviation	9	pretty decent airline	11th November 2019	True	Moroni to Moheli. Turned out to be a pretty decent airline. Online booking worked well, checkin and boarding was fine and the plane looked well maintained. Its a very short flight - just 20 minutes or so so I didn't expect much but they still managed to hand our a bottle of water and some biscuits which I thought was very nice. Both flights on time.		Solo Leisure
```

Transformed Data Format
```bash
Airline Name,Overall Rating,sentiment,Review Title,Review Date,Verified,Review,Aircraft,type_of_traveller,seat_type,route,date_flown,seat_comfort,cabin_staff_service,food_and_beverages,ground_service,inflight_entertainment,wifi_and_connectivity,value_and_money,recommended
AB Aviation,9,1,pretty decent airline,2019-11-11,True,"Moroni to Moheli. Turned out to be a pretty decent airline. Online booking worked well, checkin and boarding was fine and the plane looked well maintained. Its a very short flight - just 20 minutes or so so i didn't expect much but they still managed to hand our a bottle of water and some biscuits which i though was very nice. Both flights on time.",,Solo Leisure,Economy Class,Moroni to Moheli,November 2019,4.0,5.0,4.0,4.0,,,3.0,yes
```

## Code Overview
Review object for deserializing and deserializing airline reviews.

```rust
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
```

Converting &str date to NaiveDate

```rust
pub fn convert_date(date_vec: Vec<&str>) -> Result<NaiveDate, &'static str> {
    let year = parse_year_str(date_vec[2]).unwrap();
    let month = date_vec[1].parse::<RawMonth>().unwrap().into();
    let day = parse_day_str(date_vec[0]).unwrap();
    NaiveDate::from_ymd_opt(year, month, day).ok_or("Invalid date")
}

pub fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let date_str: &str = Deserialize::deserialize(deserializer)?;
    let date_vec: Vec<&str> = split_date(date_str).unwrap();
    convert_date(date_vec).map_err(serde::de::Error::custom)
}
```

Assigning sentiment score based on review

```rust
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
```

## Cleaned CSV 
Airline_review_clean.csv
