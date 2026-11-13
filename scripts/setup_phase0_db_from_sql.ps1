# Windows PowerShell script to bootstrap Phase 0 DB from phase0_schema.sql
$dbDir = "e:\\Projects\\Kalthuko\\data"
$dbPath = "$dbDir\\kalthuko_phase0.db"
$sqlPath = "$dbDir\\phase0_schema.sql"

if (-not (Test-Path $dbDir)) { New-Item -ItemType Directory -Path $dbDir -Force | Out-Null }

if (-not (Test-Path $dbPath)) { New-Item -ItemType File -Path $dbPath -Force | Out-Null }

if (-not (Test-Path $sqlPath)) { Write-Error "Missing SQL schema file at $sqlPath"; exit 1 }

if (-not (Get-Command sqlite3 -ErrorAction SilentlyContinue)) { Write-Error "sqlite3 not found. Install SQLite and ensure sqlite3 is in PATH"; exit 1 }

& sqlite3 "$dbPath" ".read '$sqlPath'"

Write-Output "Phase 0 schema loaded from $sqlPath into $dbPath"