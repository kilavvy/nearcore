use near_primitives::transaction::SignedTransaction;

pub struct TxGenerator {}

impl TxGenerator {
    pub async fn produce() -> SignedTransaction {
        panic!("not implemented");
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
