function Compare-Report {
    param (
        [string]$Label,
        [string]$JSPath,
        [string]$RustPath
    )

    if (Compare-Object (Get-Content $JSPath) (Get-Content $RustPath) -SyncWindow 0) {
        Write-Host "$Label does not match:" -ForegroundColor Red
        Compare-Object (Get-Content $JSPath) (Get-Content $RustPath) -SyncWindow 0 | Format-Table
    }
    else {
        Write-Host "$Label matches!" -ForegroundColor Green
    }
}

Compare-Report "Report 1 (Regional Summary)" `
    -JSPath "./js/mco2/report1_regional_summary.csv" `
    -RustPath "./rust/mco2/report1_regional_summary.csv"

Compare-Report "Report 2 (Contractor Ranking)" `
    -JSPath "./js/mco2/report2_contractor_ranking.csv" `
    -RustPath "./rust/mco2/report2_contractor_ranking.csv"

Compare-Report "Report 3 (Annual Trends)" `
    -JSPath "./js/mco2/report3_annual_trends.csv" `
    -RustPath "./rust/mco2/report3_annual_trends.csv"

Compare-Report "Summary" `
    -JSPath "./js/mco2/summary.json" `
    -RustPath "./rust/mco2/summary.json"
