
use rand::{Rng, StdRng};

use std::time::Duration;
use std::io::{Result, Write};
use std::fs::OpenOptions;

//random a value, which between two different
//time duration,  it's used for request peer to vote
//after a random period to avoid conflict
pub fn after_between(low: Duration, high: Duration) -> Duration {

    let mut rand = StdRng::new().unwrap();
    let value = rand.gen_range(low.as_secs(), high.as_secs());

    println!("rand value: {:?}", value);
    low + Duration::from_secs(value - low.as_secs())
}

//create new or open a file,
//write the data into the file.
//sync to disk
pub fn write_and_sync_file(file_name: &str, data: &[u8]) -> Result<()> {

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_name)?;

    file.write_all(data)?;
    file.sync_all()?;

    Ok(())
}


#[cfg(test)]
mod test {
    use util;
    use std::time::Duration;

    #[test]
    fn test_write() {
        const FILE: &str = "./a.txt";
        const CONTENT: &[u8] = b"hello world";

        util::write_and_sync_file(FILE, CONTENT).unwrap();
        if let Err(e) = util::write_and_sync_file(FILE, CONTENT) {
            println!("{:?}", e);
            panic!("{:?}", e);
        }
    }

    #[test]
    fn test_after_between() {

        let low = Duration::from_secs(10);
        let high = Duration::from_secs(100);

        for _ in 0..10990 {
            let value = util::after_between(low, high);
            println!("{:?} {:?} {:?}", value, low, high);
            if value < low || value >= high {

                panic!("wrong randomize");
            }
        }
    }
}
