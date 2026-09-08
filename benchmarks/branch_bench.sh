#!/usr/bin/env bash
set -euo pipefail

# Run all gungraun (iai-callgrind) benchmarks and format branch misprediction results
# as benchmarks/BRANCHES.md. Requires --branch-sim=yes in the callgrind config.

# Run the benchmarks
cargo bench -p frozen-collections --all-features --bench gungraun_scalar_keys
cargo bench -p frozen-collections --all-features --bench gungraun_string_keys
cargo bench -p frozen-collections --all-features --bench gungraun_hashed_keys
cargo bench -p frozen-collections --all-features --bench gungraun_ordered_keys

BASE_PATH="target/gungraun/frozen-collections"

# Extract a named event value from a callgrind output file.
# Usage: get_event FILE EVENT_NAME
# Events line: "events: Ir Dr Dw ... Bc Bcm Bi Bim"
# Summary line: "summary: 123 456 789 ... 10 2 5 1"
get_event() {
    local out_file="$1"
    local event_name="$2"
    if [[ ! -f "$out_file" ]]; then
        echo ""
        return
    fi

    python3 -c "
import sys
events = None
summary = None
with open('$out_file') as f:
    for line in f:
        if line.startswith('events:'):
            events = line.strip().split()[1:]
        elif line.startswith('summary:'):
            summary = line.strip().split()[1:]
            break
if events is None or summary is None:
    sys.exit(0)
name = '$event_name'
if name in events:
    idx = events.index(name)
    if idx < len(summary):
        print(summary[idx])
"
}

format_number() {
    printf "%'d" "$1"
}

format_cell() {
    local value=$1
    local baseline=$2
    local formatted
    formatted=$(format_number "$value")

    if [[ "$value" -eq "$baseline" ]]; then
        echo "\`${formatted}\` (baseline)"
        return
    fi

    local compare
    compare=$(awk "BEGIN {print ($baseline >= $value) ? 1 : 0}")

    if [[ "$compare" -eq 1 ]]; then
        local pct
        pct=$(awk "BEGIN {printf \"%.0f\", (1 - $value / $baseline) * 100}")
        local emoji
        emoji=$(awk "BEGIN {print ($baseline / $value >= 2.0) ? \"🚀\" : \"✅\"}")
        echo "\`${formatted}\` (${emoji} **${pct}% fewer**)"
    else
        local pct
        pct=$(awk "BEGIN {printf \"%.0f\", ($value / $baseline - 1) * 100}")
        echo "\`${formatted}\` (❌ **${pct}% more**)"
    fi
}

get_bench_result() {
    local bench_name="$1" group_name="$2" fn_name="$3" variant="$4" event_name="$5"
    local out_file="${BASE_PATH}/${bench_name}/${group_name}/${fn_name}.${variant}/callgrind.${fn_name}.${variant}.out"
    get_event "$out_file" "$event_name"
}

# build_table BENCH_NAME GROUP_NAME SUITE_PREFIX MACRO_PREFIX MACRO_DISPLAY_NAME COL_NAMES... -- COL_DISPLAYS...
# The first collection column is used as the baseline.
build_table() {
    local bench_name="$1" group_name="$2" suite_prefix="$3" macro_prefix="$4" macro_display="$5"
    shift 5

    local col_names=()
    local col_displays=()
    local in_displays=0

    for arg in "$@"; do
        if [[ "$arg" == "--" ]]; then
            in_displays=1
            continue
        fi
        if [[ "$in_displays" -eq 0 ]]; then
            col_names+=("$arg")
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
        local display_name="${macro_display:-$macro_prefix}"
        header+=" \`${display_name}\` |"
        separator+=":----------------------------------|"
    fi

    echo "$header"
    echo "$separator"

    for size in "${sizes[@]}"; do
        local row="| **\`${size}\`** |"
        local values=()

        for col_name in "${col_names[@]}"; do
            local fn_name="${suite_prefix}_${col_name}"
            local val
            val=$(get_bench_result "$bench_name" "$group_name" "$fn_name" "size_${size}" "Bcm")
            values+=("$val")
        done

        if [[ -n "$macro_prefix" ]]; then
            local macro_fn="${suite_prefix}_${macro_prefix}_${size}"
            local macro_val
            macro_val=$(get_bench_result "$bench_name" "$group_name" "$macro_fn" "run" "Bcm")
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
# Benchmarks (Branch Mispredictions)

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

These benchmarks compare the branch misprediction counts of the frozen collections relative
to the classic Rust collections, measured using callgrind's branch simulation via gungraun.

Branch mispredictions are deterministic under callgrind's simulation and independent of
system load, making them reproducible across runs.

The frozen collections have different optimizations depending on the type of data they
store and how it is declared. The benchmarks probe those different features to show
the effect of the different optimizations on branch prediction behavior.

When you see `HashSet(SipHasher)` vs. `HashSet(foldhash)` this reflects the difference between the
normal hasher used by the standard collections as opposed to the
`foldhash` hasher used with those same collections. 

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
build_table "gungraun_scalar_keys" "dense_scalar_group" "dense_scalar" \
    "fz_scalar_set_macro" "fz_scalar_set" \
    hashset_foldhash hashset_classic fz_scalar_set \
    -- \
    "HashSet(foldhash)" "HashSet(siphasher)" "FzScalarSet"

echo ""
echo "### sparse_scalar"
echo ""
echo "Scalar sets where the values are in a non-contiguous range."
echo ""
build_table "gungraun_scalar_keys" "sparse_scalar_group" "sparse_scalar" \
    "fz_scalar_set_macro" "fz_scalar_set" \
    hashset_foldhash hashset_classic fz_scalar_set \
    -- \
    "HashSet(foldhash)" "HashSet(siphasher)" "FzScalarSet"

echo ""
echo "### random_scalar"
echo ""
echo "Scalar sets where the values are randomly distributed."
echo ""
build_table "gungraun_scalar_keys" "random_scalar_group" "random_scalar" \
    "fz_scalar_set_macro" "fz_scalar_set" \
    hashset_foldhash hashset_classic fz_scalar_set \
    -- \
    "HashSet(foldhash)" "HashSet(siphasher)" "FzScalarSet"

echo ""
echo "### random_string"
echo ""
echo "String sets where the values are random."
echo ""
build_table "gungraun_string_keys" "random_string_group" "random_string" \
    "fz_string_set_macro" "fz_string_set" \
    hashset_foldhash hashset_classic fz_string_set fz_string_set_str fz_string_set_compact \
    -- \
    "HashSet(foldhash)" "HashSet(siphasher)" "FzStringSet(box)" "FzStringSet(str)" "FzStringSet(compact)"

echo ""
echo "### prefixed_string"
echo ""
echo "String sets where the values are random but share a common prefix."
echo ""
build_table "gungraun_string_keys" "prefixed_string_group" "prefixed_string" \
    "fz_string_set_macro" "fz_string_set" \
    hashset_foldhash hashset_classic fz_string_set fz_string_set_str fz_string_set_compact \
    -- \
    "HashSet(foldhash)" "HashSet(siphasher)" "FzStringSet(box)" "FzStringSet(str)" "FzStringSet(compact)"

echo ""
echo "### hashed"
echo ""
echo "Sets with a complex value type that is hashable."
echo ""
build_table "gungraun_hashed_keys" "hashed_group" "hashed" \
    "fz_hash_set_macro" "fz_hash_set" \
    hashset_foldhash hashset_classic fz_hash_set \
    -- \
    "HashSet(foldhash)" "HashSet(siphasher)" "FzHashSet"

echo ""
echo "### ordered"
echo ""
echo "Sets with a complex value type that is ordered."
echo ""
build_table "gungraun_ordered_keys" "ordered_group" "ordered" \
    "fz_ordered_set_macro" "fz_ordered_set" \
    btreeset fz_ordered_set \
    -- \
    "BTreeSet" "FzOrderedSet"

echo ""
echo "---"
echo "Generated using gungraun (iai-callgrind) benchmarks with branch simulation"
echo ""
} > benchmarks/BRANCHES.md

echo "Results written to benchmarks/BRANCHES.md"
