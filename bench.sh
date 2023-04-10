#!/usr/bin/env bash

# Benchmark 1: ators -i -o dwarf -l 0x0100360000 -f ./fixtures/many_addrs.txt
#   Time (mean ± σ):      63.8 ms ±   1.1 ms    [User: 59.1 ms, System: 3.3 ms]
#   Range (min … max):    61.6 ms …  65.9 ms    41 runs
#
# Benchmark 2: atos -i -o dwarf -l 0x0100360000 -f ./fixtures/many_addrs.txt
#   Time (mean ± σ):     227.0 ms ±   0.9 ms    [User: 211.7 ms, System: 11.3 ms]
#   Range (min … max):   225.7 ms … 228.5 ms    12 runs
#
# Benchmark 3: atosl -o dwarf -l 0 ...
#   Time (mean ± σ):      1.249 s ±  0.006 s    [User: 1.230 s, System: 0.016 s]
#   Range (min … max):    1.243 s …  1.263 s    10 runs
#
# Summary
#   'ators -i -o dwarf -l 0x0100360000 -f ./fixtures/many_addrs.txt' ran
#     3.56 ± 0.06 times faster than 'atos -i -o dwarf -l 0x0100360000 -f ./fixtures/many_addrs.txt'
#    19.58 ± 0.34 times faster than 'atosl -o dwarf -l 0 ...

cargo build --release

hyperfine --warmup 3 \
    "./target/release/ators -i -o ./fixtures/rollbar -l 0x0100360000 -f ./fixtures/many_addrs.txt" \
    "atos -i -o ./fixtures/rollbar -l 0x0100360000 -f ./fixtures/many_addrs.txt" \
    "atosl -o ./fixtures/rollbar -l 0x0100360000 $(cat ./fixtures/many_addrs.txt)"
