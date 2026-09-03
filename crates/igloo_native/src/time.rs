//! Time-based subscriptions provided by the iced host.
/// Emits epoch milliseconds at each iced timer tick, matching the WASM guest API.
pub fn every(interval: std::time::Duration) -> crate::Subscription<u64> {
    crate::Subscription::every(interval)
}
