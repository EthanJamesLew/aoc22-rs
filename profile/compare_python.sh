#!/bin/sh
set -e

# run the rust part
cargo build --release --example day$1
echo "\033[0;31mRunning Rust\033[0m"
perf stat ./target/release/examples/day$1 2> rust.out

# run the python part
echo "\033[0;31mRunning Python\033[0m"
perf stat python ./python/day$1.py 2> python.out
PY_TIME=$(grep -Po "([0-9\.]+) seconds time elapsed" python.out | grep -Po "([0-9\.]+)")
RS_TIME=$(grep -Po "([0-9\.]+) seconds time elapsed" rust.out | grep -Po "([0-9\.]+)")
P_MULT=$(echo $PY_TIME / $RS_TIME | bc)

# summarize performance
echo "-----Performance Summary-----"
echo "Python Time: $PY_TIME seconds"
echo "Rust Time: $RS_TIME seconds"
echo "Rust is $P_MULT times faster"
