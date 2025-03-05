## RandomX IronLight lib standalone example

This binary runs, proves and prints out the results of IronLight, that is RandomX light mode written in Rust. 
[Here](https://github.com/fluencelabs/randomx-rust-wrapper/tree/randomx-plonky3-sp1) is the IronLight library repo. 


## Usage example
```
SP1_DEV=1 RAYON_NUM_THREADS=2 FRI_QUERIES=1 RUST_LOG=info RUSTFLAGS='-C target-cpu=native' cargo run --release
cp -rp ~/.sp1/circuits/dev/* ~/.sp1/circuits/groth16/v3.0.0/
RAYON_NUM_THREADS=2 FRI_QUERIES=1 RUST_LOG=info RUSTFLAGS='-C target-cpu=native' cargo run --release
```