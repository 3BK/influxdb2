use std::env;

use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use futures::stream;
use influxdb2::models::DataPoint;
use influxdb2::models::Query;
use influxdb2::{Client, FromDataPoint};

#[derive(Debug, FromDataPoint)]
pub struct StockPrice {
   ticker: String,
   value: f64,
   open: f64,
   time: DateTime<FixedOffset>,
}

impl Default for StockPrice {
    fn default() -> Self {
        let now = Utc::now().naive_utc();
        Self {
            ticker: "".to_owned(),
            value: 0.0,
            open: 0.0,
            time: FixedOffset::east_opt(7 * 3600).unwrap().from_utc_datetime(&now),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host    = env::var("INFLUXDB_HOST").unwrap();
    let org     = env::var("INFLUXDB_ORG").unwrap();
    let token   = env::var("INFLUXDB_TOKEN").unwrap();
    let bucket  = env::var("INFLUXDB_BUCKET").unwrap();

    let client = Client::new(host, org, token);


    println!("HealthCheck: {:#?}", client.health().await?);
    let mut pb1 = DataPoint::builder("bar");
    pb1.tag("ticker", "AAPL");
    pb1.field("value", 123.46);

    let mut pb2 = DataPoint::builder("bar");
    pb2.tag("ticker", "GOOG");
    pb2.field("value", 321.09);
    pb2.field("open", 309.2);

    let points: Vec<DataPoint> = vec![ 
        pb1.build()?,
        pb2.build()?,
    ];
    client.write(&bucket, stream::iter(points)).await?;
    let qs = format!("from(bucket: \"{}\") 
      |> range(start: -1w)
   ", bucket);
    let query = Query::new(qs.to_string());

    println!(
        "Query result was: {:#?}", 
        client.query::<StockPrice>(Some(query)).await?
    );

    Ok(())
}

