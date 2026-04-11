use date::cli::DateConfig;

fn parse(args: &[&str]) -> Option<DateConfig> {
    let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    DateConfig::from_args(&args)
}

#[test]
fn default_no_args() {
    let config = parse(&[]).unwrap();
    assert_eq!(config, DateConfig::default());
}

#[test]
fn format_argument() {
    let config = parse(&["+%Y-%m-%d"]).unwrap();
    assert_eq!(config.format, Some("%Y-%m-%d".to_string()));
}

#[test]
fn utc_short() {
    let config = parse(&["-u"]).unwrap();
    assert!(config.utc);
}

#[test]
fn utc_long() {
    let config = parse(&["--utc"]).unwrap();
    assert!(config.utc);
}

#[test]
fn utc_universal() {
    let config = parse(&["--universal"]).unwrap();
    assert!(config.utc);
}

#[test]
fn date_string_short() {
    let config = parse(&["-d", "2025-04-11 14:30:00"]).unwrap();
    assert_eq!(config.date_string, Some("2025-04-11 14:30:00".to_string()));
}

#[test]
fn date_string_long() {
    let config = parse(&["--date=2025-04-11"]).unwrap();
    assert_eq!(config.date_string, Some("2025-04-11".to_string()));
}

#[test]
fn date_string_long_separate() {
    let config = parse(&["--date", "2025-04-11"]).unwrap();
    assert_eq!(config.date_string, Some("2025-04-11".to_string()));
}

#[test]
fn iso_default() {
    let config = parse(&["-I"]).unwrap();
    assert_eq!(config.iso_format, Some("date".to_string()));
}

#[test]
fn iso_long_default() {
    let config = parse(&["--iso-8601"]).unwrap();
    assert_eq!(config.iso_format, Some("date".to_string()));
}

#[test]
fn iso_with_spec() {
    let config = parse(&["--iso-8601=seconds"]).unwrap();
    assert_eq!(config.iso_format, Some("seconds".to_string()));
}

#[test]
fn rfc_email_short() {
    let config = parse(&["-R"]).unwrap();
    assert!(config.rfc_email);
}

#[test]
fn rfc_email_long() {
    let config = parse(&["--rfc-email"]).unwrap();
    assert!(config.rfc_email);
}

#[test]
fn rfc_3339_long() {
    let config = parse(&["--rfc-3339=seconds"]).unwrap();
    assert_eq!(config.rfc_3339, Some("seconds".to_string()));
}

#[test]
fn rfc_3339_separate() {
    let config = parse(&["--rfc-3339", "ns"]).unwrap();
    assert_eq!(config.rfc_3339, Some("ns".to_string()));
}

#[test]
fn reference_short() {
    let config = parse(&["-r", "somefile.txt"]).unwrap();
    assert_eq!(config.reference, Some("somefile.txt".to_string()));
}

#[test]
fn reference_long() {
    let config = parse(&["--reference=somefile.txt"]).unwrap();
    assert_eq!(config.reference, Some("somefile.txt".to_string()));
}

#[test]
fn combined_flags() {
    let config = parse(&["-uR"]).unwrap();
    assert!(config.utc);
    assert!(config.rfc_email);
}

#[test]
fn utc_with_format() {
    let config = parse(&["-u", "+%Y-%m-%d"]).unwrap();
    assert!(config.utc);
    assert_eq!(config.format, Some("%Y-%m-%d".to_string()));
}

#[test]
fn help_returns_none() {
    assert!(parse(&["--help"]).is_none());
}

#[test]
fn version_returns_none() {
    assert!(parse(&["--version"]).is_none());
}
