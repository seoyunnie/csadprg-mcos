import { createReadStream } from "node:fs";
import { parse } from "csv";
import process from "node:process";
import { stringToNumber } from "./conversion.ts";

export class Project {
  readonly mainIsland: string;
  readonly region: string;
  readonly province: string;
  readonly legislativeDistrict: string;
  readonly municipality: string;
  readonly districtEngineeringOffice: string;
  readonly projectId: string;
  readonly projectName: string;
  readonly typeOfWork: string;
  readonly fundingYear: number;
  readonly contractId: string;
  readonly approvedBudgetForContract: number;
  readonly contractCost: number;
  readonly actualCompletionDate: Date;
  readonly contractor: string;
  readonly startDate: Date;
  readonly projectLatitude: number;
  readonly projectLongitude: number;
  readonly provincialCapital: string;
  readonly provincialCapitalLatitude: number;
  readonly provincialCapitalLongitude: number;

  readonly costSavings: number;
  readonly completionDelayDays: number;

  constructor({
    MainIsland,
    Region,
    Province,
    LegislativeDistrict,
    Municipality,
    DistrictEngineeringOffice,
    ProjectId,
    ProjectName,
    TypeOfWork,
    FundingYear,
    ContractId,
    ApprovedBudgetForContract,
    ContractCost,
    ActualCompletionDate,
    Contractor,
    StartDate,
    ProjectLatitude,
    ProjectLongitude,
    ProvincialCapital,
    ProvincialCapitalLatitude,
    ProvincialCapitalLongitude,
  }: Record<string, string>) {
    this.mainIsland = MainIsland;
    this.region = Region;
    this.province = Province;
    this.legislativeDistrict = LegislativeDistrict;
    this.municipality = Municipality;
    this.districtEngineeringOffice = DistrictEngineeringOffice;
    this.projectId = ProjectId;
    this.projectName = ProjectName;
    this.typeOfWork = TypeOfWork;
    this.fundingYear = stringToNumber(FundingYear);
    this.contractId = ContractId;
    this.approvedBudgetForContract = stringToNumber(ApprovedBudgetForContract);
    this.contractCost = stringToNumber(ContractCost);
    this.actualCompletionDate = new Date(ActualCompletionDate);
    this.contractor = Contractor;
    this.startDate = new Date(StartDate);
    this.projectLatitude = stringToNumber(ProjectLatitude);
    this.projectLongitude = stringToNumber(ProjectLongitude);
    this.provincialCapital = ProvincialCapital;
    this.provincialCapitalLatitude = stringToNumber(ProvincialCapitalLatitude);
    this.provincialCapitalLongitude = stringToNumber(ProvincialCapitalLongitude);

    this.costSavings = this.approvedBudgetForContract - this.contractCost;
    this.completionDelayDays = Math.round(
      (this.actualCompletionDate.getTime() - this.startDate.getTime()) / (1000 * 60 * 60 * 24),
    );
  }
}

export async function parseProjectCSVRecords(): Promise<Project[]> {
  const parser = createReadStream(`${process.cwd()}/dpwh_flood_control_projects.csv`).pipe(parse({ columns: true }));

  process.stdout.write("Processing dataset...");

  const projects: Project[] = [];

  for await (const rec of parser) {
    try {
      // oxlint-disable-next-line no-unsafe-type-assertion
      projects.push(new Project(rec as Record<string, string>));
    } catch {
      continue;
    }
  }

  const minYear = 2021;
  const maxYear = 2023;

  const filteredProjects = projects.filter((p) => p.fundingYear >= minYear && p.fundingYear <= maxYear);

  console.log(
    `  (${projects.length.toLocaleString()} rows loaded, ${filteredProjects.length.toLocaleString()} filtered for ${minYear}-${maxYear})`,
  );

  return filteredProjects;
}
