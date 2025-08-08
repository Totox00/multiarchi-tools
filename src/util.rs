use hashlink::LinkedHashMap;
use rand::thread_rng;
use rand_distr::{Distribution, WeightedIndex};
use yaml_rust2::Yaml;

pub fn as_i64(yaml: &Yaml) -> Option<i64> {
    if let Some(value) = yaml.as_i64() {
        return Some(value);
    }

    if let Some(value) = yaml.as_f64() {
        return Some(value as i64);
    }

    None
}

pub fn resolve_weighted_option(hash: &mut LinkedHashMap<Yaml, Yaml>, key: &str) {
    if let Some(values) = hash.get_mut(&Yaml::from_str(key)) {
        let new_value = if let Some(values_hash) = values.as_mut_hash() {
            let mut rng = thread_rng();

            let options: Vec<_> = values_hash.iter().filter_map(|(k, v)| as_i64(v).map(|weight| (k, weight))).collect();

            let dist = WeightedIndex::new(options.iter().map(|(_, weight)| weight)).expect("Failed to create index");

            Some(options[dist.sample(&mut rng)].0.to_owned())
        } else {
            None
        };

        if let Some(new_value) = new_value {
            *values = new_value;
        }
    }
}
