use bpe::find_chunk_boundries;
use std::fs::File;

fn main() {
    let mut file = File::open("../../AGENTS.md").unwrap();
    dbg!(&file);

    let special_tokens = [1, 2, 3];
    let out_vec = find_chunk_boundries(&mut file, 4, &special_tokens);
    dbg!(&out_vec);

    let arr: [i32; 10] = std::array::from_fn(|i| i as i32);
    let v: Vec<&[i32]> = arr.windows(3).collect();
    dbg!(&v);
    let found_at = arr.windows(3).position(|w| w == [3, 4, 5]);
    dbg!(&found_at);

    let mut v = vec![1, 2, 3, 4];
    let mut val = v[3];
    dbg!(val);
    v[3] = 10;
    val = 0;
    dbg!(v);
    dbg!(val);
}
