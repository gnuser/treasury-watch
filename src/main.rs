mod config;

use config::Thresholds;

/// Paper-mode loop: watch -> simulate -> log only.
/// Live execution is intentionally not wired; see docs.
fn main() {
    let t = Thresholds::default();
    println!(
        "treasury-watch paper mode: hf_watch={} hf_act={} min_net_usd={}",
        t.hf_watch, t.hf_act, t.min_net_usd
    );
    // TODO: watch.rs (poll positions + events)
    // TODO: sim.rs (eth_call liquidation simulation)
    // TODO: metrics.rs (accounting + alerts)
}
