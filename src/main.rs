use std::fs;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::process;
use std::collections::HashMap;

use clap::Parser;
use onig::Regex;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::de::{self, Deserializer, Visitor};
use std::fmt;

/// コマンドライン引数
#[derive(Parser)]
struct Args {
    #[arg(default_value = "/etc/glint/rules.toml")]
    config: PathBuf,

    /// Show Version
    #[clap(short = 'V', long)]
    version: bool,
}

/// 設定ファイル構造
#[derive(Debug, Deserialize)]
struct Rule {
    #[serde(deserialize_with = "string_or_vec")]
    regexp: Vec<String>,
    color: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    rules: Vec<Rule>,
}

/// string or list of strings に対応するデシリアライザ
fn string_or_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrVec;

    impl<'de> Visitor<'de> for StringOrVec {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or list of strings")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(vec![value.to_owned()])
        }

        fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            Deserialize::deserialize(de::value::SeqAccessDeserializer::new(seq))
        }
    }

    deserializer.deserialize_any(StringOrVec)
}

static DEFAULT_COLOR: &str = "\x1b[0m";

static COLORS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        ("black", "\x1b[30m"),
        ("red", "\x1b[31m"),
        ("green", "\x1b[32m"),
        ("yellow", "\x1b[33m"),
        ("blue", "\x1b[34m"),
        ("magenta", "\x1b[35m"),
        ("cyan", "\x1b[36m"),
        ("white", "\x1b[37m"),
        ("bold", "\x1b[1m"),
        ("underline", "\x1b[4m"),
        ("blink", "\x1b[5m"),
        ("reverse", "\x1b[7m"),
        ("concealed", "\x1b[8m"),
        ("default", DEFAULT_COLOR),
    ])
});

fn parse_config(path: &PathBuf) -> io::Result<Vec<(Regex, String)>> {
    let contents = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&contents)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let mut parsed = Vec::new();
    for rule in config.rules {
        // スタイルをスペースで split して連結
        let color_seq = rule
            .color
            .split_whitespace()
            .filter_map(|c| COLORS.get(c))
            .fold(String::new(), |mut acc, &s| {
                acc.push_str(s);
                acc
            });

        for pattern in rule.regexp {
            let re = Regex::new(&pattern)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            parsed.push((re, color_seq.clone()));
        }
    }

    Ok(parsed)
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if args.version {
        println!(env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let config_path = &args.config;
    if !config_path.exists() {
        eprintln!("Config file not found: {}", config_path.display());
        process::exit(1);
    }

    let rules = parse_config(config_path).unwrap_or_else(|e| {
        eprintln!("Config error: {}", e);
        process::exit(1);
    });

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let mut segments: Vec<(usize, usize, &str)> = Vec::new();

        for (re, color) in &rules {
            for (start, end) in re.find_iter(&line) {
                if start < end {
                    segments.push((start, end, color.as_str()));
                }
            }
        }

        segments.sort_by_key(|&(start, _, _)| start);
        let mut output = String::new();
        let mut last_index = 0;

        for (start, end, color) in segments {
            if start >= last_index {
                output.push_str(&line[last_index..start]);
                output.push_str(color);
                output.push_str(&line[start..end]);
                output.push_str(DEFAULT_COLOR);
                last_index = end;
            }
        }

        output.push_str(&line[last_index..]);
        println!("{}", output);
    }

    Ok(())
}
