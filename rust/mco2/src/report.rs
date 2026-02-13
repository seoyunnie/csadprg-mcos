#![allow(clippy::cast_precision_loss)]

use crate::formatted_serializer::{serialize_f64, serialize_usize};
use crate::project::Project;
use itertools::Itertools;
use serde::Serialize;
use std::{
    collections::{HashMap, HashSet},
    error,
    fs::File,
    io::Write,
};

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "PascalCase"))]
struct RegionEfficiency {
    region: String,
    main_island: String,
    #[serde(serialize_with = "serialize_f64")]
    total_budget: f64,
    #[serde(serialize_with = "serialize_f64")]
    median_savings: f64,
    #[serde(serialize_with = "serialize_f64")]
    avg_delay: f64,
    #[serde(serialize_with = "serialize_f64")]
    high_delay_pct: f64,
    #[serde(serialize_with = "serialize_f64")]
    efficiency_score: f64,
}

pub(crate) fn create_report_1(projects: &[Project]) -> Result<(), Box<dyn error::Error>> {
    let mut region_efficiencies = Vec::<RegionEfficiency>::new();

    for (region, projects) in projects.iter().into_group_map_by(|&p| &p.region) {
        let mut cost_savings = projects.iter().map(|&p| p.cost_savings()).collect::<Vec<f64>>();

        cost_savings.sort_by(f64::total_cmp);

        let median_savings = cost_savings[cost_savings.len() / 2];

        let high_delay_min_days = 30;

        let completion_delay_days = projects
            .iter()
            .map(|&p| p.completion_delay_days())
            .collect::<Vec<i64>>();
        let avg_delay = completion_delay_days.iter().copied().sum::<i64>() as f64 / completion_delay_days.len() as f64;

        region_efficiencies.push(RegionEfficiency {
            region: region.clone(),
            main_island: projects[0].main_island.clone(),
            total_budget: projects.iter().map(|&p| p.approved_budget_for_contract).sum::<f64>(),
            median_savings,
            avg_delay,
            high_delay_pct: (completion_delay_days
                .iter()
                .copied()
                .filter(|&d| d > high_delay_min_days)
                .count() as f64
                / completion_delay_days.len() as f64)
                * 100.0,
            efficiency_score: (median_savings / avg_delay) * 100.0,
        });
    }

    region_efficiencies.sort_by(|a, b| b.efficiency_score.total_cmp(&a.efficiency_score));

    let file_name = "report1_regional_summary.csv";
    let mut fw = csv::Writer::from_path(file_name)?;

    for region_efficiency in region_efficiencies {
        fw.serialize(region_efficiency)?;
    }

    fw.flush()?;

    println!("1. Flood Mitigation Efficiency Summary (exported to {file_name})");

    Ok(())
}

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "PascalCase"))]
struct ContractorPerformance {
    rank: usize,
    contractor: String,
    #[serde(serialize_with = "serialize_f64")]
    total_cost: f64,
    #[serde(serialize_with = "serialize_usize")]
    num_projects: usize,
    #[serde(serialize_with = "serialize_f64")]
    avg_delay: f64,
    #[serde(serialize_with = "serialize_f64")]
    total_savings: f64,
    #[serde(serialize_with = "serialize_f64")]
    reliability_index: f64,
    risk_flag: String,
}

pub(crate) fn create_report_2(projects: &[Project]) -> Result<(), Box<dyn error::Error>> {
    let min_project_cnt = 5;

    let mut contractor_performances = Vec::<ContractorPerformance>::new();

    for (contractor, projects) in projects.iter().into_group_map_by(|&p| &p.contractor) {
        if projects.len() < min_project_cnt {
            continue;
        }

        let total_cost = projects.iter().map(|&p| p.contract_cost).sum::<f64>();

        let completion_delay_days = projects
            .iter()
            .map(|&p| p.completion_delay_days())
            .collect::<Vec<i64>>();
        let avg_delay = completion_delay_days.iter().copied().sum::<i64>() as f64 / completion_delay_days.len() as f64;

        let total_savings = projects.iter().map(|&p| p.cost_savings()).sum::<f64>();

        let low_risk_min_idx = 50.0;

        let reliability_idx = ((1.0 - (avg_delay / 90.0)) * (total_savings / total_cost) * 100.0)
            .clamp(0.0, 100.0)
            .abs();

        contractor_performances.push(ContractorPerformance {
            rank: 0,
            contractor: contractor.clone(),
            total_cost,
            num_projects: projects.len(),
            avg_delay,
            total_savings,
            reliability_index: reliability_idx,
            risk_flag: if reliability_idx < low_risk_min_idx {
                String::from("High Risk")
            } else {
                String::from("Low Risk")
            },
        });
    }

    contractor_performances.sort_by(|a, b| a.total_cost.total_cmp(&b.total_cost));

    let contractor_cnt = 15;

    contractor_performances = contractor_performances.into_iter().take(contractor_cnt).rev().collect();

    for (i, contractor_perf) in contractor_performances.iter_mut().enumerate() {
        contractor_perf.rank = i + 1;
    }

    let file_name = "report2_contractor_ranking.csv";
    let mut fw = csv::Writer::from_path(file_name)?;

    for contractor_perf in contractor_performances {
        fw.serialize(contractor_perf)?;
    }

    fw.flush()?;

    println!("2. Top Contractors Performance Ranking (exported to {file_name})");

    Ok(())
}

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "PascalCase"))]
struct ProjectOverrunTrend {
    funding_year: u32,
    type_of_work: String,
    #[serde(serialize_with = "serialize_usize")]
    total_projects: usize,
    #[serde(serialize_with = "serialize_f64")]
    avg_savings: f64,
    #[serde(serialize_with = "serialize_f64")]
    overrun_rate: f64,
    #[serde(rename(serialize = "YoYChange"))]
    #[serde(serialize_with = "serialize_f64")]
    year_over_year_change: f64,
}

pub(crate) fn create_report_3(projects: &[Project]) -> Result<(), Box<dyn error::Error>> {
    let mut project_overrun_trends = Vec::<ProjectOverrunTrend>::new();

    for (year, projects) in projects.iter().into_group_map_by(|&p| p.funding_year) {
        for (type_of_work, projects) in projects.into_iter().into_group_map_by(|&p| &p.type_of_work) {
            let savings = projects.iter().map(|&p| p.cost_savings()).collect::<Vec<f64>>();

            project_overrun_trends.push(ProjectOverrunTrend {
                funding_year: year,
                type_of_work: type_of_work.clone(),
                total_projects: projects.len(),
                avg_savings: savings.iter().copied().sum::<f64>() / savings.len() as f64,
                overrun_rate: (savings.iter().copied().filter(|&s| s < 0.0).count() as f64 / savings.len() as f64)
                    * 100.0,
                year_over_year_change: 0.0,
            });
        }
    }

    project_overrun_trends.sort_by(|a, b| {
        a.funding_year
            .cmp(&b.funding_year)
            .then_with(|| b.avg_savings.total_cmp(&a.avg_savings))
    });

    let base_year = 2021;
    let trends_avg_savings = project_overrun_trends
        .iter()
        .map(|t| (t.funding_year, t.avg_savings))
        .collect::<HashMap<u32, f64>>();

    for trend in &mut project_overrun_trends {
        if trend.funding_year <= base_year {
            continue;
        }

        if let Some(prev_avg_savings) = trends_avg_savings.iter().find(|s| *s.0 == trend.funding_year - 1) {
            trend.year_over_year_change = ((trend.avg_savings - prev_avg_savings.1) / prev_avg_savings.1) * 100.0;
        }
    }

    let file_name = "report3_annual_trends.csv";
    let mut fw = csv::Writer::from_path(file_name)?;

    for trend in project_overrun_trends {
        fw.serialize(trend)?;
    }

    fw.flush()?;

    println!("3. Annual Project Type Cost Overrun Trends (exported to {file_name})");

    Ok(())
}

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "PascalCase"))]
struct Summary {
    total_projects: usize,
    total_contractors: usize,
    global_avg_delay: f64,
    total_savings: f64,
}

pub(crate) fn create_summary(projects: &[Project]) -> Result<(), Box<dyn error::Error>> {
    let completion_delay_days = projects
        .iter()
        .map(Project::completion_delay_days)
        .collect::<Vec<i64>>();
    let avg_delay = completion_delay_days.iter().copied().sum::<i64>() as f64 / completion_delay_days.len() as f64;

    let summary = Summary {
        total_projects: projects.len(),
        total_contractors: projects
            .iter()
            .map(|p| &p.contractor)
            .collect::<HashSet<&String>>()
            .len(),
        global_avg_delay: avg_delay,
        total_savings: projects.iter().map(Project::cost_savings).sum::<f64>(),
    };

    let mut file = File::create("summary.json")?;
    let stringified_data = serde_json::to_string_pretty(&summary)?;

    file.write_all(stringified_data.as_bytes())?;

    Ok(())
}
