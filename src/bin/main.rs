extern crate chrono;
extern crate donchain;

use donchain::block::{Block, HASH_SIZE};


fn main() {
    let dt = chrono::NaiveDateTime::new(
        chrono::NaiveDate::from_ymd(2018, 9, 2),
        chrono::NaiveTime::from_hms(7, 36, 55));

    let block = Block::from_slice(
        0,
        [0; HASH_SIZE],
        // TODO: update to use safe `TryInto` when it finally becomes stable
        dt.timestamp() as u64,
        b"This is the genesis block of donchain.");

    print!("{}", block);
}
