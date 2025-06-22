pub fn variant_to_str<T>(_: &T) -> &'static str {
    std::any::type_name::<T>().rsplit("::").next().unwrap()
}

pub fn varient_to_string<T>(_: &T) -> String {
    std::any::type_name::<T>()
        .rsplit("::")
        .next()
        .unwrap()
        .to_string()
}
