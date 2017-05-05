//! Consenus constants

pub const BLOCK_MAX_FUTURE: i64 = 2 * 60 * 60; // 2 hours
pub const COINBASE_MATURITY: u32 = 100; // 2 hours
// partial reference:
// https://github.com/gandrewstone/BitcoinUnlimited/blob/150c86e427187a71bc17ccd32483c98dc9dbfba2/src/consensus/consensus.h
// differently from the merged code, MAX_BLOCK_SIZE is set to 2Mb. More will be done in successive
// commits
pub const MAX_BLOCK_SIZE: usize = 2_000_000; // 2Mb
pub const MAX_BLOCK_SIGOPS: usize = MAX_BLOCK_SIZE/50; // 40000
pub const MIN_COINBASE_SIZE: usize = 2;
pub const MAX_COINBASE_SIZE: usize = 100;

pub const RETARGETING_FACTOR: u32 = 4;
pub const TARGET_SPACING_SECONDS: u32 = 10 * 60;
pub const DOUBLE_SPACING_SECONDS: u32 = 2 * TARGET_SPACING_SECONDS;
pub const TARGET_TIMESPAN_SECONDS: u32 = 2 * 7 * 24 * 60 * 60;

// The upper and lower bounds for retargeting timespan
pub const MIN_TIMESPAN: u32 = TARGET_TIMESPAN_SECONDS / RETARGETING_FACTOR;
pub const MAX_TIMESPAN: u32 = TARGET_TIMESPAN_SECONDS * RETARGETING_FACTOR;

// Target number of blocks, 2 weaks, 2016
pub const RETARGETING_INTERVAL: u32 = TARGET_TIMESPAN_SECONDS / TARGET_SPACING_SECONDS;
