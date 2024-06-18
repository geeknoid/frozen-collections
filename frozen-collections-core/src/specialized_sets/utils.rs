macro_rules! partial_eq {
    () => {
        fn eq(&self, other: &ST) -> bool {
            if self.len() != other.len() {
                return false;
            }

            self.iter().all(|value| other.contains(value))
        }
    };
}

pub(crate) use partial_eq;
