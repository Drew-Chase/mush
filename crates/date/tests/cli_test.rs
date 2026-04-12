use clap::Parser;

use date::cli::DateConfig;

fn parse(args: &[&str]) -> DateConfig {
    let mut full = vec!["date"];
    // Filter out +FORMAT args before passing to clap (clap can't handle them)
    let clap_args: Vec<&&str> = args.iter().filter(|a| !a.starts_with('+')).collect();
    let mut clap_full = vec!["date"];
    clap_full.extend(clap_args.iter().map(|a| **a));
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    DateConfig::parse_from(clap_full).resolve(&owned)
}

#[test]
fn default_no_args() {
    let config = parse(&[]);
    assert_eq!(config.format, None);
    assert_eq!(config.date_string, None);
    assert!(!config.utc);
    assert_eq!(config.iso_format, None);
    assert!(!config.rfc_email);
    assert_eq!(config.rfc_3339, None);
    assert_eq!(config.reference, None);
}

#[test]
fn format_argument() {
    let config = parse(&["+%Y-%m-%d"]);
    assert_eq!(config.format, Some("%Y-%m-%d".to_string()));
}

#[test]
fn utc_short() {
    let config = parse(&["-u"]);
    assert!(config.utc);
}

#[test]
fn utc_long() {
    let config = parse(&["--utc"]);
    assert!(config.utc);
}

#[test]
fn utc_universal() {
    let config = parse(&["--universal"]);
    assert!(config.utc);
}

#[test]
fn date_string_short() {
    let config = parse(&["-d", "2025-04-11 14:30:00"]);
    assert_eq!(config.date_string, Some("2025-04-11 14:30:00".to_string()));
}

#[test]
fn date_string_long() {
    let config = parse(&["--date=2025-04-11"]);
    assert_eq!(config.date_string, Some("2025-04-11".to_string()));
}

#[test]
fn date_string_long_separate() {
    let config = parse(&["--date", "2025-04-11"]);
    assert_eq!(config.date_string, Some("2025-04-11".to_string()));
}

#[test]
fn iso_default() {
    let config = parse(&["--iso-8601=date"]);
    assert_eq!(config.iso_format, Some("date".to_string()));
}

#[test]
fn iso_with_spec() {
    let config = parse(&["--iso-8601=seconds"]);
    assert_eq!(config.iso_format, Some("seconds".to_string()));
}

#[test]
fn rfc_email_short() {
    let config = parse(&["-R"]);
    assert!(config.rfc_email);
}

#[test]
fn rfc_email_long() {
    let config = parse(&["--rfc-email"]);
    assert!(config.rfc_email);
}

#[test]
fn rfc_3339_long() {
    let config = parse(&["--rfc-3339=seconds"]);
    assert_eq!(config.rfc_3339, Some("seconds".to_string()));
}

#[test]
fn rfc_3339_separate() {
    let config = parse(&["--rfc-3339", "ns"]);
    assert_eq!(config.rfc_3339, Some("ns".to_string()));
}

#[test]
fn reference_short() {
    let config = parse(&["-r", "somefile.txt"]);
    assert_eq!(config.reference, Some("somefile.txt".to_string()));
}

#[test]
fn reference_long() {
    let config = parse(&["--reference=somefile.txt"]);
    assert_eq!(config.reference, Some("somefile.txt".to_string()));
}

#[test]
fn utc_with_format() {
    let config = parse(&["-u", "+%Y-%m-%d"]);
    assert!(config.utc);
    assert_eq!(config.format, Some("%Y-%m-%d".to_string()));
}

#[test]
fn help_returns_err() {
    assert!(DateConfig::try_parse_from(["date", "--help"]).is_err());
}

#[test]
fn version_returns_err() {
    assert!(DateConfig::try_parse_from(["date", "--version"]).is_err());
}
