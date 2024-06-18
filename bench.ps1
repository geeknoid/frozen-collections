# Run all benchmarks and convert into the markdown
cargo criterion --message-format=json | criterion-table > BENCHMARKS.md
