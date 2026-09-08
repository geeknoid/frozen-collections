#!/usr/bin/env bash
set -euo pipefail

# Run all criterion benchmarks and format results as benchmarks/TIME.md

# Run the benchmarks
cargo bench -p frozen-collections --all-features --bench scalar_keys
cargo bench -p frozen-collections --all-features --bench string_keys
cargo bench -p frozen-collections --all-features --bench hashed_keys
cargo bench -p frozen-collections --all-features --bench ordered_keys

BASE_PATH="target/criterion"

get_time_ns() {
    local estimates="$1"
    if [[ ! -f "$estimates" ]]; then
        echo ""
        return
    fi
    # Extract slope.point_estimate (nanoseconds)
    python3 -c "
import json, sys
with open('$estimates') as f:
    d = json.load(f)
# Prefer slope, fall back to mean
est = d.get('slope', d.get('mean', {}))
print(est.get('point_estimate', ''))
"
}

format_time() {
    local ns="$1"
    # Convert nanoseconds to human-readable with 2 decimal places
    python3 -c "
ns = $ns
if ns >= 1_000_000:
    print(f'{ns / 1_000_000:.2f} ms')
elif ns >= 1_000:
    print(f'{ns / 1_000:.2f} us')
else:
    print(f'{ns:.2f} ns')
"
}

format_cell() {
    local value="$1"
    local baseline="$2"
    local formatted
    formatted=$(format_time "$value")

    local result
    result=$(python3 -c "
baseline = $baseline
value = $value
if abs(value - baseline) / baseline < 0.005:
    print(f'\`{\"$formatted\"}\` (baseline)')
elif baseline >= value:
    pct = (1 - value / baseline) * 100
    emoji = '🚀' if baseline / value >= 2.0 else '✅'
    print(f'\`{\"$formatted\"}\` ({emoji} **{pct:.0f}% faster**)')
else:
    pct = (value / baseline - 1) * 100
    print(f'\`{\"$formatted\"}\` (❌ **{pct:.0f}% slower**)')
")
    echo "$result"
}

# get_bench_time GROUP DIR_NAME SIZE
get_bench_time() {
    local group="$1" dir_name="$2" size="$3"
    local estimates="${BASE_PATH}/${group}/${dir_name}/${size}/new/estimates.json"
    get_time_ns "$estimates"
}

# build_table GROUP MACRO_PREFIX MACRO_DISPLAY COL_DIR_NAMES... -- COL_DISPLAYS...
build_table() {
    local group="$1" macro_prefix="$2" macro_display="$3"
    shift 3

    local col_dirs=()
    local col_displays=()
    local in_displays=0

    for arg in "$@"; do
        if [[ "$arg" == "--" ]]; then
            in_displays=1
            continue
        fi
        if [[ "$in_displays" -eq 0 ]]; then
            col_dirs+=("$arg")
        else
            col_displays+=("$arg")
        fi
    done

    local sizes=(3 16 256 1000)

    # Build header
    local header="|            |"
    local separator="|:-----------|"
    for display in "${col_displays[@]}"; do
        header+=" \`${display}\` |"
        separator+=":----------------------------------|"
    done
    if [[ -n "$macro_prefix" ]]; then
        header+=" \`${macro_display}\` |"
        separator+=":----------------------------------|"
    fi

    echo "$header"
    echo "$separator"

    for size in "${sizes[@]}"; do
        local row="| **\`${size}\`** |"
        local values=()

        for dir_name in "${col_dirs[@]}"; do
            local val
            val=$(get_bench_time "$group" "$dir_name" "$size")
            values+=("$val")
        done

        if [[ -n "$macro_prefix" ]]; then
            local macro_val
            macro_val=$(get_bench_time "$group" "$macro_prefix" "$size")
            values+=("$macro_val")
        fi

        local baseline="${values[0]}"
        for val in "${values[@]}"; do
            if [[ -z "$val" ]]; then
                row+=" N/A |"
            else
                row+=" $(format_cell "$val" "$baseline") |"
            fi
        done

        echo "$row"
    done
}

# --- Build all tables ---

{
cat << 'HEADER'
# Benchmarks (Wall-Clock Time)

## Table of Contents

- [Overview](#overview)
- [Benchmark Results](#benchmark-results)
    - [dense_scalar](#dense_scalar)
    - [sparse_scalar](#sparse_scalar)
    - [random_scalar](#random_scalar)
    - [random_string](#random_string)
    - [prefixed_string](#prefixed_string)
    - [hashed](#hashed)
    - [ordered](#ordered)

## Overview

These benchmarks compare the wall-clock performance of the frozen collections relative
to the classic Rust collections, measured using criterion.

The frozen collections have different optimizations depending on the type of data they
store and how it is declared. The benchmarks probe those different features to show
the effect of the different optimizations on effective performance.

When you see `HashSet(SipHasher)` vs. `HashSet(foldhash)` this reflects the performance difference between the
normal hasher used by the standard collections as opposed to the performance that the
`foldhash` hasher provides using those same collections.

The benchmarks assume a 50% hit rate when probing for lookup, meaning that
half the queries are for non-existing data. Some algorithms perform differently between
present vs. non-existing cases, so real world performance of these algorithms depends on the
real world hit rate you experience.

## Benchmark Results
HEADER

echo ""
echo "### dense_scalar"
echo ""
echo "Scalar sets where the values are in a contiguous range."
echo ""
build_table "dense_scalar" "fz_scalar_set" "fz_scalar_set" \
    "hashset(foldhash)" "hashset(siphasher)" fzscalarset \
    -- \
    "HashSet(foldhash)" "HashSet(SipHasher)" "FzScalarSet"

echo ""
echo "### sparse_scalar"
echo ""
echo "Scalar sets where the values are in a non-contiguous range."
echo ""
build_table "sparse_scalar" "fz_scalar_set" "fz_scalar_set" \
    "hashset(foldhash)" "hashset(siphasher)" fzscalarset \
    -- \
    "HashSet(foldhash)" "HashSet(SipHasher)" "FzScalarSet"

echo ""
echo "### random_scalar"
echo ""
echo "Scalar sets where the values are randomly distributed."
echo ""
build_table "random_scalar" "fz_scalar_set" "fz_scalar_set" \
    "hashset(foldhash)" "hashset(siphasher)" fzscalarset \
    -- \
    "HashSet(foldhash)" "HashSet(SipHasher)" "FzScalarSet"

echo ""
echo "### random_string"
echo ""
echo "String sets where the values are random."
echo ""
build_table "random_string" "fz_string_set" "fz_string_set" \
    "hashset(foldhash)" "hashset(siphasher)" "fzstringset(box)" "fzstringset(str)" "fzstringset(compact)" \
    -- \
    "HashSet(foldhash)" "HashSet(SipHasher)" "FzStringSet(box)" "FzStringSet(str)" "FzStringSet(compact)"

echo ""
echo "### prefixed_string"
echo ""
echo "String sets where the values are random but share a common prefix."
echo ""
build_table "prefixed_string" "fz_string_set" "fz_string_set" \
    "hashset(foldhash)" "hashset(siphasher)" "fzstringset(box)" "fzstringset(str)" "fzstringset(compact)" \
    -- \
    "HashSet(foldhash)" "HashSet(SipHasher)" "FzStringSet(box)" "FzStringSet(str)" "FzStringSet(compact)"

echo ""
echo "### hashed"
echo ""
echo "Sets with a complex value type that is hashable."
echo ""
build_table "hashed" "fz_hash_set" "fz_hash_set" \
    "hashset(foldhash)" "hashset(siphasher)" fzhashset \
    -- \
    "HashSet(foldhash)" "HashSet(SipHasher)" "FzHashSet"

echo ""
echo "### ordered"
echo ""
echo "Sets with a complex value type that is ordered."
echo ""
build_table "ordered" "fz_ordered_set" "fz_ordered_set" \
    btreeset fzorderedset \
    -- \
    "BTreeSet" "FzOrderedSet"

echo ""
echo "---"
echo "Generated using criterion benchmarks"
echo ""
} > benchmarks/TIME.md

echo "Results written to benchmarks/TIME.md"
