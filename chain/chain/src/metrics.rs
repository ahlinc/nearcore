use near_metrics::{
    try_create_histogram, try_create_int_counter, try_create_int_gauge, Histogram, IntCounter,
    IntGauge,
};
use once_cell::sync::Lazy;

pub static BLOCK_PROCESSING_ATTEMPTS_TOTAL: Lazy<IntCounter> = Lazy::new(|| {
    try_create_int_counter(
        "near_block_processing_attempts_total",
        "Total number of block processing attempts. The most common reason for aborting block processing is missing chunks",
    )
    .unwrap()
});
pub static BLOCK_PROCESSED_TOTAL: Lazy<IntCounter> = Lazy::new(|| {
    try_create_int_counter("near_block_processed_total", "Total number of blocks processed")
        .unwrap()
});
pub static BLOCK_PROCESSING_TIME: Lazy<Histogram> = Lazy::new(|| {
    try_create_histogram("near_block_processing_time", "Time taken to process blocks successfully. Measures only the time taken by the successful attempts of block processing")
        .unwrap()
});
pub static BLOCK_HEIGHT_HEAD: Lazy<IntGauge> = Lazy::new(|| {
    try_create_int_gauge("near_block_height_head", "Height of the current head of the blockchain")
        .unwrap()
});
pub static VALIDATOR_AMOUNT_STAKED: Lazy<IntGauge> = Lazy::new(|| {
    try_create_int_gauge(
        "near_validators_stake_total",
        "The total stake of all active validators during the last block",
    )
    .unwrap()
});
pub static VALIDATOR_ACTIVE_TOTAL: Lazy<IntGauge> = Lazy::new(|| {
    try_create_int_gauge(
        "near_validator_active_total",
        "The total number of validators active after last block",
    )
    .unwrap()
});
pub static NUM_ORPHANS: Lazy<IntGauge> =
    Lazy::new(|| try_create_int_gauge("near_num_orphans", "Number of orphan blocks.").unwrap());
pub static HEADER_HEAD_HEIGHT: Lazy<IntGauge> = Lazy::new(|| {
    try_create_int_gauge("near_header_head_height", "Height of the header head").unwrap()
});

pub static GC_TAIL: Lazy<IntGauge> =
    Lazy::new(|| try_create_int_gauge("near_gc_tail_height", "Tail height for GC").unwrap());
pub static GC_STOP_HEIGHT: Lazy<IntGauge> =
    Lazy::new(|| try_create_int_gauge("near_gc_stop_height", "Stop height for GC").unwrap());
