import process from "node:process";
import { parseProjectCSVRecords } from "./project.ts";
import { createReport1, createReport2, createReport3, createSummary } from "./report.ts";

const projects = await parseProjectCSVRecords();

if (projects.length > 0) {
  console.log();

  console.log("Generating reports...");

  await createReport1(projects);
  await createReport2(projects);
  await createReport3(projects);

  console.log();

  process.stdout.write("Generating summary...");

  await createSummary(projects);

  console.log("  (exported to summary.json)");
}
