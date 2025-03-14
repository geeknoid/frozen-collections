use rand::Rng;
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

fn emit_benchmark_preamble(name: &str) -> BufWriter<File> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(format!("{name}.rs"));
    let mut file = BufWriter::new(File::create(dest_path).unwrap());

    writeln!(file, "#[allow(clippy::unreadable_literal)]").unwrap();
    writeln!(file, "#[allow(clippy::items_after_statements)]").unwrap();
    writeln!(file, "#[allow(clippy::explicit_auto_deref)]").unwrap();
    writeln!(file, "#[allow(clippy::redundant_closure_for_method_calls)]").unwrap();
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
    writeln!(file, "                _ = black_box(s.contains(key));").unwrap();
    writeln!(file, "            }}").unwrap();
    writeln!(file, "        }});").unwrap();
    writeln!(file, "    }});").unwrap();
}

fn emit_scalar_suite<F>(file: &mut BufWriter<File>, size: usize, literal_producer: F)
where
    F: Fn(&mut BufWriter<File>, usize),
{
    writeln!(file, "    let frozen = fz_scalar_set!({{").unwrap();
    literal_producer(file, size);
    writeln!(file, "    }});").unwrap();

    writeln!(
        file,
        "    let input: Vec<i32> = frozen.clone().into_iter().collect();"
    )
    .unwrap();
    writeln!(file, "    let size = input.len();").unwrap();
    writeln!(file, "    let mut probe = Vec::new();").unwrap();
    writeln!(file, "    for i in &input {{").unwrap();
    writeln!(file, "        probe.push(*i);").unwrap();
    writeln!(file, "        probe.push(-(*i));").unwrap();
    writeln!(file, "    }}").unwrap();

    writeln!(file, "    let s = std::collections::HashSet::<_, std::hash::RandomState>::from_iter(input.clone());").unwrap();
    emit_loop(file, "HashSet(classic)");

    writeln!(
        file,
        "    let s = std::collections::HashSet::<_, foldhash::fast::RandomState>::from_iter(input.clone());"
    )
    .unwrap();
    emit_loop(file, "HashSet(foldhash)");

    writeln!(file, "    let s = FzScalarSet::new(input);").unwrap();
    emit_loop(file, "FzScalarSet");

    writeln!(file, "    let s = frozen;").unwrap();
    emit_loop(file, "fz_scalar_set");
}

fn emit_string_suite<F>(file: &mut BufWriter<File>, size: usize, literal_producer: F)
where
    F: Fn(&mut BufWriter<File>, usize),
{
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

    writeln!(file, "    let s = std::collections::HashSet::<_, std::hash::RandomState>::from_iter(input.clone());").unwrap();
    emit_loop(file, "HashSet(classic)");

    writeln!(
        file,
        "    let s = std::collections::HashSet::<_, foldhash::fast::RandomState>::from_iter(input.clone());"
    )
    .unwrap();
    emit_loop(file, "HashSet(foldhash)");

    writeln!(file, "    let s = FzStringSet::new(input);").unwrap();
    emit_loop(file, "FzStringSet");

    writeln!(file, "    let s = frozen;").unwrap();
    emit_loop(file, "fz_string_set");
}

fn emit_hashed_suite<F>(file: &mut BufWriter<File>, size: usize, literal_producer: F)
where
    F: Fn(&mut BufWriter<File>, usize),
{
    writeln!(file, "    let frozen = fz_hash_set!({{").unwrap();
    literal_producer(file, size);
    writeln!(file, "    }});").unwrap();

    writeln!(
        file,
        "    let input: Vec<_> = frozen.clone().into_iter().collect();"
    )
    .unwrap();
    writeln!(file, "    let size = input.len();").unwrap();
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

    writeln!(file, "    let s = std::collections::HashSet::<_, std::hash::RandomState>::from_iter(input.clone());").unwrap();
    emit_loop(file, "HashSet(classic)");

    writeln!(
        file,
        "    let s = std::collections::HashSet::<_, foldhash::fast::RandomState>::from_iter(input.clone());"
    )
    .unwrap();
    emit_loop(file, "HashSet(foldhash)");

    writeln!(file, "    let s = FzHashSet::new(input);").unwrap();
    emit_loop(file, "FzHashSet");

    writeln!(file, "    let s = frozen;").unwrap();
    emit_loop(file, "fz_hash_set");
}

fn emit_ordered_suite<F>(file: &mut BufWriter<File>, size: usize, literal_producer: F)
where
    F: Fn(&mut BufWriter<File>, usize),
{
    writeln!(file, "    let frozen = fz_ordered_set!({{").unwrap();
    literal_producer(file, size);
    writeln!(file, "    }});").unwrap();

    writeln!(
        file,
        "    let input: Vec<_> = frozen.clone().into_iter().collect();"
    )
    .unwrap();
    writeln!(file, "    let size = input.len();").unwrap();
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

    writeln!(
        file,
        "    let s = std::collections::BTreeSet::<_>::from_iter(input.clone());"
    )
    .unwrap();
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
            writeln!(
                file,
                "        Record {{ name: \"{s}\".to_string(), age: {age} }},"
            )
            .unwrap();
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
            writeln!(
                file,
                "        Record {{ name: \"{s}\".to_string(), age: {age} }},"
            )
            .unwrap();
        }
    }

    let mut file = emit_benchmark_preamble("ordered");

    writeln!(
        file,
        "#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]"
    )
    .unwrap();
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

fn main() {
    emit_dense_scalar_benchmark();
    emit_sparse_scalar_benchmark();
    emit_random_scalar_benchmark();
    emit_prefixed_string_benchmark();
    emit_random_string_benchmark();
    emit_ordered_benchmark();
    emit_hashed_benchmark();

    println!("cargo::rerun-if-changed=build.rs");
}
