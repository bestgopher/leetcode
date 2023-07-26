#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

struct UndergroundSystem {
    /// 乘客进站的时间记录
    check_in: std::collections::HashMap<i32, (String, i32)>,

    /// 记录站到站花费的总时间和总人次
    station: std::collections::HashMap<(String, String), (i32, i32)>,
}

// /**
//  * `&self` means the method takes an immutable reference.
//  * If you need a mutable reference, change it to `&mut self` instead.
//  */
impl UndergroundSystem {
    fn new() -> Self {
        Self {
            check_in: Default::default(),
            station: Default::default(),
        }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.check_in.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let (start_station, start) = self.check_in.remove(&id).unwrap();
        self.station
            .entry((start_station, station_name))
            .and_modify(|x| {
                x.0 += t - start;
                x.1 += 1;
            })
            .or_insert((t - start, 1));
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let x = self.station.get(&(start_station, end_station)).unwrap();

        x.0 as f64 / x.1 as f64
    }
}

// /**
//  * Your UndergroundSystem object will be instantiated and called as such:
//  * let obj = UndergroundSystem::new();
//  * obj.check_in(id, stationName, t);
//  * obj.check_out(id, stationName, t);
//  * let ret_3: f64 = obj.get_average_time(startStation, endStation);
//  */
