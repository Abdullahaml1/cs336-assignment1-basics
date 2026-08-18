use boltffi::*;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

#[export]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

fn find_chunk_boundries_rs(
    file: &mut File,
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
    let mut buffer: [u8; 4096] = [0; 4096]; // a buffer in stack fast
    for bi in 1..chunk_boundries.len() - 1 {
        let mut initial_position = chunk_boundries[bi];
        file.seek(SeekFrom::Start(initial_position))
            .expect("Could not seek file with boundary");
        loop {
            let num_bytes_read = file.read(&mut buffer).expect("Error reading the buffer");

            // EOF
            if num_bytes_read == 0 {
                break;
            }

            // searching the position of special_token
            let idx_opt = buffer[..num_bytes_read]
                .windows(split_special_token.len())
                .position(|w| w == split_special_token);
            if let Some(idx) = idx_opt {
                // update the index of the indices vector
                chunk_boundries[bi] = idx as u64 + initial_position;
                break;
            }

            // it is not granuteed to return exaclly 4096 bytes
            initial_position += num_bytes_read as u64;
        }
    }

    chunk_boundries.sort_unstable(); // sorting unstable fast
    chunk_boundries.dedup(); // droping duplicate itmes
    chunk_boundries
}
#[export]
pub fn find_chunk_boundries(
    file_path: &str,
    desired_num_chunks: usize,
    split_special_token: &[u8],
) -> Vec<u64> {
    let mut file = File::open(file_path).expect("Can not open the file");
    find_chunk_boundries_rs(&mut file, desired_num_chunks, split_special_token)
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
