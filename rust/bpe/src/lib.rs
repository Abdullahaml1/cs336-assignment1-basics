use boltffi::*;
use std::collections::HashMap;
use std::path::Path;

#[export]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// struct TrainedBPEOut {
//     vocab: HashMap<u64, Bytes>,
//     merges: Vec<(bytes, Bytes)>,
// }
//
// pub fn train_bpe(input_path: Path, vocab_size: u64, speial_tokens: &[&str]) -> TrainedBPEOut {
//
//     // append special tokens in the vocab but not in the merges execpt for the
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
