#!/bin/bash

echo -n "Please enter the day number (example: 11): "
read day_number
if [ -z "$day_number" ]; then
    echo "Day number is required."
    exit 1
fi

echo -n "Please enter the title (example: sonar_sweep): "
read title
if [ -z "$title" ]; then
    echo "Title is required."
    exit 1
fi

tee "./src/bin/day_${day_number}_${title}.rs" <<EOF >/dev/null
/// Solution to an Advent of Code problem, day ${day_number}, 2021
/// https://adventofcode.com/2021/day/${day_number}

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let lines = contents.lines();
}
EOF

echo "Created ./src/bin/day_${day_number}_${title}.rs"

mkdir -p test_inputs
mkdir -p inputs

touch "./test_inputs/day_${day_number}_${title}.txt"
echo "Created ./test_inputs/day_${day_number}_${title}.txt"

touch "./inputs/day_${day_number}_${title}.txt"
echo "Created ./inputs/day_${day_number}_${title}.txt"
