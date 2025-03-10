/*
 * Copyright 2025 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use ccp_ironlight::{ironlight::HashWithGroth16Proof, Cache, IronLightVM, RandomXFlags};

fn run_prove_light(global_nonce: &[u8], local_nonce: &[u8], flags: RandomXFlags) -> HashWithGroth16Proof {
    let cache = Cache::new(&global_nonce, flags).unwrap();
    let mut vm = IronLightVM::new(cache, flags).unwrap();
    vm.prove_light(&local_nonce)
}

fn main() {
    let global_nonce = vec![3, 1, 2, 3, 4, 5, 6, 7];
    let local_nonce = vec![4, 2, 3, 4, 5, 6, 7];
    let flags = RandomXFlags::DEFAULT | RandomXFlags::FLAG_IRONLIGHT;

    let  HashWithGroth16Proof {hash, proof} = run_prove_light(&global_nonce, &local_nonce, flags);
    println!("hash: {:?}", hash);
    println!("groth16 proof: {:?}", hex::encode(proof));
}
