extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use core::hint::black_box;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use hashbrown::HashMap as HashbrownMap;

use frozen_collections::fz_string_map;
use frozen_collections::hashers::{BridgeHasher, PassthroughHasher, RightRangeHasher};
use frozen_collections::maps::*;

#[allow(clippy::useless_conversion)]
fn string_keys_passthrough_hasher(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_keys_passthrough_hasher");

    let frozen = fz_string_map!({
        "1": 1,
        "22": 2,
        "333": 3,
        "4444": 4,
        "55555": 5,
        "666666": 6,
        "7777777": 7,
        "88888888": 8,
        "999999999": 9,
    });

    let input: Vec<(&str, i32)> = frozen.clone().into_iter().collect();

    // 50% hit rate
    let probe = [
        "1",
        "22",
        "333",
        "4444",
        "55555",
        "666666",
        "7777777",
        "88888888",
        "999999999",
        "x1",
        "22x",
        "x333",
        "4444x",
        "x55555",
        "666666x",
        "x7777777",
        "88888888x",
        "x999999999",
    ];

    let map = HashbrownMap::<_, _, std::hash::RandomState>::from_iter(input.clone().into_iter());
    group.bench_function("HashbrownMap", |b| {
        b.iter(|| {
            for key in probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map = HashbrownMap::<_, _, ahash::RandomState>::from_iter(input.clone().into_iter());
    group.bench_function("HashbrownMap(ahash)", |b| {
        b.iter(|| {
            for key in probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map = HashMap::<_, _, _>::new(input, PassthroughHasher::default());
    group.bench_function("HashMap", |b| {
        b.iter(|| {
            for key in probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map = frozen;
    group.bench_function("InlineHashMap", |b| {
        b.iter(|| {
            for key in probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    group.finish();
}

#[allow(clippy::useless_conversion)]
fn string_keys_right_range_hasher(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_keys_right_range_hasher");

    let frozen = fz_string_map!({
        "Red": 1,
        "Green": 2,
        "Blue": 3,
        "Cyan": 4,
        "Yellow": 5,
        "Magenta": 6,
        "Purple": 7,
        "Orange": 8,
        "Maroon": 9,
        "Lilac": 10,
        "Burgundy": 11,
        "Peach": 12,
        "White": 13,
        "Black": 14,
        "Brown": 15,
        "Beige": 16,
        "Grey": 17,
        "Ecru": 18,
        "Tan": 19,
        "Lavender": 20,
    });

    let input: Vec<(&str, i32)> = frozen.clone().into_iter().collect();

    // 50% hit rate
    let probe = [
        "Red",
        "Green",
        "Blue",
        "Cyan",
        "Yellow",
        "Magenta",
        "Purple",
        "Orange",
        "Maroon",
        "Lilac",
        "Burgundy",
        "Peach",
        "White",
        "Black",
        "Brown",
        "Beige",
        "Grey",
        "Ecru",
        "Tan",
        "Lavender",
        "RedX",
        "XGreen",
        "BlueX",
        "XCyan",
        "YellowX",
        "XMagenta",
        "PurpleX",
        "XOrange",
        "MaroonX",
        "XLilac",
        "BurgundyX",
        "XPeach",
        "WhiteX",
        "XBlack",
        "BrownX",
        "XBeige",
        "GreyX",
        "XEcru",
        "TanX",
        "XLavender",
    ];

    let map = HashbrownMap::<_, _, std::hash::RandomState>::from_iter(input.clone().into_iter());
    group.bench_function("HashbrownMap", |b| {
        b.iter(|| {
            for key in probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map = HashbrownMap::<_, _, ahash::RandomState>::from_iter(input.clone().into_iter());
    group.bench_function("HashbrownMap(ahash)", |b| {
        b.iter(|| {
            for key in probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map = HashMap::<_, _, _>::new(
        input,
        RightRangeHasher::new(ahash::RandomState::new(), 1..3),
    );
    group.bench_function("HashMap", |b| {
        b.iter(|| {
            for key in probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map = frozen;
    group.bench_function("InlineHashMap", |b| {
        b.iter(|| {
            for key in probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    group.finish();
}

fn string_keys_bridge_hasher(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_keys_bridge_hasher");

    for size in 10..15 {
        let mut input = vec![
            ("Red", 1),
            ("Green", 2),
            ("Blue", 3),
            ("Cyan", 4),
            ("Yellow", 5),
            ("Magenta", 6),
            ("Purple", 7),
            ("Orange", 8),
            ("Maroon", 9),
            ("Lilac", 10),
            ("Burgundy", 11),
            ("Peach", 12),
            ("White", 13),
            ("Black", 14),
            ("Brown", 15),
            ("Beige", 16),
            ("Grey", 17),
            ("Ecru", 18),
            ("Tan", 19),
            ("Lavender", 20),
        ];

        let mut probe = vec![
            "Red",
            "XRed",
            "Green",
            "GreenX",
            "Blue",
            "XBlue",
            "Cyan",
            "CyanX",
            "Yellow",
            "XYellow",
            "Magenta",
            "MagentaX",
            "Purple",
            "XPurple",
            "Orange",
            "OrangeX",
            "Maroon",
            "XMaroon",
            "Lilac",
            "LilacX",
            "Burgundy",
            "XBurgundy",
            "Peach",
            "PeachX",
            "White",
            "XWhite",
            "Black",
            "BlackX",
            "Brown",
            "XBrown",
            "Beige",
            "BeigeX",
            "Grey",
            "XGrey",
            "Ecru",
            "EcruX",
            "Tan",
            "XTan",
            "Lavender",
            "LavenderX",
        ];

        input.truncate(size);

        // 50% hit rate
        probe.truncate(size * 2);

        let map = HashbrownMap::<_, _, std::hash::RandomState>::from_iter(input.clone());
        group.bench_with_input(BenchmarkId::new("HashbrownMap", size), &size, |b, _| {
            b.iter(|| {
                for key in &probe {
                    _ = black_box(map.contains_key(key));
                }
            });
        });

        let map = HashbrownMap::<_, _, ahash::RandomState>::from_iter(input.clone());
        group.bench_with_input(
            BenchmarkId::new("HashbrownMap(ahash)", size),
            &size,
            |b, _| {
                b.iter(|| {
                    for key in &probe {
                        _ = black_box(map.contains_key(key));
                    }
                });
            },
        );

        let map = HashMap::<_, _, _>::new(
            input.clone(),
            BridgeHasher::new(ahash::RandomState::default()),
        );
        group.bench_with_input(BenchmarkId::new("HashMap", size), &size, |b, _| {
            b.iter(|| {
                for key in &probe {
                    _ = black_box(map.contains_key(key));
                }
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    string_keys_bridge_hasher,
    string_keys_passthrough_hasher,
    string_keys_right_range_hasher,
);

criterion_main!(benches);
