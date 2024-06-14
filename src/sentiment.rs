

pub mod sentiment {
    use std::{collections::HashMap, hash::Hash};
    use reqwest::{self, Client, Response};


    async fn get_sentiment() -> Result<String, Box<dyn std::error::Error + Send>> {
        let github_link = "https://raw.githubusercontent.com/fnielsen/afinn/master/afinn/data/AFINN-111.txt";
        let client = Client::builder()
            .build()
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

        let res_raw = client
            .get(github_link)
            .send()
            .await
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
            .text()
            .await
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;
        Ok(res_raw)
    }

    fn parse_string_to_hashmap(string: &str) -> HashMap<String, i8> {
        let sentiment_map = string
            .lines()
            .filter_map(|line| {
                let mut parts = line.split('\t');
                if let (Some(word), Some(score)) = (parts.next(), parts.next()) {
                    if let Ok(score) = score.parse::<i8>() {  // Convert score to i8
                        return Some((word.to_string(), score));  // Collect as tuple
                    }
                }
                None
            })
            .collect::<HashMap<String, i8>>();

        sentiment_map
    }    

    pub async fn get_sentiment_map() -> Option<HashMap<String, i8>> {
        let sentiment_res = get_sentiment().await;
        if let Ok(res) = sentiment_res {
            return Some(parse_string_to_hashmap(&res));
        } else{
            return None;
        }
    }
    
    #[cfg(test)]
    mod tests {

        use super::*;

        #[tokio::test]
        async fn test_get_sentiment() {
            let res = get_sentiment().await.unwrap();
            dbg!(res);
        }

        #[tokio::test]
        async fn test_parse_to_str_vec() {
            let res = get_sentiment().await.unwrap();
            parse_string_to_hashmap(&res);
        }
    }
}
