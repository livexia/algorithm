# Try to finish Blind 75 LeetCode Questions

Run `cargo run {id} {name}` to:

- create the solution template for "question #id" 
- at `./src/solution/s{id:0>4}_{name}.rs` and 
- insert `mod s{id:0>4}_{name};` to the file `./src/solution/mod.rs`
- also create the solution file for python
- at `./python/test_s{id:0>4}_{name}.py`

Question list: https://leetcode.com/discuss/general-discussion/460599/blind-75-leetcode-questions

Leetcode Template: https://github.com/aylei/leetcode-rust

## Run Tests

- `make` / `make all`: run all rust and python solutions with tests
- `make quiet`: run all tests quite
- `make python`: run all python solutions with tests
    - same as `pytest .`
- `make rust`: run all rust solutions with tests
    - same as `cargo t`
- `cargo test test_{id}`: run the solution for "question #id" 
- `python -m unittest ./python/test_s{id:0>4}_{name}.py`: run the solution for "question #id" 
- `pytest ./python/test_s{id:0>4}_{name}.py`: run the solution for "question #id" 

## ToDo

- [x] Run `cargo run {id} {name}` will generate the template
- [ ] auto fetech the problem link and description
- [ ] Generate a list of completed questions
