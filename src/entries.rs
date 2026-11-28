use time::{Date, Duration, macros::format_description};
use anyhow::{ Context, ensure, Result };
use std::str::FromStr;
use std::path::Path;

pub trait HasDate {
    fn date(&self) -> Date;
}

pub struct DateRange {
    from: Date,
    to: Date
}

impl DateRange {
  pub fn new(from: Date, to: Date) -> Result<Self> {
    ensure!(from <= to, "invalid date range: {from} is after {to}");
    Ok(Self { from, to })
  }

  pub fn contains(&self, date: Date) -> bool {
    self.from <= date && date <= self.to
  }
}

pub fn parse_header_date(s: &str) -> Result<Date> {
    Date::parse(s.trim(), format_description!("[year]-[month]-[day]")).with_context(|| format!("parsing date '{}'", s))
}

pub fn entries_in_range<T>(log_file_path: &Path, date_range: DateRange) -> Result<Vec<T>>
where T: FromStr<Err = anyhow::Error> + HasDate
{
    let log_file_content = match std::fs::read_to_string(log_file_path) {
        Ok(content) => content,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(e).with_context(|| format!("reading {}", log_file_path.display()))
    };
    let all: Vec<T> = parse_all_log_entries(&log_file_content)?;
    Ok(all.into_iter().filter(|e| date_range.contains(e.date())).collect())
}

pub fn parse_all_log_entries<T: FromStr<Err = anyhow::Error>>(log_file_content: &str) -> Result<Vec<T>> {
    log_file_content.split("\n---\n").map(str::trim).filter(|c| !c.is_empty()).map(|c| T::from_str(c)).collect()
}

pub fn previous_monday(date: Date) -> Date {
    date - Duration::days(date.weekday().number_days_from_monday() as i64)
}

pub fn extract_details_from_log_file<'a, I: Iterator<Item = &'a str>>(lines: I) -> Option<String> {
    let mut in_details = false;
    let mut detail_lines: Vec<&str> = Vec::new();

    for line in lines {
        let trimmed = line.trim();
        if !in_details {
            if trimmed == "Details:" {
                in_details = true;
            }
            continue;
        }
        if trimmed == "---" {
            break;
        }
        detail_lines.push(line);
    }

    let joined = detail_lines.join("\n").trim().to_string();
    if joined.is_empty() || joined == ("No details") {
        None
    } else {
        Some(joined)
    }
}
