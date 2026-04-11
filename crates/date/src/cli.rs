const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = "\
Usage: date [OPTION]... [+FORMAT]

Display the current time in the given FORMAT, or set the system date.

  -d, --date=STRING        display time described by STRING, not 'now'
  -I, --iso-8601[=TIMESPEC] output date/time in ISO 8601 format.
                             TIMESPEC='date' for date only (the default),
                             'hours', 'minutes', 'seconds', or 'ns'
  -R, --rfc-email          output date and time in RFC 5322 format
      --rfc-3339=TIMESPEC  output date/time in RFC 3339 format.
                             TIMESPEC='date', 'seconds', or 'ns'
  -r, --reference=FILE     display the last modification time of FILE
  -u, --utc, --universal   print or set Coordinated Universal Time (UTC)
      --help     display this help and exit
      --version  output version information and exit

FORMAT controls the output. Interpreted sequences are:
  %Y  year (e.g., 2025)
  %m  month (01..12)
  %d  day of month (01..31)
  %H  hour (00..23)
  %M  minute (00..59)
  %S  second (00..60)
  %A  locale's full weekday name (e.g., Sunday)
  %a  locale's abbreviated weekday name (e.g., Sun)
  %B  locale's full month name (e.g., January)
  %b  locale's abbreviated month name (e.g., Jan)
  %Z  alphabetic time zone abbreviation (e.g., EDT)
  %z  +hhmm numeric time zone (e.g., -0400)
  %s  seconds since 1970-01-01 00:00:00 UTC
  %n  a newline
  %t  a tab
  %%  a literal %";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DateConfig {
    pub format: Option<String>,
    pub date_string: Option<String>,
    pub utc: bool,
    pub iso_format: Option<String>,
    pub rfc_email: bool,
    pub rfc_3339: Option<String>,
    pub reference: Option<String>,
}

impl DateConfig {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let mut config = DateConfig::default();
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" {
                println!("{HELP_TEXT}");
                return None;
            }
            if arg == "--version" {
                println!("date {VERSION}");
                return None;
            }
            if arg == "--" {
                i += 1;
                break;
            }

            // Long options with = form
            if let Some(rest) = arg.strip_prefix("--date=") {
                config.date_string = Some(rest.to_string());
                i += 1;
                continue;
            }
            if let Some(rest) = arg.strip_prefix("--iso-8601=") {
                config.iso_format = Some(rest.to_string());
                i += 1;
                continue;
            }
            if arg == "--iso-8601" || arg == "-I" {
                config.iso_format = Some("date".to_string());
                i += 1;
                continue;
            }
            if let Some(rest) = arg.strip_prefix("--rfc-3339=") {
                config.rfc_3339 = Some(rest.to_string());
                i += 1;
                continue;
            }
            if let Some(rest) = arg.strip_prefix("--reference=") {
                config.reference = Some(rest.to_string());
                i += 1;
                continue;
            }

            match arg.as_str() {
                "--date" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("date: option '--date' requires an argument");
                        return None;
                    }
                    config.date_string = Some(args[i].clone());
                }
                "--rfc-3339" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("date: option '--rfc-3339' requires an argument");
                        return None;
                    }
                    config.rfc_3339 = Some(args[i].clone());
                }
                "--rfc-email" => config.rfc_email = true,
                "--utc" | "--universal" => config.utc = true,
                "--reference" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!("date: option '--reference' requires an argument");
                        return None;
                    }
                    config.reference = Some(args[i].clone());
                }
                _ if arg.starts_with('+') => {
                    config.format = Some(arg.strip_prefix('+').unwrap().to_string());
                }
                _ if arg.starts_with('-') && !arg.starts_with("--") => {
                    let chars: Vec<char> = arg[1..].chars().collect();
                    let mut j = 0;
                    while j < chars.len() {
                        match chars[j] {
                            'u' => config.utc = true,
                            'R' => config.rfc_email = true,
                            'I' => {
                                // Check if there's a value attached
                                if j + 1 < chars.len() {
                                    let spec: String = chars[j + 1..].iter().collect();
                                    config.iso_format = Some(spec);
                                    j = chars.len();
                                    continue;
                                } else {
                                    config.iso_format = Some("date".to_string());
                                }
                            }
                            'd' => {
                                let val = if j + 1 < chars.len() {
                                    chars[j + 1..].iter().collect::<String>()
                                } else {
                                    i += 1;
                                    if i >= args.len() {
                                        eprintln!("date: option requires an argument -- 'd'");
                                        return None;
                                    }
                                    args[i].clone()
                                };
                                config.date_string = Some(val);
                                j = chars.len();
                                continue;
                            }
                            'r' => {
                                let val = if j + 1 < chars.len() {
                                    chars[j + 1..].iter().collect::<String>()
                                } else {
                                    i += 1;
                                    if i >= args.len() {
                                        eprintln!("date: option requires an argument -- 'r'");
                                        return None;
                                    }
                                    args[i].clone()
                                };
                                config.reference = Some(val);
                                j = chars.len();
                                continue;
                            }
                            _ => {
                                eprintln!("date: invalid option -- '{}'", chars[j]);
                                return None;
                            }
                        }
                        j += 1;
                    }
                }
                _ if arg.starts_with("--") => {
                    eprintln!("date: unrecognized option '{arg}'");
                    return None;
                }
                _ => {
                    eprintln!("date: unexpected argument '{arg}'");
                    return None;
                }
            }

            i += 1;
        }

        // Handle remaining args after --
        while i < args.len() {
            let arg = &args[i];
            if let Some(fmt) = arg.strip_prefix('+') {
                config.format = Some(fmt.to_string());
            } else {
                eprintln!("date: unexpected argument '{arg}'");
                return None;
            }
            i += 1;
        }

        Some(config)
    }
}
