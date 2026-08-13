use bpe::find_chunk_boundries;
use std::fs::File;

fn main() {
    let file = File::open("../../AGENTS.md").unwrap();
    dbg!(&file);

    let special_tokens = [1, 2, 3];
    let out_vec = find_chunk_boundries(&file, 4, &special_tokens);
    dbg!(&out_vec);
}
