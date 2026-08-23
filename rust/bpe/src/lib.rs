use boltffi::*;
use fancy_regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::os::unix::fs::FileExt;
use std::thread;

#[export]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
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

///Reads a file from start and end bytes postion and returns a string
fn read_file(file: &File, start: u64, end: u64) -> String {
    if end < start {
        panic!(
            "reding file {:?} start has to be  end got start: {}, end: {}",
            file, start, end
        );
    }
    let mut offset = start;
    let len = end - start;
    let mut filled = 0;
    let mut buffer: Vec<u8> = vec![0; len as usize];
    while filled < len {
        let num_bytes_read = file
            .read_at(&mut buffer[filled as usize..], offset)
            .expect("Error reading the buffer");
        offset += num_bytes_read as u64;
        filled += num_bytes_read as u64;
    }
    String::from_utf8(buffer).expect("can not")
}

fn get_words_count_worker(file_content: &String, re: &Regex) -> HashMap<String, u64> {
    // using cheap &str in indexing
    let mut w_to_c: HashMap<&str, u64> = HashMap::new();
    for res in re.find_iter(file_content) {
        match res {
            Ok(m) => {
                *w_to_c.entry(m.as_str()).or_insert(0) += 1;
            }
            Err(e) => {
                panic!("Error during matching: {}", e);
            }
        }
    }
    // converting it into String for ouput
    w_to_c
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect()
}

pub fn get_words_count(
    input_path: &str,
    split_special_token: &[u8],
    desired_num_chunks: usize,
) -> HashMap<String, u64> {
    let mut file = File::open(input_path).expect("Can not open the file");
    let boundries = find_chunk_boundries_rs(&mut file, desired_num_chunks, split_special_token);
    let pat = r"(?:[sdmt]|ll|ve|re)| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(?!\S)|\s+";
    let re = Regex::new(pat).expect("Colud no pare the regs");
    let mut word_to_count: HashMap<String, u64> = HashMap::new();

    thread::scope(|s| {
        let mut handlers = Vec::new();
        for (idx, [start, end]) in boundries.array_windows::<2>().enumerate() {
            let file_clone = match file.try_clone() {
                Ok(f) => f,
                Err(e) => {
                    panic!(
                        "Colud not clone file descriptor number: {}. because of {} Try to lower the `desired_num_chunks`",
                        idx, e
                    );
                }
            };
            let re_clone = re.clone();
            // count number of words
            let h = s.spawn(move || {
                // get word counts
                let file_content = read_file(&file_clone, *start, *end);
                get_words_count_worker(&file_content, &re_clone)
            });
            handlers.push(h);
        }

        // collecting results
        for h in handlers {
            match h.join() {
                Ok(w_to_c) => {
                    for (w, c) in &w_to_c {
                        *word_to_count.entry(w.clone()).or_insert(0) += c;
                    }
                }
                Err(e) => {
                    panic!("Conting worker thread panciced with {:?}", e);
                }
            }
        }
    });
    word_to_count
}

pub struct TrainedBPEOut<'a> {
    vocab: Vec<&'a [u8]>,
    merges: Vec<(Vec<u8>, Vec<u8>)>, // TODO: to &'a[u8]
}

pub fn train_bpe<'a>(
    input_path: &str,
    vocab_size: u64,
    special_tokens: &'a [&str],
    split_special_token: &[u8],
    num_workers: usize,
) -> TrainedBPEOut<'a> {
    let mut vocab = Vec::new();
    // add special tokens
    for token in special_tokens {
        vocab.push(token.as_bytes());
    }

    let words_to_counts: HashMap<String, u64> =
        get_words_count(input_path, split_special_token, num_workers);

    // computing merges
    TrainedBPEOut {
        vocab: vocab,
        merges: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
