use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Debug, Clone, Eq)]
pub struct Map<K: Eq + Hash + Ord, V: PartialEq + Hash + Ord>(pub HashMap<K, V>);

impl<K: Eq + Hash + Ord, V: PartialEq + Hash + Ord> Map<K, V> {
    pub fn new() -> Map<K, V> {
        Map(HashMap::new())
    }
}

#[macro_export]
macro_rules! map {
    ( $( { $k:expr, $v:expr } ),* $(,)? ) => {
        {
            let mut temp_map = Map::new();
            $(
                temp_map.insert($k, $v);
            )*
            temp_map
        }
    };
}

impl<K: Eq + Hash + Ord, V: PartialEq + Hash + Ord> PartialEq for Map<K, V> {
    fn eq(&self, other: &Self) -> bool {
        if self.0.keys().count() != other.0.keys().count() {
            return false;
        }

        for (k, v) in self.0.iter() {
            if other.0.get(k) != Some(v) {
                return false;
            }
        }
        true
    }
}

impl<K: Eq + Hash + Ord, V: PartialEq + Hash + Ord> PartialOrd for Map<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Eq + Hash + Ord, V: PartialEq + Hash + Ord> Ord for Map<K, V> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut s: Vec<_> = self.0.iter().collect();
        s.sort_by_key(|(k, _)| *k);
        let mut o: Vec<_> = other.0.iter().collect();
        o.sort_by_key(|(k, _)| *k);
        s.cmp(&o)
    }
}

impl<K: Eq + Hash + Ord, V: PartialEq + Hash + Ord> Hash for Map<K, V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut pairs: Vec<_> = self.0.iter().collect();
        pairs.sort_by_key(|(k, _)| *k);

        for (k, v) in pairs {
            k.hash(state);
            v.hash(state);
        }
    }
}

impl<K: Eq + Hash + Ord, V: PartialEq + Hash + Ord> Deref for Map<K, V> {
    type Target = HashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K: Eq + Hash + Ord, V: PartialEq + Hash + Ord> DerefMut for Map<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_eq() {
        let mut map = Map(HashMap::new());
        map.insert('a', 42);
        map.insert('c', 56);

        let mut eq = Map(HashMap::new());
        eq.insert('c', 56);
        eq.insert('a', 42);

        let mut diff = Map(HashMap::new());
        diff.insert('c', 56);
        diff.insert('a', 42);
        diff.insert('j', 79);

        assert_eq!(map, eq);
        assert_ne!(map, diff);
    }

    #[test]
    fn test_constructor_macro() {
        let mut map = Map(HashMap::new());
        map.insert('a', 42);
        map.insert('c', 56);
        assert_eq!(map, map![{'a', 42}, {'c', 56}],);
    }

    #[test]
    fn test_nested() {
        let one = map![{'a', 42}, {'c', 56}];
        let two = map![{'d', 78}, {'h', 99}];
        let nested = map![
            {'a', one.clone()},
            {'y', two.clone()},
        ];

        assert_eq!(nested.0[&'a'], one);
        assert_eq!(nested.0[&'y'], two);

        let nested = map![
            {one.clone(), 'a'},
            {two.clone(), 'y'},
        ];

        assert_eq!(nested.0.get(&one), Some(&'a'));
        assert_eq!(nested.0.get(&two), Some(&'y'));
    }
}
