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
Airline Name	Overall Rating	Sentiment	Review Title	Review Date	Verified	Review	Aircraft	Type Of Traveller	Seat Type	Route	Date Flown	Seat Comfort	Cabin Staff Service	Food & Beverages	Ground Service	Inflight Entertainment	Wifi & Connectivity	Value For Money	Recommended
AB Aviation	9	1	pretty decent airline	2019-11-11	True	Moroni to Moheli. Turned out to be a pretty decent airline. Online booking worked well, checkin and boarding was fine and the plane looked well maintained. It's a very short flight - just 20 minutes or so so I didn't expect much but they still managed to hand out a bottle of water and some biscuits which I thought was very nice. Both flights on time.		Solo Leisure	Economy Class	Moroni to Moheli	November 2019	4.0	5.0	4.0	4.0			3.0	yes
'''
