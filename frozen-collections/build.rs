//! Generates benchmark code.

#![expect(clippy::unwrap_used, reason = "panicking on failure is standard in build scripts")]

use rand::RngExt;
use rand_chacha::ChaChaRng;
use rand_chacha::rand_core::SeedableRng;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

const SMALL: usize = 3;
const MEDIUM: usize = 16;
const LARGE: usize = 256;
const HUGE: usize = 1000;

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect()
}

fn emit_benchmark_preamble(name: &str) -> BufWriter<File> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(format!("{name}.rs"));
    let mut file = BufWriter::new(File::create(dest_path).unwrap());

    writeln!(file, "#[allow(clippy::allow_attributes, reason = \"Generated code\")]").unwrap();
    writeln!(file, "#[allow(clippy::unreadable_literal, reason = \"Generated code\")]").unwrap();
    writeln!(file, "#[allow(clippy::items_after_statements, reason = \"Generated code\")]").unwrap();
    writeln!(file, "#[allow(clippy::explicit_auto_deref, reason = \"Generated code\")]").unwrap();
    writeln!(
        file,
        "#[allow(clippy::redundant_closure_for_method_calls, reason = \"Generated code\")]"
    )
    .unwrap();
    writeln!(file, "#[allow(unused_results, reason = \"Generated code\")]").unwrap();
    writeln!(file, "fn {name}(c: &mut Criterion) {{").unwrap();
    writeln!(file, "    let mut group = c.benchmark_group(\"{name}\");").unwrap();

    file
}

fn emit_benchmark_postamble(mut file: BufWriter<File>) {
    writeln!(file, "        group.finish();").unwrap();
    writeln!(file, "}}").unwrap();
}

fn emit_loop(file: &mut BufWriter<File>, name: &str) {
    writeln!(
        file,
        "    group.bench_with_input(BenchmarkId::new(\"{name}\", size), &size, |b, _| {{"
    )
    .unwrap();
    writeln!(file, "        b.iter(|| {{").unwrap();
    writeln!(file, "            for key in &probe {{").unwrap();
    writeln!(file, "                _ = black_box(black_box(&s).contains(black_box(key)));").unwrap();
    writeln!(file, "            }}").unwrap();
    writeln!(file, "        }});").unwrap();
    writeln!(file, "    }});").unwrap();
}

fn emit_scalar_suite(file: &mut BufWriter<File>, size: usize, literal_producer: impl Fn(&mut BufWriter<File>, usize)) {
    writeln!(file, "    let frozen = fz_scalar_set!({{").unwrap();
    literal_producer(file, size);
    writeln!(file, "    }});").unwrap();

    writeln!(file, "    let input: Vec<i32> = frozen.clone().into_iter().collect();").unwrap();
    writeln!(file, "    let size = input.len();").unwrap();
    writeln!(file, "    let mut probe = Vec::new();").unwrap();
    writeln!(file, "    for i in &input {{").unwrap();
    writeln!(file, "        probe.push(*i);").unwrap();
    writeln!(file, "        probe.push(-(*i));").unwrap();
    writeln!(file, "    }}").unwrap();

    writeln!(
        file,
        "    let s = std::collections::HashSet::<_, foldhash::fast::FixedState>::from_iter(input.clone());"
    )
    .unwrap();
    emit_loop(file, "HashSet(foldhash)");

    writeln!(
        file,
        "    let s = std::collections::HashSet::<_, std::hash::BuildHasherDefault<std::hash::DefaultHasher>>::from_iter(input.clone());"
    )
        .unwrap();
    emit_loop(file, "HashSet(SipHasher)");

    writeln!(file, "    let s = FzScalarSet::new(input);").unwrap();
    emit_loop(file, "FzScalarSet");

    writeln!(file, "    let s = frozen;").unwrap();
    emit_loop(file, "fz_scalar_set");
}

fn emit_string_suite(file: &mut BufWriter<File>, size: usize, literal_producer: impl Fn(&mut BufWriter<File>, usize)) {
    writeln!(file, "    let frozen = fz_string_set!({{").unwrap();
    literal_producer(file, size);
    writeln!(file, "    }});").unwrap();

    writeln!(
        file,
        "    let input: Vec<String> = frozen.clone().into_iter().map(|x| x.to_string()).collect();"
    )
    .unwrap();
    writeln!(file, "    let size = input.len();").unwrap();
    writeln!(file, "    let mut probe = Vec::new();").unwrap();
    writeln!(file, "    for s in &input {{").unwrap();
    writeln!(file, "        probe.push((*s).clone());").unwrap();
    writeln!(file, "        probe.push((*s).clone().add(\"Hello\"));").unwrap();
    writeln!(file, "    }}").unwrap();

    writeln!(file, "    let mut tmp: Vec<&str> = Vec::new();").unwrap();
    writeln!(file, "    for x in &input {{").unwrap();
    writeln!(file, "        tmp.push(x.as_str());").unwrap();
    writeln!(file, "    }}").unwrap();
    writeln!(file, "    let input = tmp;").unwrap();

    writeln!(file, "    let mut tmp: Vec<&str> = Vec::new();").unwrap();
    writeln!(file, "    for x in &probe {{").unwrap();
    writeln!(file, "        tmp.push(x.as_str());").unwrap();
    writeln!(file, "    }}").unwrap();
    writeln!(file, "    let probe = tmp;").unwrap();

    writeln!(
        file,
        "    let s = std::collections::HashSet::<_, foldhash::fast::FixedState>::from_iter(input.clone());"
    )
    .unwrap();
    emit_loop(file, "HashSet(foldhash)");

    writeln!(
        file,
        "    let s = std::collections::HashSet::<_, std::hash::BuildHasherDefault<std::hash::DefaultHasher>>::from_iter(input.clone());"
    )
        .unwrap();
    emit_loop(file, "HashSet(SipHasher)");

    writeln!(file, "    let s = FzStringSet::new(input.clone());").unwrap();
    emit_loop(file, "FzStringSet(box)");

    writeln!(file, "    let s = FzStringSet::new_for_str(input.clone());").unwrap();
    emit_loop(file, "FzStringSet(str)");

    writeln!(file, "    let s = FzStringSet::new_compact(input);").unwrap();
    emit_loop(file, "FzStringSet(compact)");

    writeln!(file, "    let s = frozen;").unwrap();
    emit_loop(file, "fz_string_set");
}

fn emit_hashed_suite(file: &mut BufWriter<File>, size: usize, literal_producer: impl Fn(&mut BufWriter<File>, usize)) {
    writeln!(file, "    let frozen = fz_hash_set!({{").unwrap();
    literal_producer(file, size);
    writeln!(file, "    }});").unwrap();

    writeln!(file, "    let input: Vec<_> = frozen.clone().into_iter().collect();").unwrap();
    writeln!(file, "    let size = input.len();").unwrap();
    writeln!(file, "    let mut probe = Vec::new();").unwrap();
    writeln!(file, "    for i in &input {{").unwrap();
    writeln!(file, "        probe.push(Record {{ name: (*i).name.clone(), age: (*i).age }});").unwrap();
    writeln!(file, "        probe.push(Record {{ name: (*i).name.clone(), age: -(*i).age }});").unwrap();
    writeln!(file, "    }}").unwrap();

    writeln!(
        file,
        "    let s = std::collections::HashSet::<_, foldhash::fast::FixedState>::from_iter(input.clone());"
    )
    .unwrap();
    emit_loop(file, "HashSet(foldhash)");

    writeln!(
        file,
        "    let s = std::collections::HashSet::<_, std::hash::BuildHasherDefault<std::hash::DefaultHasher>>::from_iter(input.clone());"
    )
    .unwrap();
    emit_loop(file, "HashSet(SipHasher)");

    writeln!(file, "    let s = FzHashSet::new(input);").unwrap();
    emit_loop(file, "FzHashSet");

    writeln!(file, "    let s = frozen;").unwrap();
    emit_loop(file, "fz_hash_set");
}

fn emit_ordered_suite(file: &mut BufWriter<File>, size: usize, literal_producer: impl Fn(&mut BufWriter<File>, usize)) {
    writeln!(file, "    let frozen = fz_ordered_set!({{").unwrap();
    literal_producer(file, size);
    writeln!(file, "    }});").unwrap();

    writeln!(file, "    let input: Vec<_> = frozen.clone().into_iter().collect();").unwrap();
    writeln!(file, "    let size = input.len();").unwrap();
    writeln!(file, "    let mut probe = Vec::new();").unwrap();
    writeln!(file, "    for i in &input {{").unwrap();
    writeln!(file, "        probe.push(Record {{ name: (*i).name.clone(), age: (*i).age }});").unwrap();
    writeln!(file, "        probe.push(Record {{ name: (*i).name.clone(), age: -(*i).age }});").unwrap();
    writeln!(file, "    }}").unwrap();

    writeln!(file, "    let s = std::collections::BTreeSet::<_>::from_iter(input.clone());").unwrap();
    emit_loop(file, "BTreeSet");

    writeln!(file, "    let s = FzOrderedSet::new(input);").unwrap();
    emit_loop(file, "FzOrderedSet");

    writeln!(file, "    let s = frozen;").unwrap();
    emit_loop(file, "fz_ordered_set");
}

fn emit_dense_scalar_benchmark() {
    fn dense_producer(file: &mut BufWriter<File>, size: usize) {
        for i in 0..size {
            writeln!(file, "        {i},",).unwrap();
        }
    }

    let mut file = emit_benchmark_preamble("dense_scalar");

    emit_scalar_suite(&mut file, SMALL, dense_producer);
    emit_scalar_suite(&mut file, MEDIUM, dense_producer);
    emit_scalar_suite(&mut file, LARGE, dense_producer);
    emit_scalar_suite(&mut file, HUGE, dense_producer);

    emit_benchmark_postamble(file);
}

fn emit_sparse_scalar_benchmark() {
    fn sparse_producer(file: &mut BufWriter<File>, size: usize) {
        for i in 0..size {
            let x = i * 2;
            writeln!(file, "        {x},",).unwrap();
        }
    }

    let mut file = emit_benchmark_preamble("sparse_scalar");

    emit_scalar_suite(&mut file, SMALL, sparse_producer);
    emit_scalar_suite(&mut file, MEDIUM, sparse_producer);
    emit_scalar_suite(&mut file, LARGE, sparse_producer);
    emit_scalar_suite(&mut file, HUGE, sparse_producer);

    emit_benchmark_postamble(file);
}

fn emit_random_scalar_benchmark() {
    fn random_producer(file: &mut BufWriter<File>, size: usize) {
        let mut rng = ChaChaRng::seed_from_u64(0x1234_5678);

        for _ in 0..size {
            let x: i32 = rng.random();
            writeln!(file, "        {x},",).unwrap();
        }
    }

    let mut file = emit_benchmark_preamble("random_scalar");

    emit_scalar_suite(&mut file, SMALL, random_producer);
    emit_scalar_suite(&mut file, MEDIUM, random_producer);
    emit_scalar_suite(&mut file, LARGE, random_producer);
    emit_scalar_suite(&mut file, HUGE, random_producer);

    emit_benchmark_postamble(file);
}

fn emit_prefixed_string_benchmark() {
    fn prefixed_producer(file: &mut BufWriter<File>, size: usize) {
        let mut rng = ChaChaRng::seed_from_u64(0x1234_5678);

        for _ in 0..size {
            let len: u32 = rng.random();
            let len = (len % 10) + 5;
            let mut s = String::new();
            for _ in 0..len {
                let x: u8 = rng.random();
                let x = (x % 26) + 97;
                s.push(x as char);
            }

            writeln!(file, "        \"Color-{s}\",",).unwrap();
        }
    }

    let mut file = emit_benchmark_preamble("prefixed_string");

    emit_string_suite(&mut file, SMALL, prefixed_producer);
    emit_string_suite(&mut file, MEDIUM, prefixed_producer);
    emit_string_suite(&mut file, LARGE, prefixed_producer);
    emit_string_suite(&mut file, HUGE, prefixed_producer);

    emit_benchmark_postamble(file);
}

fn emit_random_string_benchmark() {
    fn random_producer(file: &mut BufWriter<File>, size: usize) {
        let mut rng = ChaChaRng::seed_from_u64(0x1234_5678);

        for _ in 0..size {
            let len: u32 = rng.random();
            let len = (len % 10) + 5;
            let mut s = String::new();
            for _ in 0..len {
                let x: u8 = rng.random();
                let x = (x % 26) + 97;
                s.push(x as char);
            }

            writeln!(file, "        \"{s}\",",).unwrap();
        }
    }

    let mut file = emit_benchmark_preamble("random_string");

    emit_string_suite(&mut file, SMALL, random_producer);
    emit_string_suite(&mut file, MEDIUM, random_producer);
    emit_string_suite(&mut file, LARGE, random_producer);
    emit_string_suite(&mut file, HUGE, random_producer);

    emit_benchmark_postamble(file);
}

fn emit_hashed_benchmark() {
    fn hashed_producer(file: &mut BufWriter<File>, size: usize) {
        let mut rng = ChaChaRng::seed_from_u64(0x1234_5678);

        for _ in 0..size {
            let len: u32 = rng.random();
            let len = (len % 10) + 5;
            let mut s = String::new();
            for _ in 0..len {
                let x: u8 = rng.random();
                let x = (x % 26) + 97;
                s.push(x as char);
            }

            let age: i32 = rng.random();
            writeln!(file, "        Record {{ name: \"{s}\".to_string(), age: {age} }},").unwrap();
        }
    }

    let mut file = emit_benchmark_preamble("hashed");

    writeln!(file, "#[derive(Clone, Debug, Eq, Hash, PartialEq)]").unwrap();
    writeln!(file, "struct Record {{").unwrap();
    writeln!(file, "    name: String,").unwrap();
    writeln!(file, "    age: i32,").unwrap();
    writeln!(file, "}}").unwrap();

    emit_hashed_suite(&mut file, SMALL, hashed_producer);
    emit_hashed_suite(&mut file, MEDIUM, hashed_producer);
    emit_hashed_suite(&mut file, LARGE, hashed_producer);
    emit_hashed_suite(&mut file, HUGE, hashed_producer);

    emit_benchmark_postamble(file);
}

fn emit_ordered_benchmark() {
    fn ordered_producer(file: &mut BufWriter<File>, size: usize) {
        let mut rng = ChaChaRng::seed_from_u64(0x1234_5678);

        for _ in 0..size {
            let len: u32 = rng.random();
            let len = (len % 10) + 5;
            let mut s = String::new();
            for _ in 0..len {
                let x: u8 = rng.random();
                let x = (x % 26) + 97;
                s.push(x as char);
            }

            let age: i32 = rng.random();
            writeln!(file, "        Record {{ name: \"{s}\".to_string(), age: {age} }},").unwrap();
        }
    }

    let mut file = emit_benchmark_preamble("ordered");

    writeln!(file, "#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]").unwrap();
    writeln!(file, "struct Record {{").unwrap();
    writeln!(file, "    name: String,").unwrap();
    writeln!(file, "    age: i32,").unwrap();
    writeln!(file, "}}").unwrap();

    emit_ordered_suite(&mut file, SMALL, ordered_producer);
    emit_ordered_suite(&mut file, MEDIUM, ordered_producer);
    emit_ordered_suite(&mut file, LARGE, ordered_producer);
    emit_ordered_suite(&mut file, HUGE, ordered_producer);

    emit_benchmark_postamble(file);
}

// === GUNGRAUN BENCHMARK GENERATION ===

fn emit_iai_preamble(name: &str) -> BufWriter<File> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(format!("{name}.rs"));
    BufWriter::new(File::create(dest_path).unwrap())
}

fn emit_iai_loop(file: &mut BufWriter<File>, set_expr: &str, probe_expr: &str) {
    writeln!(file, "    for key in {probe_expr} {{").unwrap();
    writeln!(
        file,
        "        _ = black_box(black_box({set_expr}).contains(black_box(key)));"
    )
    .unwrap();
    writeln!(file, "    }}").unwrap();
}

// --- Scalar IAI benchmarks ---

fn emit_iai_scalar_suite(
    file: &mut BufWriter<File>,
    suite: &str,
    size: usize,
    literal_producer: &dyn Fn(&mut BufWriter<File>, usize),
) {
    let suite_upper = suite.to_uppercase();
    let suite_pascal = to_pascal_case(suite);
    let static_name = format!("FROZEN_{suite_upper}_{size}");
    let alias_name = format!("Frozen{suite_pascal}{size}Type");

    writeln!(file, "fz_scalar_set!(static {static_name}: {alias_name}<i32>, {{").unwrap();
    literal_producer(file, size);
    writeln!(file, "}});").unwrap();
    writeln!(file).unwrap();

    writeln!(
        file,
        "fn data_{suite}_{size}() -> (Vec<i32>, Vec<i32>) {{"
    )
    .unwrap();
    writeln!(
        file,
        "    let input: Vec<i32> = {static_name}.iter().copied().collect();"
    )
    .unwrap();
    writeln!(file, "    let mut probe = Vec::new();").unwrap();
    writeln!(file, "    for i in &input {{").unwrap();
    writeln!(file, "        probe.push(*i);").unwrap();
    writeln!(file, "        probe.push(-(*i));").unwrap();
    writeln!(file, "    }}").unwrap();
    writeln!(file, "    (input, probe)").unwrap();
    writeln!(file, "}}").unwrap();
    writeln!(file).unwrap();

    let collections: &[(&str, &str, &str)] = &[
        (
            "hashset_classic",
            "std::collections::HashSet<i32, std::hash::BuildHasherDefault<std::hash::DefaultHasher>>",
            "std::collections::HashSet::<_, std::hash::BuildHasherDefault<std::hash::DefaultHasher>>::from_iter(input)",
        ),
        (
            "hashset_foldhash",
            "std::collections::HashSet<i32, foldhash::fast::FixedState>",
            "std::collections::HashSet::<_, foldhash::fast::FixedState>::from_iter(input)",
        ),
        (
            "fz_scalar_set",
            "FzScalarSet<i32>",
            "FzScalarSet::new(input)",
        ),
    ];

    for (name, typ, creator) in collections {
        writeln!(
            file,
            "fn setup_{suite}_{size}_{name}() -> ({typ}, Vec<i32>) {{"
        )
        .unwrap();
        writeln!(file, "    let (input, probe) = data_{suite}_{size}();").unwrap();
        writeln!(file, "    let s = {creator};").unwrap();
        writeln!(file, "    (s, probe)").unwrap();
        writeln!(file, "}}").unwrap();
        writeln!(file).unwrap();
    }

    writeln!(
        file,
        "fn setup_{suite}_{size}_probe() -> Vec<i32> {{"
    )
    .unwrap();
    writeln!(file, "    data_{suite}_{size}().1").unwrap();
    writeln!(file, "}}").unwrap();
    writeln!(file).unwrap();
}

fn emit_iai_scalar_bench_fns(file: &mut BufWriter<File>, suite: &str) {
    let sizes = [SMALL, MEDIUM, LARGE, HUGE];
    let collections: &[(&str, &str)] = &[
        (
            "hashset_classic",
            "std::collections::HashSet<i32, std::hash::BuildHasherDefault<std::hash::DefaultHasher>>",
        ),
        (
            "hashset_foldhash",
            "std::collections::HashSet<i32, foldhash::fast::FixedState>",
        ),
        ("fz_scalar_set", "FzScalarSet<i32>"),
    ];

    for (name, typ) in collections {
        writeln!(file, "#[library_benchmark]").unwrap();
        for &size in &sizes {
            writeln!(
                file,
                "#[bench::size_{size}(setup_{suite}_{size}_{name}())]"
            )
            .unwrap();
        }
        writeln!(
            file,
            "fn {suite}_{name}(data: ({typ}, Vec<i32>)) {{"
        )
        .unwrap();
        writeln!(file, "    let (ref s, ref probe) = data;").unwrap();
        emit_iai_loop(file, "s", "probe");
        writeln!(file, "}}").unwrap();
        writeln!(file).unwrap();
    }
}

fn emit_iai_scalar_macro_bench_fns(
    file: &mut BufWriter<File>,
    suite: &str,
) {
    let sizes = [SMALL, MEDIUM, LARGE, HUGE];
    let suite_upper = suite.to_uppercase();

    for &size in &sizes {
        let static_name = format!("FROZEN_{suite_upper}_{size}");

        writeln!(file, "#[library_benchmark]").unwrap();
        writeln!(
            file,
            "#[bench::run(setup_{suite}_{size}_probe())]"
        )
        .unwrap();
        writeln!(
            file,
            "fn {suite}_fz_scalar_set_macro_{size}(probe: Vec<i32>) {{"
        )
        .unwrap();
        emit_iai_loop(file, &format!("&{static_name}"), "&probe");
        writeln!(file, "}}").unwrap();
        writeln!(file).unwrap();
    }
}

fn emit_iai_scalar_group(file: &mut BufWriter<File>, suite: &str) {
    let sizes = [SMALL, MEDIUM, LARGE, HUGE];
    writeln!(file, "library_benchmark_group!(").unwrap();
    writeln!(file, "    name = {suite}_group,").unwrap();
    writeln!(file, "    benchmarks = [").unwrap();
    writeln!(file, "        {suite}_hashset_classic,").unwrap();
    writeln!(file, "        {suite}_hashset_foldhash,").unwrap();
    writeln!(file, "        {suite}_fz_scalar_set,").unwrap();
    for &size in &sizes {
        writeln!(file, "        {suite}_fz_scalar_set_macro_{size},").unwrap();
    }
    writeln!(file, "    ]").unwrap();
    writeln!(file, ");").unwrap();
    writeln!(file).unwrap();
}

fn emit_iai_scalar_benchmark(suite: &str, literal_producer: impl Fn(&mut BufWriter<File>, usize)) {
    let sizes = [SMALL, MEDIUM, LARGE, HUGE];
    let mut file = emit_iai_preamble(&format!("gungraun_{suite}"));

    for &size in &sizes {
        emit_iai_scalar_suite(&mut file, suite, size, &literal_producer);
    }
    emit_iai_scalar_bench_fns(&mut file, suite);
    emit_iai_scalar_macro_bench_fns(&mut file, suite);
    emit_iai_scalar_group(&mut file, suite);
}

fn emit_iai_dense_scalar_benchmark() {
    fn dense_producer(file: &mut BufWriter<File>, size: usize) {
        for i in 0..size {
            writeln!(file, "        {i},").unwrap();
        }
    }
    emit_iai_scalar_benchmark("dense_scalar", dense_producer);
}

fn emit_iai_sparse_scalar_benchmark() {
    fn sparse_producer(file: &mut BufWriter<File>, size: usize) {
        for i in 0..size {
            let x = i * 2;
            writeln!(file, "        {x},").unwrap();
        }
    }
    emit_iai_scalar_benchmark("sparse_scalar", sparse_producer);
}

fn emit_iai_random_scalar_benchmark() {
    fn random_producer(file: &mut BufWriter<File>, size: usize) {
        let mut rng = ChaChaRng::seed_from_u64(0x1234_5678);
        for _ in 0..size {
            let x: i32 = rng.random();
            writeln!(file, "        {x},").unwrap();
        }
    }
    emit_iai_scalar_benchmark("random_scalar", random_producer);
}

// --- String IAI benchmarks ---

fn emit_iai_string_suite(
    file: &mut BufWriter<File>,
    suite: &str,
    size: usize,
    literal_producer: &dyn Fn(&mut BufWriter<File>, usize),
) {
    let suite_upper = suite.to_uppercase();
    let suite_pascal = to_pascal_case(suite);
    let static_name = format!("FROZEN_{suite_upper}_{size}");
    let alias_name = format!("Frozen{suite_pascal}{size}Type");

    writeln!(file, "fz_string_set!(static {static_name}: {alias_name}<&'static str>, {{").unwrap();
    literal_producer(file, size);
    writeln!(file, "}});").unwrap();
    writeln!(file).unwrap();

    writeln!(
        file,
        "fn data_{suite}_{size}() -> (Vec<&'static str>, Vec<&'static str>) {{"
    )
    .unwrap();
    writeln!(
        file,
        "    let input: Vec<&'static str> = {static_name}.iter().copied().collect();"
    )
    .unwrap();
    writeln!(file, "    let mut probe_owned: Vec<String> = Vec::new();").unwrap();
    writeln!(file, "    for s in &input {{").unwrap();
    writeln!(file, "        probe_owned.push((*s).to_string());").unwrap();
    writeln!(
        file,
        "        probe_owned.push((*s).to_string() + \"Hello\");"
    )
    .unwrap();
    writeln!(file, "    }}").unwrap();
    writeln!(
        file,
        "    let probe: Vec<&'static str> = probe_owned.into_iter().map(leak_str).collect();"
    )
    .unwrap();
    writeln!(file, "    (input, probe)").unwrap();
    writeln!(file, "}}").unwrap();
    writeln!(file).unwrap();

    let collections: &[(&str, &str, &str)] = &[
        (
            "hashset_classic",
            "std::collections::HashSet<&'static str, std::hash::BuildHasherDefault<std::hash::DefaultHasher>>",
            "std::collections::HashSet::<_, std::hash::BuildHasherDefault<std::hash::DefaultHasher>>::from_iter(input)",
        ),
        (
            "hashset_foldhash",
            "std::collections::HashSet<&'static str, foldhash::fast::FixedState>",
            "std::collections::HashSet::<_, foldhash::fast::FixedState>::from_iter(input)",
        ),
        (
            "fz_string_set",
            "FzStringSet<Box<str>>",
            "FzStringSet::new(input)",
        ),
        (
            "fz_string_set_str",
            "FzStringSet<&'static str>",
            "FzStringSet::new_for_str(input)",
        ),
        (
            "fz_string_set_compact",
            "FzStringSet<compact_str::CompactString>",
            "FzStringSet::new_compact(input)",
        ),
    ];

    for (name, typ, creator) in collections {
        writeln!(
            file,
            "fn setup_{suite}_{size}_{name}() -> ({typ}, Vec<&'static str>) {{"
        )
        .unwrap();
        writeln!(file, "    let (input, probe) = data_{suite}_{size}();").unwrap();
        writeln!(file, "    let s = {creator};").unwrap();
        writeln!(file, "    (s, probe)").unwrap();
        writeln!(file, "}}").unwrap();
        writeln!(file).unwrap();
    }

    writeln!(
        file,
        "fn setup_{suite}_{size}_probe() -> Vec<&'static str> {{"
    )
    .unwrap();
    writeln!(file, "    data_{suite}_{size}().1").unwrap();
    writeln!(file, "}}").unwrap();
    writeln!(file).unwrap();
}

fn emit_iai_string_bench_fns(file: &mut BufWriter<File>, suite: &str) {
    let sizes = [SMALL, MEDIUM, LARGE, HUGE];
    let collections: &[(&str, &str)] = &[
        (
            "hashset_classic",
            "std::collections::HashSet<&'static str, std::hash::BuildHasherDefault<std::hash::DefaultHasher>>",
        ),
        (
            "hashset_foldhash",
            "std::collections::HashSet<&'static str, foldhash::fast::FixedState>",
        ),
        ("fz_string_set", "FzStringSet<Box<str>>"),
        ("fz_string_set_str", "FzStringSet<&'static str>"),
        (
            "fz_string_set_compact",
            "FzStringSet<compact_str::CompactString>",
        ),
    ];

    for (name, typ) in collections {
        writeln!(file, "#[library_benchmark]").unwrap();
        for &size in &sizes {
            writeln!(
                file,
                "#[bench::size_{size}(setup_{suite}_{size}_{name}())]"
            )
            .unwrap();
        }
        writeln!(
            file,
            "fn {suite}_{name}(data: ({typ}, Vec<&'static str>)) {{"
        )
        .unwrap();
        writeln!(file, "    let (ref s, ref probe) = data;").unwrap();
        emit_iai_loop(file, "s", "probe");
        writeln!(file, "}}").unwrap();
        writeln!(file).unwrap();
    }
}

fn emit_iai_string_macro_bench_fns(
    file: &mut BufWriter<File>,
    suite: &str,
) {
    let sizes = [SMALL, MEDIUM, LARGE, HUGE];
    let suite_upper = suite.to_uppercase();

    for &size in &sizes {
        let static_name = format!("FROZEN_{suite_upper}_{size}");

        writeln!(file, "#[library_benchmark]").unwrap();
        writeln!(
            file,
            "#[bench::run(setup_{suite}_{size}_probe())]"
        )
        .unwrap();
        writeln!(
            file,
            "fn {suite}_fz_string_set_macro_{size}(probe: Vec<&'static str>) {{"
        )
        .unwrap();
        emit_iai_loop(file, &format!("&{static_name}"), "&probe");
        writeln!(file, "}}").unwrap();
        writeln!(file).unwrap();
    }
}

fn emit_iai_string_group(file: &mut BufWriter<File>, suite: &str) {
    let sizes = [SMALL, MEDIUM, LARGE, HUGE];
    writeln!(file, "library_benchmark_group!(").unwrap();
    writeln!(file, "    name = {suite}_group,").unwrap();
    writeln!(file, "    benchmarks = [").unwrap();
    writeln!(file, "        {suite}_hashset_classic,").unwrap();
    writeln!(file, "        {suite}_hashset_foldhash,").unwrap();
    writeln!(file, "        {suite}_fz_string_set,").unwrap();
    writeln!(file, "        {suite}_fz_string_set_str,").unwrap();
    writeln!(file, "        {suite}_fz_string_set_compact,").unwrap();
    for &size in &sizes {
        writeln!(file, "        {suite}_fz_string_set_macro_{size},").unwrap();
    }
    writeln!(file, "    ]").unwrap();
    writeln!(file, ");").unwrap();
    writeln!(file).unwrap();
}

fn emit_iai_string_benchmark(suite: &str, literal_producer: impl Fn(&mut BufWriter<File>, usize)) {
    let sizes = [SMALL, MEDIUM, LARGE, HUGE];
    let mut file = emit_iai_preamble(&format!("gungraun_{suite}"));

    for &size in &sizes {
        emit_iai_string_suite(&mut file, suite, size, &literal_producer);
    }
    emit_iai_string_bench_fns(&mut file, suite);
    emit_iai_string_macro_bench_fns(&mut file, suite);
    emit_iai_string_group(&mut file, suite);
}

fn emit_iai_prefixed_string_benchmark() {
    fn prefixed_producer(file: &mut BufWriter<File>, size: usize) {
        let mut rng = ChaChaRng::seed_from_u64(0x1234_5678);
        for _ in 0..size {
            let len: u32 = rng.random();
            let len = (len % 10) + 5;
            let mut s = String::new();
            for _ in 0..len {
                let x: u8 = rng.random();
                let x = (x % 26) + 97;
                s.push(x as char);
            }
            writeln!(file, "        \"Color-{s}\",").unwrap();
        }
    }
    emit_iai_string_benchmark("prefixed_string", prefixed_producer);
}

fn emit_iai_random_string_benchmark() {
    fn random_producer(file: &mut BufWriter<File>, size: usize) {
        let mut rng = ChaChaRng::seed_from_u64(0x1234_5678);
        for _ in 0..size {
            let len: u32 = rng.random();
            let len = (len % 10) + 5;
            let mut s = String::new();
            for _ in 0..len {
                let x: u8 = rng.random();
                let x = (x % 26) + 97;
                s.push(x as char);
            }
            writeln!(file, "        \"{s}\",").unwrap();
        }
    }
    emit_iai_string_benchmark("random_string", random_producer);
}

// --- Hashed IAI benchmarks ---

fn emit_iai_hashed_suite(
    file: &mut BufWriter<File>,
    size: usize,
    literal_producer: &dyn Fn(&mut BufWriter<File>, usize),
) {
    let static_name = format!("FROZEN_HASHED_{size}");
    let alias_name = format!("FrozenHashed{size}Type");

    writeln!(file, "fz_hash_set!(static {static_name}: {alias_name}<Record>, {{").unwrap();
    literal_producer(file, size);
    writeln!(file, "}});").unwrap();
    writeln!(file).unwrap();

    writeln!(
        file,
        "fn data_hashed_{size}() -> (Vec<Record>, Vec<Record>) {{"
    )
    .unwrap();
    writeln!(
        file,
        "    let input: Vec<Record> = {static_name}.iter().cloned().collect();"
    )
    .unwrap();
    writeln!(file, "    let mut probe = Vec::new();").unwrap();
    writeln!(file, "    for i in &input {{").unwrap();
    writeln!(
        file,
        "        probe.push(Record {{ name: (*i).name.clone(), age: (*i).age }});"
    )
    .unwrap();
    writeln!(
        file,
        "        probe.push(Record {{ name: (*i).name.clone(), age: -(*i).age }});"
    )
    .unwrap();
    writeln!(file, "    }}").unwrap();
    writeln!(file, "    (input, probe)").unwrap();
    writeln!(file, "}}").unwrap();
    writeln!(file).unwrap();

    let collections: &[(&str, &str, &str)] = &[
        (
            "hashset_classic",
            "std::collections::HashSet<Record, std::hash::BuildHasherDefault<std::hash::DefaultHasher>>",
            "std::collections::HashSet::<_, std::hash::BuildHasherDefault<std::hash::DefaultHasher>>::from_iter(input)",
        ),
        (
            "hashset_foldhash",
            "std::collections::HashSet<Record, foldhash::fast::FixedState>",
            "std::collections::HashSet::<_, foldhash::fast::FixedState>::from_iter(input)",
        ),
        (
            "fz_hash_set",
            "FzHashSet<Record>",
            "FzHashSet::new(input)",
        ),
    ];

    for (name, typ, creator) in collections {
        writeln!(
            file,
            "fn setup_hashed_{size}_{name}() -> ({typ}, Vec<Record>) {{"
        )
        .unwrap();
        writeln!(file, "    let (input, probe) = data_hashed_{size}();").unwrap();
        writeln!(file, "    let s = {creator};").unwrap();
        writeln!(file, "    (s, probe)").unwrap();
        writeln!(file, "}}").unwrap();
        writeln!(file).unwrap();
    }

    writeln!(
        file,
        "fn setup_hashed_{size}_probe() -> Vec<Record> {{"
    )
    .unwrap();
    writeln!(file, "    data_hashed_{size}().1").unwrap();
    writeln!(file, "}}").unwrap();
    writeln!(file).unwrap();
}

fn emit_iai_hashed_bench_fns(file: &mut BufWriter<File>) {
    let sizes = [SMALL, MEDIUM, LARGE, HUGE];
    let collections: &[(&str, &str)] = &[
        (
            "hashset_classic",
            "std::collections::HashSet<Record, std::hash::BuildHasherDefault<std::hash::DefaultHasher>>",
        ),
        (
            "hashset_foldhash",
            "std::collections::HashSet<Record, foldhash::fast::FixedState>",
        ),
        ("fz_hash_set", "FzHashSet<Record>"),
    ];

    for (name, typ) in collections {
        writeln!(file, "#[library_benchmark]").unwrap();
        for &size in &sizes {
            writeln!(
                file,
                "#[bench::size_{size}(setup_hashed_{size}_{name}())]"
            )
            .unwrap();
        }
        writeln!(
            file,
            "fn hashed_{name}(data: ({typ}, Vec<Record>)) {{"
        )
        .unwrap();
        writeln!(file, "    let (ref s, ref probe) = data;").unwrap();
        emit_iai_loop(file, "s", "probe");
        writeln!(file, "}}").unwrap();
        writeln!(file).unwrap();
    }
}

fn emit_iai_hashed_macro_bench_fns(file: &mut BufWriter<File>) {
    let sizes = [SMALL, MEDIUM, LARGE, HUGE];
    for &size in &sizes {
        let static_name = format!("FROZEN_HASHED_{size}");

        writeln!(file, "#[library_benchmark]").unwrap();
        writeln!(
            file,
            "#[bench::run(setup_hashed_{size}_probe())]"
        )
        .unwrap();
        writeln!(
            file,
            "fn hashed_fz_hash_set_macro_{size}(probe: Vec<Record>) {{"
        )
        .unwrap();
        emit_iai_loop(file, &format!("&*{static_name}"), "&probe");
        writeln!(file, "}}").unwrap();
        writeln!(file).unwrap();
    }
}

fn emit_iai_hashed_group(file: &mut BufWriter<File>) {
    let sizes = [SMALL, MEDIUM, LARGE, HUGE];
    writeln!(file, "library_benchmark_group!(").unwrap();
    writeln!(file, "    name = hashed_group,").unwrap();
    writeln!(file, "    benchmarks = [").unwrap();
    writeln!(file, "        hashed_hashset_classic,").unwrap();
    writeln!(file, "        hashed_hashset_foldhash,").unwrap();
    writeln!(file, "        hashed_fz_hash_set,").unwrap();
    for &size in &sizes {
        writeln!(file, "        hashed_fz_hash_set_macro_{size},").unwrap();
    }
    writeln!(file, "    ]").unwrap();
    writeln!(file, ");").unwrap();
    writeln!(file).unwrap();
}

fn emit_iai_hashed_benchmark() {
    fn hashed_producer(file: &mut BufWriter<File>, size: usize) {
        let mut rng = ChaChaRng::seed_from_u64(0x1234_5678);
        for _ in 0..size {
            let len: u32 = rng.random();
            let len = (len % 10) + 5;
            let mut s = String::new();
            for _ in 0..len {
                let x: u8 = rng.random();
                let x = (x % 26) + 97;
                s.push(x as char);
            }
            let age: i32 = rng.random();
            writeln!(file, "        Record {{ name: \"{s}\".to_string(), age: {age} }},").unwrap();
        }
    }

    let sizes = [SMALL, MEDIUM, LARGE, HUGE];
    let mut file = emit_iai_preamble("gungraun_hashed");

    writeln!(file, "#[derive(Clone, Debug, Eq, Hash, PartialEq)]").unwrap();
    writeln!(file, "struct Record {{").unwrap();
    writeln!(file, "    name: String,").unwrap();
    writeln!(file, "    age: i32,").unwrap();
    writeln!(file, "}}").unwrap();
    writeln!(file).unwrap();

    for &size in &sizes {
        emit_iai_hashed_suite(&mut file, size, &hashed_producer);
    }
    emit_iai_hashed_bench_fns(&mut file);
    emit_iai_hashed_macro_bench_fns(&mut file);
    emit_iai_hashed_group(&mut file);
}

// --- Ordered IAI benchmarks ---

fn emit_iai_ordered_suite(
    file: &mut BufWriter<File>,
    size: usize,
    literal_producer: &dyn Fn(&mut BufWriter<File>, usize),
) {
    let static_name = format!("FROZEN_ORDERED_{size}");
    let alias_name = format!("FrozenOrdered{size}Type");

    writeln!(file, "fz_ordered_set!(static {static_name}: {alias_name}<Record>, {{").unwrap();
    literal_producer(file, size);
    writeln!(file, "}});").unwrap();
    writeln!(file).unwrap();

    writeln!(
        file,
        "fn data_ordered_{size}() -> (Vec<Record>, Vec<Record>) {{"
    )
    .unwrap();
    writeln!(
        file,
        "    let input: Vec<Record> = {static_name}.iter().cloned().collect();"
    )
    .unwrap();
    writeln!(file, "    let mut probe = Vec::new();").unwrap();
    writeln!(file, "    for i in &input {{").unwrap();
    writeln!(
        file,
        "        probe.push(Record {{ name: (*i).name.clone(), age: (*i).age }});"
    )
    .unwrap();
    writeln!(
        file,
        "        probe.push(Record {{ name: (*i).name.clone(), age: -(*i).age }});"
    )
    .unwrap();
    writeln!(file, "    }}").unwrap();
    writeln!(file, "    (input, probe)").unwrap();
    writeln!(file, "}}").unwrap();
    writeln!(file).unwrap();

    let collections: &[(&str, &str, &str)] = &[
        (
            "btreeset",
            "std::collections::BTreeSet<Record>",
            "std::collections::BTreeSet::<_>::from_iter(input)",
        ),
        (
            "fz_ordered_set",
            "FzOrderedSet<Record>",
            "FzOrderedSet::new(input)",
        ),
    ];

    for (name, typ, creator) in collections {
        writeln!(
            file,
            "fn setup_ordered_{size}_{name}() -> ({typ}, Vec<Record>) {{"
        )
        .unwrap();
        writeln!(file, "    let (input, probe) = data_ordered_{size}();").unwrap();
        writeln!(file, "    let s = {creator};").unwrap();
        writeln!(file, "    (s, probe)").unwrap();
        writeln!(file, "}}").unwrap();
        writeln!(file).unwrap();
    }

    writeln!(
        file,
        "fn setup_ordered_{size}_probe() -> Vec<Record> {{"
    )
    .unwrap();
    writeln!(file, "    data_ordered_{size}().1").unwrap();
    writeln!(file, "}}").unwrap();
    writeln!(file).unwrap();
}

fn emit_iai_ordered_bench_fns(file: &mut BufWriter<File>) {
    let sizes = [SMALL, MEDIUM, LARGE, HUGE];
    let collections: &[(&str, &str)] = &[
        (
            "btreeset",
            "std::collections::BTreeSet<Record>",
        ),
        ("fz_ordered_set", "FzOrderedSet<Record>"),
    ];

    for (name, typ) in collections {
        writeln!(file, "#[library_benchmark]").unwrap();
        for &size in &sizes {
            writeln!(
                file,
                "#[bench::size_{size}(setup_ordered_{size}_{name}())]"
            )
            .unwrap();
        }
        writeln!(
            file,
            "fn ordered_{name}(data: ({typ}, Vec<Record>)) {{"
        )
        .unwrap();
        writeln!(file, "    let (ref s, ref probe) = data;").unwrap();
        emit_iai_loop(file, "s", "probe");
        writeln!(file, "}}").unwrap();
        writeln!(file).unwrap();
    }
}

fn emit_iai_ordered_macro_bench_fns(file: &mut BufWriter<File>) {
    let sizes = [SMALL, MEDIUM, LARGE, HUGE];
    for &size in &sizes {
        let static_name = format!("FROZEN_ORDERED_{size}");

        writeln!(file, "#[library_benchmark]").unwrap();
        writeln!(
            file,
            "#[bench::run(setup_ordered_{size}_probe())]"
        )
        .unwrap();
        writeln!(
            file,
            "fn ordered_fz_ordered_set_macro_{size}(probe: Vec<Record>) {{"
        )
        .unwrap();
        emit_iai_loop(file, &format!("&*{static_name}"), "&probe");
        writeln!(file, "}}").unwrap();
        writeln!(file).unwrap();
    }
}

fn emit_iai_ordered_group(file: &mut BufWriter<File>) {
    let sizes = [SMALL, MEDIUM, LARGE, HUGE];
    writeln!(file, "library_benchmark_group!(").unwrap();
    writeln!(file, "    name = ordered_group,").unwrap();
    writeln!(file, "    benchmarks = [").unwrap();
    writeln!(file, "        ordered_btreeset,").unwrap();
    writeln!(file, "        ordered_fz_ordered_set,").unwrap();
    for &size in &sizes {
        writeln!(file, "        ordered_fz_ordered_set_macro_{size},").unwrap();
    }
    writeln!(file, "    ]").unwrap();
    writeln!(file, ");").unwrap();
    writeln!(file).unwrap();
}

fn emit_iai_ordered_benchmark() {
    fn ordered_producer(file: &mut BufWriter<File>, size: usize) {
        let mut rng = ChaChaRng::seed_from_u64(0x1234_5678);
        for _ in 0..size {
            let len: u32 = rng.random();
            let len = (len % 10) + 5;
            let mut s = String::new();
            for _ in 0..len {
                let x: u8 = rng.random();
                let x = (x % 26) + 97;
                s.push(x as char);
            }
            let age: i32 = rng.random();
            writeln!(file, "        Record {{ name: \"{s}\".to_string(), age: {age} }},").unwrap();
        }
    }

    let sizes = [SMALL, MEDIUM, LARGE, HUGE];
    let mut file = emit_iai_preamble("gungraun_ordered");

    writeln!(file, "#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]").unwrap();
    writeln!(file, "struct Record {{").unwrap();
    writeln!(file, "    name: String,").unwrap();
    writeln!(file, "    age: i32,").unwrap();
    writeln!(file, "}}").unwrap();
    writeln!(file).unwrap();

    for &size in &sizes {
        emit_iai_ordered_suite(&mut file, size, &ordered_producer);
    }
    emit_iai_ordered_bench_fns(&mut file);
    emit_iai_ordered_macro_bench_fns(&mut file);
    emit_iai_ordered_group(&mut file);
}

fn main() {
    emit_dense_scalar_benchmark();
    emit_sparse_scalar_benchmark();
    emit_random_scalar_benchmark();
    emit_prefixed_string_benchmark();
    emit_random_string_benchmark();
    emit_ordered_benchmark();
    emit_hashed_benchmark();

    emit_iai_dense_scalar_benchmark();
    emit_iai_sparse_scalar_benchmark();
    emit_iai_random_scalar_benchmark();
    emit_iai_prefixed_string_benchmark();
    emit_iai_random_string_benchmark();
    emit_iai_hashed_benchmark();
    emit_iai_ordered_benchmark();

    println!("cargo::rerun-if-changed=build.rs");
}
