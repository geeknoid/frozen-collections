fn make_static_collections() -> Vec<TokenStream> {
    let mut v = Vec::new();

    let entries = vec![
        CollectionEntry::set_entry("Red", parse_quote! { "Red" }),
        CollectionEntry::set_entry("Green", parse_quote! { "Green" }),
    ];

    v.push(CollectionEmitter::new(&parse_quote! { &'static str })
        .symbol_name("SMALL_STATIC_ORDERED_SET")
        .alias_name("A1")
        .static_instance(true)
        .const_keys(true)
        .const_values(true)
        .emit_ordered_collection(entries)
        .unwrap());

    let entries = vec![
        CollectionEntry::set_entry("Red", parse_quote! { "Red" }),
        CollectionEntry::set_entry("Green", parse_quote! { "Green" }),
        CollectionEntry::set_entry("Blue", parse_quote! { "Blue" }),
        CollectionEntry::set_entry("Yellow", parse_quote! { "Yellow" }),
        CollectionEntry::set_entry("Cyan", parse_quote! { "Cyan" }),
        CollectionEntry::set_entry("Magenta", parse_quote! { "Magenta" }),
        CollectionEntry::set_entry("White", parse_quote! { "White" }),
        CollectionEntry::set_entry("Black", parse_quote! { "Black" }),
        CollectionEntry::set_entry("Grey", parse_quote! { "Gray" }),
    ];

    v.push(CollectionEmitter::new(&parse_quote! { &'static str })
        .symbol_name("MEDIUM_STATIC_ORDERED_SET")
        .alias_name("A2")
        .static_instance(true)
        .const_keys(true)
        .const_values(true)
        .emit_ordered_collection(entries)
        .unwrap());

    let entries = vec![
        CollectionEntry::set_entry("Red", parse_quote! { "Red" }),
        CollectionEntry::set_entry("Green", parse_quote! { "Green" }),
        CollectionEntry::set_entry("Blue", parse_quote! { "Blue" }),
        CollectionEntry::set_entry("Yellow", parse_quote! { "Yellow" }),
        CollectionEntry::set_entry("Cyan", parse_quote! { "Cyan" }),
        CollectionEntry::set_entry("Magenta", parse_quote! { "Magenta" }),
        CollectionEntry::set_entry("White", parse_quote! { "White" }),
        CollectionEntry::set_entry("Black", parse_quote! { "Black" }),
        CollectionEntry::set_entry("Grey", parse_quote! { "Gray" }),

        CollectionEntry::set_entry("1Red", parse_quote! { "1Red" }),
        CollectionEntry::set_entry("1Green", parse_quote! { "1Green" }),
        CollectionEntry::set_entry("1Blue", parse_quote! { "1Blue" }),
        CollectionEntry::set_entry("1Yellow", parse_quote! { "1Yellow" }),
        CollectionEntry::set_entry("1Cyan", parse_quote! { "1Cyan" }),
        CollectionEntry::set_entry("1Magenta", parse_quote! { "1Magenta" }),
        CollectionEntry::set_entry("1White", parse_quote! { "1White" }),
        CollectionEntry::set_entry("1Black", parse_quote! { "1Black" }),
        CollectionEntry::set_entry("1Grey", parse_quote! { "1Gray" }),

        CollectionEntry::set_entry("2Red", parse_quote! { "2Red" }),
        CollectionEntry::set_entry("2Green", parse_quote! { "2Green" }),
        CollectionEntry::set_entry("2Blue", parse_quote! { "2Blue" }),
        CollectionEntry::set_entry("2Yellow", parse_quote! { "2Yellow" }),
        CollectionEntry::set_entry("2Cyan", parse_quote! { "2Cyan" }),
        CollectionEntry::set_entry("2Magenta", parse_quote! { "2Magenta" }),
        CollectionEntry::set_entry("2White", parse_quote! { "2White" }),
        CollectionEntry::set_entry("2Black", parse_quote! { "2Black" }),
        CollectionEntry::set_entry("2Grey", parse_quote! { "2Gray" }),

        CollectionEntry::set_entry("3Red", parse_quote! { "3Red" }),
        CollectionEntry::set_entry("3Green", parse_quote! { "3Green" }),
        CollectionEntry::set_entry("3Blue", parse_quote! { "3Blue" }),
        CollectionEntry::set_entry("3Yellow", parse_quote! { "3Yellow" }),
        CollectionEntry::set_entry("3Cyan", parse_quote! { "3Cyan" }),
        CollectionEntry::set_entry("3Magenta", parse_quote! { "3Magenta" }),
        CollectionEntry::set_entry("3White", parse_quote! { "3White" }),
        CollectionEntry::set_entry("3Black", parse_quote! { "3Black" }),
        CollectionEntry::set_entry("3Grey", parse_quote! { "3Gray" }),

        CollectionEntry::set_entry("4Red", parse_quote! { "4Red" }),
        CollectionEntry::set_entry("4Green", parse_quote! { "4Green" }),
        CollectionEntry::set_entry("4Blue", parse_quote! { "4Blue" }),
        CollectionEntry::set_entry("4Yellow", parse_quote! { "4Yellow" }),
        CollectionEntry::set_entry("4Cyan", parse_quote! { "4Cyan" }),
        CollectionEntry::set_entry("4Magenta", parse_quote! { "4Magenta" }),
        CollectionEntry::set_entry("4White", parse_quote! { "4White" }),
        CollectionEntry::set_entry("4Black", parse_quote! { "4Black" }),
        CollectionEntry::set_entry("4Grey", parse_quote! { "4Gray" }),

        CollectionEntry::set_entry("5Red", parse_quote! { "5Red" }),
        CollectionEntry::set_entry("5Green", parse_quote! { "5Green" }),
        CollectionEntry::set_entry("5Blue", parse_quote! { "5Blue" }),
        CollectionEntry::set_entry("5Yellow", parse_quote! { "5Yellow" }),
        CollectionEntry::set_entry("5Cyan", parse_quote! { "5Cyan" }),
        CollectionEntry::set_entry("5Magenta", parse_quote! { "5Magenta" }),
        CollectionEntry::set_entry("5White", parse_quote! { "5White" }),
        CollectionEntry::set_entry("5Black", parse_quote! { "5Black" }),
        CollectionEntry::set_entry("5Grey", parse_quote! { "5Gray" }),

        CollectionEntry::set_entry("6Red", parse_quote! { "6Red" }),
        CollectionEntry::set_entry("6Green", parse_quote! { "6Green" }),
        CollectionEntry::set_entry("6Blue", parse_quote! { "6Blue" }),
        CollectionEntry::set_entry("6Yellow", parse_quote! { "6Yellow" }),
        CollectionEntry::set_entry("6Cyan", parse_quote! { "6Cyan" }),
        CollectionEntry::set_entry("6Magenta", parse_quote! { "6Magenta" }),
        CollectionEntry::set_entry("6White", parse_quote! { "6White" }),
        CollectionEntry::set_entry("6Black", parse_quote! { "6Black" }),
        CollectionEntry::set_entry("6Grey", parse_quote! { "6Gray" }),

        CollectionEntry::set_entry("7Red", parse_quote! { "7Red" }),
        CollectionEntry::set_entry("7Green", parse_quote! { "7Green" }),
        CollectionEntry::set_entry("7Blue", parse_quote! { "7Blue" }),
        CollectionEntry::set_entry("7Yellow", parse_quote! { "7Yellow" }),
        CollectionEntry::set_entry("7Cyan", parse_quote! { "7Cyan" }),
        CollectionEntry::set_entry("7Magenta", parse_quote! { "7Magenta" }),
        CollectionEntry::set_entry("7White", parse_quote! { "7White" }),
        CollectionEntry::set_entry("7Black", parse_quote! { "7Black" }),
        CollectionEntry::set_entry("7Grey", parse_quote! { "7Gray" }),
    ];

    v.push(CollectionEmitter::new(&parse_quote! { &'static str })
        .symbol_name("LARGE_STATIC_ORDERED_SET")
        .alias_name("A3")
        .static_instance(true)
        .const_keys(true)
        .const_values(true)
        .emit_ordered_collection(entries)
        .unwrap());

    let entries = vec![
        CollectionEntry::set_entry("Red", parse_quote! { "Red" }),
        CollectionEntry::set_entry("Green", parse_quote! { "Green" }),
    ];

    v.push(CollectionEmitter::new(&parse_quote! { &'static str })
        .symbol_name("SMALL_STATIC_HASH_SET")
        .static_instance(true)
        .const_keys(true)
        .const_values(true)
        .emit_hash_collection(entries)
        .unwrap());

    let entries = vec![
        CollectionEntry::set_entry("Red", parse_quote! { "Red" }),
        CollectionEntry::set_entry("Green", parse_quote! { "Green" }),
        CollectionEntry::set_entry("Blue", parse_quote! { "Blue" }),
        CollectionEntry::set_entry("Yellow", parse_quote! { "Yellow" }),
        CollectionEntry::set_entry("Cyan", parse_quote! { "Cyan" }),
        CollectionEntry::set_entry("Magenta", parse_quote! { "Magenta" }),
        CollectionEntry::set_entry("White", parse_quote! { "White" }),
        CollectionEntry::set_entry("Black", parse_quote! { "Black" }),
        CollectionEntry::set_entry("Grey", parse_quote! { "Gray" }),
    ];

    v.push(CollectionEmitter::new(&parse_quote! { &'static str })
        .symbol_name("MEDIUM_STATIC_HASH_SET")
        .static_instance(true)
        .const_values(true)
        .emit_hash_collection(entries)
        .unwrap());

    v
}
