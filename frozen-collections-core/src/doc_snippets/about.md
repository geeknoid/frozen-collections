A frozen collection differs from the traditional Rust collections, such as
[`HashMap`](std::collections::HashMap) and [`HashSet`](std::collections::HashSet) types in three
key ways. First, creating a frozen collection performs an analysis over the input data to
determine the best implementation strategy. Depending on the situation, this analysis is
performed at build time or runtime, and it can take a relatively long time when a collection is
very large. Second, once created, the keys in frozen collections are immutable. And third,
querying a frozen collection is typically considerably faster, which is the whole point.

