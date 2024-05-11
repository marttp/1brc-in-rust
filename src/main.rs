use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    // Create HashMap to store data as in-memory
    let mut station_info_table: HashMap<String, StationInfo> = HashMap::new();
    for l in read_to_string("weather_stations.csv").unwrap().lines() {
        if let Some((station, temperature)) = l.split_once(";") {
            let station = station.to_string();
            let temperature: f64 = temperature.parse().unwrap();
            let data = station_info_table.entry(station.clone())
                .or_insert(StationInfo::new(station.clone()));
            // Update data if needed
            data.amount = data.amount + 1;
            data.sum = data.sum + temperature;
            data.min = f64::min(data.min, temperature);
            data.max = f64::max(data.max, temperature);
        }
    }
    // Show the result
    for (_, station_info) in station_info_table {
        println!("{}", station_info.info())
    }
    let elapsed_time = start_time.elapsed();
    println!("Use {:?} to read file", elapsed_time);
}

struct StationInfo {
    station: String,
    min: f64,
    max: f64,
    sum: f64,
    amount: usize
}

impl StationInfo {

    fn new(station: String) -> Self {
        StationInfo {
            station,
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            sum: 0.0,
            amount: 0,
        }
    }

    fn mean(&self) -> f64 {
        self.sum / self.amount as f64
    }

    fn info(&self) -> String {
        format!("{}={:.1}/{:.1}/{:.1}", &self.station.clone(), &self.min, &self.mean(), &self.max)
    }
}