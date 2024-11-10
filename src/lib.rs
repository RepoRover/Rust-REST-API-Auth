use std::{collections::HashSet, hash::Hash};

mod controller;
pub mod server;
mod var;

/// Function to find items in `required` that are missing from `provided`
pub fn find_missing<T>(
    required_collection: &HashSet<T>,
    provided_collection: &HashSet<T>,
) -> HashSet<T>
where
    T: Eq + Clone + Hash,
{
    required_collection
        .iter()
        .filter(|&item| !provided_collection.contains(item))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_missing_single_missing() {
        let missing: (&str, &str) = ("key2", "val2");
        let present: (&str, &str) = ("key1", "val1");

        let required: HashSet<(&str, &str)> = HashSet::from([present, missing]);
        let provided: HashSet<(&str, &str)> = HashSet::from([present]);

        let expected: HashSet<(&str, &str)> = HashSet::from([missing]);
        let result: HashSet<(&str, &str)> = find_missing(&required, &provided);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_find_missing_no_required() {
        let required: HashSet<(&str, &str)> = HashSet::new();
        let provided: HashSet<(&str, &str)> = HashSet::from([("key1", "val1")]);

        let expected: HashSet<(&str, &str)> = HashSet::from([]);
        let result: HashSet<(&str, &str)> = find_missing(&required, &provided);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_find_missing_no_provided() {
        let required: HashSet<(&str, &str)> = HashSet::from([("key1", "val1"), ("key2", "val2")]);
        let provided: HashSet<(&str, &str)> = HashSet::new();

        let expected: HashSet<(&str, &str)> = HashSet::from([("key1", "val1"), ("key2", "val2")]);
        let result: HashSet<(&str, &str)> = find_missing(&required, &provided);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_find_missing_no_provided_and_no_missing() {
        let required: HashSet<(&str, &str)> = HashSet::from([]);
        let provided: HashSet<(&str, &str)> = HashSet::new();

        let expected: HashSet<(&str, &str)> = HashSet::from([]);
        let result: HashSet<(&str, &str)> = find_missing(&required, &provided);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_find_missing_no_missing() {
        let required: HashSet<(&str, &str)> = HashSet::from([("key1", "val1"), ("key2", "val2")]);
        let provided: HashSet<(&str, &str)> = required.clone();

        let expected: HashSet<(&str, &str)> = HashSet::from([]);
        let result: HashSet<(&str, &str)> = find_missing(&required, &provided);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_find_missing_multiple_missing() {
        let required: HashSet<(&str, &str)> =
            HashSet::from([("key1", "val1"), ("key2", "val2"), ("key3", "val3")]);
        let provided: HashSet<(&str, &str)> = HashSet::from([("key1", "val1")]);

        let expected: HashSet<(&str, &str)> = HashSet::from([("key2", "val2"), ("key3", "val3")]);
        let result: HashSet<(&str, &str)> = find_missing(&required, &provided);

        assert_eq!(expected, result);
    }
}
