use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::Path;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Level {
    Error,
    Verbose,
    Debug,
    Info,
}

impl Level {
    pub fn from_code(code: char) -> Option<Self> {
        match code {
            'E' => Some(Self::Error),
            'V' => Some(Self::Verbose),
            'D' => Some(Self::Debug),
            'I' => Some(Self::Info),
            _ => None,
        }
    }

    pub fn code(self) -> char {
        match self {
            Self::Error => 'E',
            Self::Verbose => 'V',
            Self::Debug => 'D',
            Self::Info => 'I',
        }
    }

    fn catalog_name(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Verbose => "verbose",
            Self::Debug => "debug",
            Self::Info => "info",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedLine {
    pub level: Level,
    pub component: String,
    pub timestamp: String,
    pub body: String,
}

/// Parse severity only from Nimble's explicit one-letter prefix code.
pub fn parse_record(line: &str) -> Option<ParsedLine> {
    let first_close = line.find("] [")?;
    if !line.starts_with('[') {
        return None;
    }
    let timestamp_header = &line[1..first_close];
    let timestamp = timestamp_header
        .split_whitespace()
        .take(2)
        .collect::<Vec<_>>()
        .join(" ");
    let after_header = &line[first_close + 3..];
    let component_end = after_header.find("] ")?;
    let component = &after_header[..component_end];
    let message = &after_header[component_end + 2..];
    let mut chars = message.chars();
    let code = chars.next()?;
    if chars.next()? != ':' {
        return None;
    }
    let level = Level::from_code(code)?;
    let body = message[2..].strip_prefix(' ').unwrap_or(&message[2..]);
    Some(ParsedLine {
        level,
        component: component.to_owned(),
        timestamp,
        body: body.to_owned(),
    })
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Record {
    pub level: Level,
    pub component: String,
    pub body: String,
    pub timestamp: String,
    pub path: String,
    pub offset: u64,
}

impl Record {
    pub fn new(
        level: Level,
        component: &str,
        body: &str,
        timestamp: &str,
        path: &str,
        offset: u64,
    ) -> Self {
        Self {
            level,
            component: component.to_owned(),
            body: body.to_owned(),
            timestamp: timestamp.to_owned(),
            path: path.to_owned(),
            offset,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MessageGroup {
    pub level: Level,
    pub component: String,
    pub body: String,
    pub count: usize,
    pub first: Record,
    pub last: Record,
}

pub fn group_records(records: Vec<Record>) -> Vec<MessageGroup> {
    let mut groups: BTreeMap<(Level, String, String), MessageGroup> = BTreeMap::new();
    for record in records {
        let key = (
            record.level,
            record.component.clone(),
            record.body.clone(),
        );
        groups
            .entry(key)
            .and_modify(|group| {
                group.count += 1;
                group.last = record.clone();
            })
            .or_insert_with(|| MessageGroup {
                level: record.level,
                component: record.component.clone(),
                body: record.body.clone(),
                count: 1,
                first: record.clone(),
                last: record,
            });
    }
    groups.into_values().collect()
}

#[derive(Clone, Debug, Deserialize)]
pub struct CatalogEntry {
    pub id: String,
    pub title: String,
    pub level: String,
    pub component: String,
    pub pattern: String,
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct CatalogIndex {
    pub schema_version: u32,
    pub entry_count: usize,
    pub entries: Vec<CatalogEntry>,
    pub terms: HashMap<String, Vec<usize>>,
}

#[derive(Clone, Debug)]
pub struct CatalogCandidate {
    pub entry: CatalogEntry,
    pub score: usize,
    pub level_agrees: bool,
}

impl CatalogIndex {
    pub fn load(path: &Path) -> Result<Self, String> {
        let raw = fs::read(path).map_err(|error| format!("read {}: {error}", path.display()))?;
        let index: Self = serde_json::from_slice(&raw)
            .map_err(|error| format!("parse {}: {error}", path.display()))?;
        if index.schema_version != 1 || index.entries.len() != index.entry_count {
            return Err("unsupported or inconsistent logger map".to_owned());
        }
        Ok(index)
    }

    /// Return candidate descriptions; the score narrows review and is not a final match.
    pub fn candidates(&self, body: &str, level: Level, limit: usize) -> Vec<CatalogCandidate> {
        if limit == 0 {
            return Vec::new();
        }
        let mut scores: HashMap<usize, usize> = HashMap::with_capacity(64);
        let mut token = String::new();
        for byte in body.bytes().chain(std::iter::once(b' ')) {
            if byte.is_ascii_alphanumeric() {
                token.push(byte.to_ascii_lowercase() as char);
            } else if !token.is_empty() {
                if let Some(indices) = self.terms.get(&token) {
                    for index in indices {
                        *scores.entry(*index).or_default() += 1;
                    }
                }
                token.clear();
            }
        }
        let mut ranked: Vec<_> = scores
            .into_iter()
            .filter_map(|(index, score)| {
                self.entries.get(index).map(|entry| {
                    let agrees = entry.level == level.catalog_name()
                        || (level == Level::Info && entry.level == "info (always)");
                    (index, score, agrees)
                })
            })
            .collect();
        let ordering = |left: &(usize, usize, bool), right: &(usize, usize, bool)| {
            right
                .2
                .cmp(&left.2)
                .then_with(|| right.1.cmp(&left.1))
                .then_with(|| self.entries[left.0].id.cmp(&self.entries[right.0].id))
        };
        if ranked.len() > limit {
            ranked.select_nth_unstable_by(limit, ordering);
            ranked.truncate(limit);
        }
        ranked.sort_unstable_by(ordering);
        ranked
            .into_iter()
            .map(|(index, score, level_agrees)| CatalogCandidate {
                entry: self.entries[index].clone(),
                score,
                level_agrees,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{group_records, parse_record, CatalogIndex, Level, Record};
    use std::path::Path;

    #[test]
    fn parses_the_four_nimble_level_codes_without_body_heuristics() {
        let cases = [
            ("E", Level::Error),
            ("V", Level::Verbose),
            ("D", Level::Debug),
            ("I", Level::Info),
        ];
        for (code, expected) in cases {
            let line = format!(
                "[2026-10-05 10:00:00 P10-T20] [main] {code}: body says error but this is the prefix level"
            );
            assert_eq!(parse_record(&line).unwrap().level, expected);
        }
    }

    #[test]
    fn leaves_unrecognized_level_codes_unclassified() {
        let line = "[2026-10-05 10:00:00 P10-T20] [main] W: warning text";
        assert!(parse_record(line).is_none());
    }

    #[test]
    fn groups_identical_records_and_retains_count_and_locations() {
        let records = vec![
            Record::new(Level::Error, "main", "same body", "10:00:00", "nimble.log", 10),
            Record::new(Level::Error, "main", "same body", "10:00:01", "nimble.log", 90),
        ];
        let groups = group_records(records);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].count, 2);
        assert_eq!(groups[0].first.offset, 10);
        assert_eq!(groups[0].last.offset, 90);
        assert_eq!(groups[0].first.timestamp, "10:00:00");
        assert_eq!(groups[0].last.timestamp, "10:00:01");
    }

    #[test]
    fn logger_map_finds_catalog_candidates_and_prefers_matching_level() {
        let map = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("logger/search-index.json");
        let index = CatalogIndex::load(&map).expect("load generated logger map");
        let candidates = index.candidates("failed to detect country code", Level::Error, 3);
        assert_eq!(candidates[0].entry.id, "log_e-328ef208f64472cf");
        assert!(candidates[0].level_agrees);
    }
}
