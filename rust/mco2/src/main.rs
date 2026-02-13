#![warn(clippy::pedantic)]

mod formatted_serializer;
mod project;
mod report;

use report::{create_report_1, create_report_2, create_report_3, create_summary};
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    let projects = project::parse_csv_records()?;

    if projects.is_empty() {
        return Ok(());
    }

    println!();

    println!("Generating reports...");

    create_report_1(&projects)?;
    create_report_2(&projects)?;
    create_report_3(&projects)?;

    println!();

    print!("Generating summary...");

    create_summary(&projects)?;

    println!("  (exported to summary.json)");

    Ok(())
}
