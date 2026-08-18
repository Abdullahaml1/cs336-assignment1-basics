from time import perf_counter_ns

from bpe import add, find_chunk_boundries

from cs336_basics.pretokenization_example import find_chunk_boundaries as find_chunk_boundries_py

if __name__ == "__main__":
    print(add(3, 4))
    file_path = "./data/owt_valid.txt"
    num_chunks = 64
    start = perf_counter_ns()
    rust_out = find_chunk_boundries(
        file_path=file_path,
        desired_num_chunks=num_chunks,
        split_special_token=b"<|endoftext|>",
    )
    print(rust_out)
    end = perf_counter_ns()
    rust_time = end - start
    print(f"Rust: {rust_time} ns")

    start = perf_counter_ns()
    python_out = find_chunk_boundries_py(
        file=open(file_path, "rb"),
        desired_num_chunks=num_chunks,
        split_special_token=b"<|endoftext|>",
    )
    print(python_out)
    end = perf_counter_ns()
    python_time = end - start
    print(f"Python: {python_time}")

    print(f"rust/python= {rust_time / python_time}")
    assert rust_out == python_out
