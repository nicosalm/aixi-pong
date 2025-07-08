pub struct CTW {
    // context tree and weighting implementation
}

impl CTW {
    pub fn new() -> Self {
        CTW {}
    }

    pub fn update(&mut self, symbol: u8) {
        // update tree with new symbol
    }

    pub fn predict(&self) -> f64 {
        0.5 // uniform probability for now
    }
}
