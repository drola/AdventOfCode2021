#!/bin/bash
day_number="$1"

if [ -z "$day_number" ]; then
    echo "Day number is required."
    echo "Example: ./test_day.sh 11"
    exit 1
fi

filename=`find ./src/bin -name "day_${day_number}_*" -printf "%f\n" | head -1 | sed 's/.rs//'`

if [ -z "$filename" ]; then
    echo "Cannot find executable for day $day_number"
    exit 1
fi

test_input_file="test_inputs/${filename}.txt"

if [[ -f "$test_input_file" ]]; then
    command="cargo run --bin ${filename} -- ${test_input_file}"
else
    command="cargo test --bin ${filename}"
fi

echo $command
$command
