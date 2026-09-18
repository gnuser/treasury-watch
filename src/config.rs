/// Hard thresholds for the liquidation watcher (paper mode).
/// Nothing here places trades; exec only logs in paper mode.

pub struct Thresholds {
    /// Start watching below this health factor.
    pub hf_watch: f64,
    /// Actionable below this health factor.
    pub hf_act: f64,
    /// Minimum simulated net profit (USD) to consider.
    pub min_net_usd: f64,
    /// Max all-in cost per attempt (USD).
    pub max_tx_cost_usd: f64,
    /// Daily gas budget (USD), kill switch above it.
    pub daily_gas_cap_usd: f64,
    /// Consecutive failures before auto-stop.
    pub max_consecutive_failures: u32,
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            hf_watch: 1.02,
            hf_act: 1.0,
            min_net_usd: 100.0,
            max_tx_cost_usd: 25.0,
            daily_gas_cap_usd: 20.0,
            max_consecutive_failures: 5,
        }
    }
}
