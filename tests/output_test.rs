use arlon::output::format_timestamp;

#[test]
fn test_format_timestamp() {
    let timestamp = 1729180800;
    let formatted = format_timestamp(timestamp);
    assert_eq!(formatted, "2024-10-17 16:00:00");
}

#[test]
fn test_format_timestamp_zero() {
    let timestamp = 0;
    let formatted = format_timestamp(timestamp);
    assert_eq!(formatted, "1970-01-01 00:00:00");
}

#[test]
fn test_format_timestamp_negative() {
    let timestamp = -86400;
    let formatted = format_timestamp(timestamp);
    assert_eq!(formatted, "1969-12-31 00:00:00");
}

#[test]
fn test_format_timestamp_recent() {
    let timestamp = 1697500000;
    let formatted = format_timestamp(timestamp);
    assert_eq!(formatted, "2023-10-16 23:46:40");
}
