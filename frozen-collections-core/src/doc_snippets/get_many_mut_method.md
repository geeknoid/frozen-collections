Attempts to get mutable references to `N` values in the map at once.

Returns an array of length `N` with the results of each query. For soundness, at most one
mutable reference will be returned to any value. [`None`] will be returned if any of the
keys are duplicates or missing.
