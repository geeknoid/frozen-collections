/// Ensure key uniqueness (assumes "keys" is a relatively small array)
pub fn any_duplicate_keys<K, const N: usize>(keys: [&K; N]) -> bool
where
    K: ?Sized + Eq,
{
    for i in 0..keys.len() {
        for j in 0..i {
            if keys[j].eq(keys[i]) {
                return true;
            }
        }
    }

    false
}
