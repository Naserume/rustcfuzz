fn main() {
    let n: usize = 1000;
    let time: u128 = 1_000_000;
    let data_len: usize = 100_000;
    let throughput = {
        let total = n as u128 * data_len as u128 * 4;
        let time_sec = time as f64 / 1e9;
        let gib = u128::pow(1024, 3);
        total as f64 / gib as f64 / time_sec
    };
    dbg!(throughput);
}
