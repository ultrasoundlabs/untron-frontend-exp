/// Number of decimal places used for the on-chain units. Equivalent to `DEFAULT_DECIMALS` in TS.
pub const DEFAULT_DECIMALS: u32 = 6;

/// Separate constant for rate scaling (matches JS RATE_SCALE).
pub const RATE_SCALE: u64 = 1_000_000; // 10^6

/// Current swap rate (scaled by RATE_SCALE). TODO: keep in sync with backend.
pub const SWAP_RATE_UNITS: u64 = 999_700; // 0.03% fee

// ---- Helper functions -----------------------------------------------------

/// Converts a human-readable decimal string into integer units (no floating-point math).
pub fn string_to_units(value: &str) -> Option<u64> {
    // Split on the optional decimal point.
    let mut parts = value.split('.');
    let whole = parts.next().unwrap_or("");
    let frac = parts.next().unwrap_or("");

    // Reject multiple decimals.
    if parts.next().is_some() {
        return None;
    }

    // Reject more fractional digits than supported.
    if frac.len() > DEFAULT_DECIMALS as usize {
        return None;
    }

    // Build a string representing the integer value scaled by `SCALING_FACTOR`.
    // Pad fractional part on the right with zeros.
    let mut scaled = String::with_capacity(whole.len() + DEFAULT_DECIMALS as usize);
    scaled.push_str(whole.trim_start_matches('0'));
    let padding = DEFAULT_DECIMALS as usize - frac.len();
    scaled.push_str(frac);
    scaled.extend(std::iter::repeat('0').take(padding));

    // Empty string after trimming => value was 0.
    let scaled = if scaled.is_empty() { "0" } else { &scaled };

    scaled.parse::<u64>().ok()
}

/// Converts integer units into a human-readable decimal string.
pub fn units_to_string(units: u64) -> String {
    let mut s = units.to_string();
    if DEFAULT_DECIMALS == 0 {
        return s;
    }

    // Ensure the string has at least DEFAULT_DECIMALS + 1 digits so we can insert the dot.
    if s.len() <= DEFAULT_DECIMALS as usize {
        let prepend = DEFAULT_DECIMALS as usize + 1 - s.len();
        s = "0".repeat(prepend) + &s;
    }

    let idx = s.len() - DEFAULT_DECIMALS as usize;
    let (whole, frac) = s.split_at(idx);

    // Trim trailing zeros from fractional part.
    let frac = frac.trim_end_matches('0');
    if frac.is_empty() {
        whole.to_string()
    } else {
        format!("{whole}.{frac}")
    }
}

/// Calculates the amount of input (send) units required to receive the given `receive_units`.
pub fn convert_receive_to_send(receive_units: u64, swap_rate_units: u64) -> u64 {
    // (receiveUnits * RATE_SCALE + swapRate/2) / swapRate  — rounded to nearest
    receive_units
        .saturating_mul(RATE_SCALE)
        .saturating_add(swap_rate_units / 2)
        / swap_rate_units
}

/// Calculates the amount of output (receive) units obtained from the given `send_units`.
pub fn convert_send_to_receive(send_units: u64, swap_rate_units: u64) -> u64 {
    // (sendUnits * swapRate + RATE_SCALE/2) / RATE_SCALE  — rounded to nearest
    send_units
        .saturating_mul(swap_rate_units)
        .saturating_add(RATE_SCALE / 2)
        / RATE_SCALE
}
