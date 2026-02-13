#!/bin/sh

RESET="\033[0m"
RED="\033[0;31m"
GREEN="\033[0;32m"

compare_reports() {
  label=$1
  js_path=$2
  rust_path=$3

  if diff "$js_path" "$rust_path" > /dev/null; then
    printf "${GREEN}${label} matches! ${RESET}\n"
  else
    printf "${RED}${label} does not match: ${RESET}\n"
    diff "$js_path" "$rust_path"
  fi
}

compare_reports "Report 1 (Regional Summary)" \
  "./js/mco2/report1_regional_summary.csv" \
  "./rust/mco2/report1_regional_summary.csv"

compare_reports "Report 2 (Contractor Ranking)" \
  "./js/mco2/report2_contractor_ranking.csv" \
  "./rust/mco2/report2_contractor_ranking.csv"

compare_reports "Report 3 (Annual Trends)" \
  "./js/mco2/report3_annual_trends.csv" \
  "./rust/mco2/report3_annual_trends.csv"

compare_reports "Summary" \
  "./js/mco2/summary.json" \
  "./rust/mco2/summary.json"
