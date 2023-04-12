use futures::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let org = "sahamee";
    let bucket = "bucket";
    let influx_url = "http://localhost:8086";
    let token = std::env::var("INFLUXDB2_TOKEN").unwrap();

    let client = influxdb2::Client::new(influx_url, org, token);

    let mut pb1 = influxdb2::models::DataPoint::builder("cpu_load_short");
    pb1.tag("host", "server01");
    pb1.tag("region", "us-west");
    pb1.field("value", 0.64);

    let mut pb2 = influxdb2::models::DataPoint::builder("cpu_load_short");
    pb2.tag("host", "server01");
    pb2.field("value", 27.99);

    let points = vec![
        pb1.build()?,
        pb2.build()?,
    ];

    client.write(bucket, stream::iter(points)).await?;

    Ok(())
}
