use anyhow::{ensure, Context, Result};
use itertools::{izip, Either};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::{self, BufReader};
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
enum Action {
    Extract {
        #[structopt(parse(from_os_str))]
        original: PathBuf,
    },
    Translate {
        #[structopt(parse(from_os_str))]
        original: PathBuf,
        #[structopt(parse(from_os_str))]
        translation: PathBuf,
    },
}

fn main() -> Result<()> {
    match Action::from_args() {
        Action::Extract { original } => do_extract(&original),
        Action::Translate {
            original,
            translation,
        } => do_translate(&original, &translation),
    }
}

fn do_extract(original: &Path) -> Result<()> {
    let columns = parse_csv(original).context("parse original file")?;
    let columns = densify_csv_columns(&columns);
    let stdout = io::stdout();
    let stdout = stdout.lock();
    serde_yaml::to_writer(stdout, &columns).context("write extract result")?;
    Ok(())
}

struct CsvColumn {
    header: String,
    values: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct DenseCsvColumn {
    header: String,
    values: Vec<String>,
}

fn parse_csv(path: &Path) -> Result<Vec<CsvColumn>> {
    let mut reader = csv::Reader::from_path(path).context("open csv file")?;
    let mut columns = reader
        .headers()
        .context("read header")?
        .iter()
        .map(|header| CsvColumn {
            header: header.into(),
            values: Vec::new(),
        })
        .collect::<Vec<_>>();
    for (i, record) in reader.records().enumerate() {
        let record = record.with_context(|| format!("read record {}", i))?;
        columns
            .iter_mut()
            .zip(record.iter())
            .for_each(|(column, field)| column.values.push(field.into()));
    }
    Ok(columns)
}

fn densify_csv_columns(columns: &[CsvColumn]) -> Vec<DenseCsvColumn> {
    columns
        .into_iter()
        .map(|column| {
            let header = column.header.clone();
            let values = column.values.iter().filter(|s| !s.is_empty());
            let values = BTreeSet::from_iter(values)
                .into_iter()
                .map(|value| value.into())
                .collect();
            DenseCsvColumn { header, values }
        })
        .collect()
}

#[derive(Deserialize)]
struct TranslatedDenseCsvColumn {
    header: String,
    values: Option<Vec<String>>,
    #[serde(default)]
    delete: bool,
}

fn do_translate(original: &Path, translation: &Path) -> Result<()> {
    let columns = parse_csv(original).context("parse original file")?;
    let dense_columns = densify_csv_columns(&columns);

    let translation = File::open(translation).context("open translation file")?;
    let translation = BufReader::new(translation);
    let translated_columns: Vec<TranslatedDenseCsvColumn> =
        serde_yaml::from_reader(translation).context("parse translation file")?;

    ensure!(
        dense_columns.len() == translated_columns.len(),
        "column count doesn't match"
    );
    let columns_mapping = dense_columns
        .iter()
        .zip(translated_columns.iter())
        .map(|(column, trans)| {
            if let Some(values) = &trans.values {
                ensure!(
                    values.len() == column.values.len(),
                    "values count doesn't match for column {}",
                    column.header,
                );
                ensure!(!trans.delete, "deleted column must not have values");
                let mapping = column
                    .values
                    .iter()
                    .map(String::as_str)
                    .zip(values.iter().map(String::as_str))
                    .collect::<HashMap<_, _>>();
                Ok(mapping)
            } else {
                Ok(HashMap::new())
            }
        })
        .collect::<Result<Vec<_>>>()
        .context("map translation to original content")?;

    let stdout = io::stdout();
    let stdout = stdout.lock();
    let mut writer = csv::Writer::from_writer(stdout);
    writer
        .write_record(translated_columns.iter().filter_map(|column| {
            if !column.delete {
                Some(column.header.as_str())
            } else {
                None
            }
        }))
        .context("write header")?;

    let mut column_iters = izip!(
        columns.iter(),
        translated_columns.iter(),
        columns_mapping.iter(),
    )
    .filter_map(|(column, translated, mapping)| {
        if translated.delete {
            None
        } else if translated.values.is_none() {
            Some(Either::Left(column.values.iter().map(String::as_str)))
        } else {
            Some(Either::Right(column.values.iter().map(move |value| {
                if value.is_empty() {
                    ""
                } else {
                    mapping.get(value.as_str()).unwrap()
                }
            })))
        }
    })
    .collect::<Vec<_>>();
    for _ in 0..columns[0].values.len() {
        writer.write_record(column_iters.iter_mut().map(|column| column.next().unwrap()))?;
    }

    Ok(())
}
