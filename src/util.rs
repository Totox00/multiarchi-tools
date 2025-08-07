use yaml_rust2::Yaml;

pub fn as_i64(yaml: &Yaml) -> Option<i64> {
    if let Some(value) = yaml.as_i64() {
        return Some(value);
    }
    
    if let Some(value) = yaml.as_f64() {
        return Some(value as i64);
    }

    return None;
}
