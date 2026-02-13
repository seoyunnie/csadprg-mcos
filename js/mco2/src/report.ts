import { writeFile } from "node:fs/promises";
import { stringify } from "csv/sync";
import { sum } from "./array-utils.ts";
import type { Project } from "./project.ts";

function csvStringify(data: any[]): string {
  return stringify(data, {
    cast: { number: (val) => val.toLocaleString("en-US", { maximumFractionDigits: 2 }) },
    header: true,
  });
}

interface RegionEfficiency {
  Region: string;
  MainIsland: string;
  TotalBudget: number;
  MedianSavings: number;
  AvgDelay: number;
  HighDelayPct: number;
  EfficiencyScore: number;
}

export async function createReport1(projects: Project[]): Promise<void> {
  const regionEfficiencies: RegionEfficiency[] = [];

  for (const [region, filteredProjects] of Object.entries(Object.groupBy(projects, ({ region }) => region))) {
    if (!filteredProjects) {
      continue;
    }

    const costSavings = filteredProjects.map((p) => p.costSavings);

    costSavings.sort((a, b) => a - b);

    const medianSavings = costSavings[Math.floor(costSavings.length / 2)];

    const highDelayMinDays = 30;

    const completionDelayDays = filteredProjects.map((p) => p.completionDelayDays);
    const avgDelay = sum(completionDelayDays) / completionDelayDays.length;

    regionEfficiencies.push({
      Region: region,
      MainIsland: filteredProjects[0].mainIsland,
      TotalBudget: sum(filteredProjects.map((p) => p.approvedBudgetForContract)),
      MedianSavings: medianSavings,
      AvgDelay: avgDelay,
      HighDelayPct: (completionDelayDays.filter((d) => d > highDelayMinDays).length / completionDelayDays.length) * 100,
      EfficiencyScore: (medianSavings / avgDelay) * 100,
    });
  }

  regionEfficiencies.sort((a, b) => b.EfficiencyScore - a.EfficiencyScore);

  const fileName = "report1_regional_summary.csv";

  await writeFile(fileName, csvStringify(regionEfficiencies));

  console.log(`1. Flood Mitigation Efficiency Summary (exported to ${fileName})`);
}

interface ContractorPerformance {
  Rank: number;
  Contractor: string;
  TotalCost: number;
  NumProjects: number;
  AvgDelay: number;
  TotalSavings: number;
  ReliabilityIndex: number;
  RiskFlag: string;
}

export async function createReport2(projects: Project[]): Promise<void> {
  const minProjectCnt = 5;

  let contractorPerformances: ContractorPerformance[] = [];

  for (const [contractor, filteredProjects] of Object.entries(
    Object.groupBy(projects, ({ contractor }) => contractor),
  )) {
    if (!filteredProjects || filteredProjects.length < minProjectCnt) {
      continue;
    }

    const totalCost = sum(filteredProjects.map((p) => p.contractCost));

    const completionDelayDays = filteredProjects.map((p) => p.completionDelayDays);
    const avgDelay = sum(completionDelayDays) / completionDelayDays.length;

    const totalSavings = sum(filteredProjects.map((p) => p.costSavings));

    const loweRiskMinIdx = 50;

    const reliabilityIdx = Math.abs(Math.min(Math.max((1 - avgDelay / 90) * (totalSavings / totalCost) * 100, 0), 100));

    contractorPerformances.push({
      Rank: 0,
      Contractor: contractor,
      TotalCost: totalCost,
      NumProjects: filteredProjects.length,
      AvgDelay: avgDelay,
      TotalSavings: totalSavings,
      ReliabilityIndex: reliabilityIdx,
      RiskFlag: reliabilityIdx < loweRiskMinIdx ? "High Risk" : "Low Risk",
    });
  }

  contractorPerformances.sort((a, b) => a.TotalCost - b.TotalCost);

  const contractorCnt = 15;

  contractorPerformances = contractorPerformances.slice(0, contractorCnt);

  contractorPerformances.reverse();

  for (const [i, contractorPerf] of contractorPerformances.entries()) {
    contractorPerf.Rank = i + 1;
  }

  const fileName = "report2_contractor_ranking.csv";

  await writeFile(fileName, csvStringify(contractorPerformances));

  console.log(`2. Top Contractors Performance Ranking (exported to ${fileName})`);
}

interface ProjectOverrunTrend {
  FundingYear: string;
  TypeOfWork: string;
  TotalProjects: number;
  AvgSavings: number;
  OverrunRate: number;
  YoYChange: number;
}

export async function createReport3(projects: Project[]): Promise<void> {
  const projectOverrunTrends: ProjectOverrunTrend[] = [];

  for (const [year, unfilteredProjects] of Object.entries(Object.groupBy(projects, ({ fundingYear }) => fundingYear))) {
    if (!unfilteredProjects) {
      continue;
    }

    for (const [typeOfWork, filteredProjects] of Object.entries(
      Object.groupBy(unfilteredProjects, ({ typeOfWork }) => typeOfWork),
    )) {
      if (!filteredProjects) {
        continue;
      }

      const savings = filteredProjects.map((p) => p.costSavings);

      projectOverrunTrends.push({
        FundingYear: year,
        TypeOfWork: typeOfWork,
        TotalProjects: filteredProjects.length,
        AvgSavings: sum(savings) / savings.length,
        OverrunRate: (savings.filter((s) => s < 0).length / savings.length) * 100,
        YoYChange: 0,
      });
    }
  }

  projectOverrunTrends.sort((a, b) => {
    const yearDiff = Number(a.FundingYear) - Number(b.FundingYear);

    if (yearDiff !== 0) {
      return yearDiff;
    }

    return b.AvgSavings - a.AvgSavings;
  });

  const baseYear = 2021;
  const trendsAvgSavings = new Map(projectOverrunTrends.values().map((t) => [Number(t.FundingYear), t.AvgSavings]));

  for (const trend of projectOverrunTrends) {
    if (Number(trend.FundingYear) <= baseYear) {
      continue;
    }

    const prevAvgSavings = trendsAvgSavings.entries().find((s) => s[0] === Number(trend.FundingYear) - 1);

    if (prevAvgSavings) {
      trend.YoYChange = ((trend.AvgSavings - prevAvgSavings[1]) / prevAvgSavings[1]) * 100;
    }
  }

  const fileName = "report3_annual_trends.csv";

  await writeFile(fileName, csvStringify(projectOverrunTrends));

  console.log(`3. Annual Project Type Cost Overrun Trends (exported to ${fileName})`);
}

interface Summary {
  TotalProjects: number;
  TotalContractors: number;
  GlobalAvgDelay: number;
  TotalSavings: number;
}

export async function createSummary(projects: Project[]): Promise<void> {
  const completionDelayDays = projects.map((p) => p.completionDelayDays);
  const avgDelay = sum(completionDelayDays) / completionDelayDays.length;

  const summary = {
    TotalProjects: projects.length,
    TotalContractors: new Set(projects.map((p) => p.contractor)).size,
    GlobalAvgDelay: avgDelay,
    TotalSavings: sum(projects.map((p) => p.costSavings)),
  } satisfies Summary;
  const jsonSummary = JSON.stringify(summary, null, 2);

  await writeFile("summary.json", jsonSummary, "utf8");
}
