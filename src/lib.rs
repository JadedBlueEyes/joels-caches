//! This crate contains custom caches used in some of my projects.
//!
//! [expiring_cache] contains a cache that stores data along with a creation time (Instant) and life time (Duration). If the creation time is more than the life time, it has the status Expired, otherwise it is Found or NotFound.

pub mod expiring_cache;


pub use expiring_cache::ExpiringCache;