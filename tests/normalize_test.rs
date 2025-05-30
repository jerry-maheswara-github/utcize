use utcize::datetime::normalize_datetime;

#[test]
fn parse_epoch_seconds() {
    let input = "1672531200"; // 2023-01-01T00:00:00Z
    let result = normalize_datetime::<&str>(input, "Asia/Jakarta", false, None).unwrap();

  
    println!("{:#?}", result);
}
