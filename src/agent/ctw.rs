// refer to: FAC-CTW implementation (Section 5)

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ctw_creation() {
        let _ctw = CTW::new();
    }

    #[test]
    fn test_ctw_predict_returns_probability() {
        let ctw = CTW::new();
        let prob = ctw.predict();
        assert!(prob >= 0.0 && prob <= 1.0);
    }

    #[test]
    fn test_ctw_update_doesnt_panic() {
        let mut ctw = CTW::new();
        ctw.update(0);
        ctw.update(255);
        ctw.update(42);
    }
}
