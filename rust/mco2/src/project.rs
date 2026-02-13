use chrono::NaiveDate;
use serde::Deserialize;
use std::{error, sync::OnceLock};
use thousands::Separable;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub(crate) struct Project {
    pub(crate) main_island: String,
    pub(crate) region: String,
    pub(crate) province: String,
    pub(crate) legislative_district: String,
    pub(crate) municipality: String,
    pub(crate) district_engineering_office: String,
    #[allow(clippy::struct_field_names)]
    pub(crate) project_id: String,
    #[allow(clippy::struct_field_names)]
    pub(crate) project_name: String,
    pub(crate) type_of_work: String,
    pub(crate) funding_year: u32,
    pub(crate) contract_id: String,
    pub(crate) approved_budget_for_contract: f64,
    pub(crate) contract_cost: f64,
    pub(crate) actual_completion_date: NaiveDate,
    pub(crate) contractor: String,
    pub(crate) start_date: NaiveDate,
    #[allow(clippy::struct_field_names)]
    pub(crate) project_latitude: f64,
    #[allow(clippy::struct_field_names)]
    pub(crate) project_longitude: f64,
    pub(crate) provincial_capital: String,
    pub(crate) provincial_capital_latitude: f64,
    pub(crate) provincial_capital_longitude: f64,

    #[serde(skip_deserializing)]
    cached_cost_savings: OnceLock<f64>,
    #[serde(skip_deserializing)]
    cached_completion_delay_days: OnceLock<i64>,
}
impl Project {
    pub(crate) fn cost_savings(&self) -> f64 {
        *self
            .cached_cost_savings
            .get_or_init(|| self.approved_budget_for_contract - self.contract_cost)
    }

    pub(crate) fn completion_delay_days(&self) -> i64 {
        *self
            .cached_completion_delay_days
            .get_or_init(|| (self.actual_completion_date - self.start_date).num_days())
    }
}

pub(crate) fn parse_csv_records() -> Result<Vec<Project>, Box<dyn error::Error>> {
    let mut fr = csv::Reader::from_path("dpwh_flood_control_projects.csv")?;

    print!("Processing dataset...");

    let projects = fr.deserialize::<Project>().flatten().collect::<Vec<Project>>();
    let project_cnt = projects.len();

    let min_year = 2021;
    let max_year = 2023;

    let filtered_projects = projects
        .into_iter()
        .filter(|p| p.funding_year >= min_year && p.funding_year <= max_year)
        .collect::<Vec<Project>>();

    println!(
        "  ({} rows loaded, {} filtered for {}-{})",
        project_cnt.separate_with_commas(),
        filtered_projects.len().separate_with_commas(),
        min_year,
        max_year,
    );

    Ok(filtered_projects)
}
