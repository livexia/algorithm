# Try to finish Blind 75 LeetCode Questions

Run `cargo test test_{id}` to run the solution for "question #id" 

Run `cargo run {id} {name}` to gen the solution template for "question #id" 

- at `./src/solution/s{id:0>4}_{name}.rs` and 
- insert `mod s{id:0>4}_{name};` to the file `./src/solution/mod.rs`

and gen the solution file for python

- at `./python/s{id:0>4}_{name}.py`

Question list: https://leetcode.com/discuss/general-discussion/460599/blind-75-leetcode-questions

Leetcode Template: https://github.com/aylei/leetcode-rust

## ToDo

- [x] Run `cargo run {id} {name}` will generate the template
- [ ] auto fetech the problem link and description
- [ ] Generate a list of completed questions
