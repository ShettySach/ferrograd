use crate::engine::Value;

/** Binary Cross-Entropy loss
For binary classification, when targets are -1 and 1.*/
#[derive(Debug)]
pub struct BinaryCrossEntropyLoss;

impl BinaryCrossEntropyLoss {
    pub fn new() -> BinaryCrossEntropyLoss {
        BinaryCrossEntropyLoss
    }

    pub fn loss(&self, ypred: &[Vec<Value>], ytrue: &[Vec<Value>]) -> Value {
        -ypred
            .iter()
            .zip(ytrue)
            .map(|(ypred_i, ytrue_i)| {
                ypred_i
                    .iter()
                    .zip(ytrue_i)
                    .map(|(ypred_j, ytrue_j)| {
                        ytrue_j * ypred_j.ln()
                            + (1.0 - ytrue_j) * (1.0 - ypred_j).ln()
                    })
                    .sum::<Value>()
            })
            .sum::<Value>()
            / (ypred.len() * ypred[0].len()) as f64
    }
}

impl Default for BinaryCrossEntropyLoss {
    fn default() -> Self {
        Self::new()
    }
}
