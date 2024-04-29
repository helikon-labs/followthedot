use ftd_config::Config;
use lazy_static::lazy_static;

pub mod graph;
pub mod relational;

const REDENOMINATION_BLOCK_NUMBER: u64 = 1_205_128;

lazy_static! {
    static ref CONFIG: Config = Config::default();
}
