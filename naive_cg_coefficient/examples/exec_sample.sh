#!/usr/bin/bash

# exec jobs
# ex_1: examples/cg_coefficient_example.rs
# ex_binomial: examples/cg_coefficient_binomial.rs

jobs=('ex_1' 'ex_binomial')
echo ${jobs[@]}

for job in ${jobs[@]};do
    echo "job = $job"
    cargo run --example "$job"
    echo ""
done
