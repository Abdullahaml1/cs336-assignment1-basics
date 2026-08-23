use bpe::{find_chunk_boundries, get_words_count};
use std::fs::File;
use std::time::Instant;

fn main() {
    // let mut file = File::open("../../AGENTS.md").unwrap();
    // dbg!(&file);
    //
    // let special_tokens = [1, 2, 3];
    // let out_vec = find_chunk_boundries("../../AGENTS.md", 4, &special_tokens);
    // dbg!(&out_vec);
    //
    // let arr: [i32; 10] = std::array::from_fn(|i| i as i32);
    // let v: Vec<&[i32]> = arr.windows(3).collect();
    // dbg!(&v);
    // let found_at = arr.windows(3).position(|w| w == [3, 4, 5]);
    // dbg!(&found_at);

    let special_tokens: Vec<u8> = b"<|endoftext|>".to_vec();
    let start = Instant::now();
    let _words_counts = get_words_count("../../data/owt_valid.txt", &special_tokens, 20);
    let duration = start.elapsed();
    // dbg!(words_counts);
    println!("Total time is `{}`", duration.as_millis());
}
