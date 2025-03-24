use winauto::Variant;

#[test]
fn test_variant_from_str() {
    let input_value = "hello";
    let variant = Variant::from(input_value);
    let output_value: String = variant.try_into().unwrap();
    assert_eq!(&output_value, input_value);
}

#[test]
fn test_variant_from_string() {
    let input_value = String::from("hello");
    let variant = Variant::from(input_value.clone());
    let output_value: String = variant.try_into().unwrap();
    assert_eq!(output_value, input_value);
}

#[test]
fn test_variant_from_bool() {
    let input_value = true;
    let variant = Variant::from(input_value);
    let output_value: bool = variant.try_into().unwrap();
    assert_eq!(output_value, input_value);
}

#[test]
fn test_variant_from_f32() {
    let input_value = 42.0;
    let variant = Variant::from(input_value);
    let output_value: f64 = variant.try_into().unwrap();
    assert_eq!(output_value as f32, input_value);
}

#[test]
fn test_variant_from_f64() {
    let input_value = 42.0;
    let variant = Variant::from(input_value);
    let output_value: f64 = variant.try_into().unwrap();
    assert_eq!(output_value, input_value);
}

#[test]
fn test_variant_from_i8() {
    let input_value = 42;
    let variant = Variant::from(input_value);
    let output_value: i16 = variant.try_into().unwrap();
    assert_eq!(output_value as i8, input_value);
}

#[test]
fn test_variant_from_i16() {
    let input_value = 42;
    let variant = Variant::from(input_value);
    let output_value: i16 = variant.try_into().unwrap();
    assert_eq!(output_value, input_value);
}

#[test]
fn test_variant_from_i32() {
    let input_value = 42;
    let variant = Variant::from(input_value);
    let output_value: i32 = variant.try_into().unwrap();
    assert_eq!(output_value, input_value);
}

#[test]
fn test_variant_from_i64() {
    let input_value = 42;
    let variant = Variant::from(input_value);
    let output_value: i64 = variant.try_into().unwrap();
    assert_eq!(output_value, input_value);
}

#[test]
fn test_variant_from_u8() {
    let input_value = 42;
    let variant = Variant::from(input_value);
    let output_value: u16 = variant.try_into().unwrap();
    assert_eq!(output_value as u8, input_value);
}

#[test]
fn test_variant_from_u16() {
    let input_value = 42;
    let variant = Variant::from(input_value);
    let output_value: u16 = variant.try_into().unwrap();
    assert_eq!(output_value, input_value);
}

#[test]
fn test_variant_from_u32() {
    let input_value = 42;
    let variant = Variant::from(input_value);
    let output_value: u32 = variant.try_into().unwrap();
    assert_eq!(output_value, input_value);
}

#[test]
fn test_variant_from_u64() {
    let input_value = 42;
    let variant = Variant::from(input_value);
    let output_value: u64 = variant.try_into().unwrap();
    assert_eq!(output_value, input_value);
}
