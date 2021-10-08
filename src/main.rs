#![allow(dead_code)]
use serde::Deserialize;
use std::error::Error;
use std::fs;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() {
    parse_csv().unwrap();
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
struct PanelRow {
    label: String,
    created: String,
}

fn parse_csv() -> Result<()> {
    let mut rdr = csv::Reader::from_path("panels.csv")?;

    for result in rdr.deserialize() {
        let row: PanelRow = result?;

        if row.created == "TRUE" {
            continue;
        }

        let template = r"C:\Users\mwright\Dropbox (Cairns Electric Ltd.)\46263 - Site Team Folder\DRAWINGS SOUTH\12 - Electrical Panels\Schedules\Template\PANEL- PREFIX.xlsx";
        let to = format!(
            r"C:\Users\mwright\Dropbox (Cairns Electric Ltd.)\46263 - Site Team Folder\DRAWINGS SOUTH\12 - Electrical Panels\Schedules\{}.xlsx",
            row.label
        );

        fs::copy(template, to).unwrap();

        println!("created schedule for {:?}", row.label)
    }

    Ok(())
}
