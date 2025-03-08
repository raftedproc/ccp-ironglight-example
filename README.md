## RandomX IronLight lib standalone example

This binary runs, proves and prints out the results of IronLight, that is RandomX light mode written in Rust. 
[Here](https://github.com/fluencelabs/randomx-rust-wrapper/tree/randomx-plonky3-sp1) is the IronLight library repo. 


## Usage example
```shell
RAYON_NUM_THREADS=2 FRI_QUERIES=1 RUSTFLAGS='-C target-cpu=native' cargo run --release
cp -rp ~/.sp1/circuits/dev/* ~/.sp1/circuits/groth16/v3.0.0/
RAYON_NUM_THREADS=2 FRI_QUERIES=1 RUST_LOG=info RUSTFLAGS='-C target-cpu=native' cargo run --release
```

## Verification

To verify the results on-chain run this standalone scenario. It needs `$PRIVATE_KEY` and `$RPC_URL` env variables to be set.
```shell
time SP1_DEV=1 RAYON_NUM_THREADS=2 FRI_QUERIES=1 RUSTFLAGS='-C target-cpu=native' cargo run --release | grep 'proof: ' | awk '{gsub(/"/,""); print $3}' > /tmp/proof_as_bytes.bin && export PROOF_AS_BYTES=$(cat /tmp/proof_as_bytes.bin)

cp ~/.sp1/circuits/dev/*.sol ./contracts/src/groth16/

cd ./contracts
forge build
export VERIFIER_ADDRESS=$(forge create --private-key $(echo $PRIVATE_KEY) src/groth16/SP1VerifierGroth16.sol:SP1Verifier | grep -i "Deployed to:" | awk '{print $3}')
forge script CallVerifyProof --broadcast -v 5  --private-key $(echo $PRIVATE_KEY)
```
