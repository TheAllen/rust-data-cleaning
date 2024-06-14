use chrono::NaiveDate;
use serde::de::{Deserialize, Deserializer};


/**
 * An inline function is a function whose definition is expanded at every point of call
 * rather than being invoked through a normal function call. This can reduce the overhead of function calls
 * and potentially improve performance, especially for small, frequently called functions.
 * The decision to actually inline the function is typically made by the compiler.
 */
#[inline]
fn split_date(date_str: &str) -> Result<Vec<&str>, &'static str> {
    Ok(date_str.split(" ").collect::<Vec<&str>>())
}

#[inline]
fn parse_day_str(day_str: &str) -> Result<u32, &'static str> {
    let day_str = &day_str[..day_str.len()-2];
    Ok(day_str.parse::<u32>().map_err(|_| "Invalid day")?)
}

#[inline]
fn parse_year_str(year_str: &str) -> Result<i32, &'static str> {
    Ok(year_str.parse::<i32>().map_err(|_| "Invalid year")?)
}

#[derive(Debug)]
#[repr(u32)]
enum RawMonth {
    Jan = 1,
    Feb = 2,
    Mar = 3,
    Apr = 4,
    May = 5,
    Jun = 6,
    Jul = 7,
    Aug = 8,
    Sept = 9,
    Oct = 10,
    Nov = 11,
    Dec = 12
}

impl std::str::FromStr for RawMonth {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "January" => Ok(RawMonth::Jan),
            "Febuary" => Ok(RawMonth::Feb),
            "March" => Ok(RawMonth::Mar),
            "April" => Ok(RawMonth::Apr),
            "May" => Ok(RawMonth::May),
            "June" => Ok(RawMonth::Jun),
            "July" => Ok(RawMonth::Jul),
            "August" => Ok(RawMonth::Aug),
            "September" => Ok(RawMonth::Sept),
            "October" => Ok(RawMonth::Oct),
            "November" => Ok(RawMonth::Nov),
            "December" => Ok(RawMonth::Dec),
            _ => Ok(RawMonth::Jan)
        }
    }
}

impl From<RawMonth> for u32 {
    fn from(month : RawMonth) -> Self {
        match month {
            RawMonth::Jan => 1,
            RawMonth::Feb => 2,
            RawMonth::Mar => 3,
            RawMonth::Apr => 4,
            RawMonth::May => 5,
            RawMonth::Jun => 6,
            RawMonth::Jul => 7,
            RawMonth::Aug => 8,
            RawMonth::Sept => 9,
            RawMonth::Oct => 10,
            RawMonth::Nov => 11,
            RawMonth::Dec => 12,
        }
    }
}

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

#[cfg(test)]
mod tests {

    use super::*;
    const DATE: &str = "11th November 2023";

    #[test]
    fn test_split_date() {
        // let date = "11th November 2023";
        dbg!(split_date(DATE));
    }

    #[test]
    fn test_parse_month() {
        // let date = "11th November 2023";
        let date_vec = split_date(DATE).unwrap_or(vec![]);
        let mar_num: u32 = date_vec[1].parse::<RawMonth>().unwrap().into();
        println!("{:?}",mar_num);
    }

    #[test]
    fn test_parse_day() {
        let day = "2nd";
        dbg!(parse_day_str(day).unwrap());
    }

    #[test]
    fn test_parse_year() {
        let year = "1995";
        dbg!(parse_year_str(year).unwrap());
    }

    #[test]
    fn test_convert_naive_datetime() {
        let split_date = split_date(DATE).unwrap();
        dbg!(convert_date(split_date).unwrap());
    }
}