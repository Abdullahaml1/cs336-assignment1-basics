use boltffi::*;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

#[export]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn find_chunk_boundries(
    file: &File,
    desired_num_chunks: usize,
    split_special_token: &[u8],
) -> Vec<u64> {
    let file_size = file.metadata().unwrap().len();
    let chunk_size = file_size / desired_num_chunks as u64;

    // initial guess of chunk_boundries uniform
    let mut chunk_boundries: Vec<u64> = (0..desired_num_chunks)
        .map(|idx| idx as u64 * chunk_size)
        .collect();
    chunk_boundries.push(file_size);

    println!("chunk_boundary");
    let mini_chunk_size = 4096; // read ahead a mini chunk (small chunk) this no minimum

    vec![1, 2, 3]
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
