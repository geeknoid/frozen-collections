# Run all benchmarks and convert into the markdown
cargo criterion --bench scalar_keys --message-format=json >scalar_keys.json
cargo criterion --bench string_keys --message-format=json >string_keys.json
cargo criterion --bench hashed_keys --message-format=json >hashed_keys.json
cargo criterion --bench ordered_keys --message-format=json >ordered_keys.json

Get-Content .\scalar_keys.json,.\string_keys.json,.\hashed_keys.json,.\ordered_keys.json | criterion-table > BENCHMARKS.md

rm scalar_keys.json
rm string_keys.json
rm hashed_keys.json
rm ordered_keys.json
