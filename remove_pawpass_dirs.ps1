$AppDataDir = [System.Environment]::GetFolderPath('ApplicationData')

$PawPassDir = Join-Path $AppDataDir "PawPass"
$BackupsDir = Join-Path $PawPassDir "backups"
$TempDir    = Join-Path $PawPassDir "temp"
$VaultFile  = Join-Path $PawPassDir "vault.dat"

Write-Host "AppData Directory: $AppDataDir"
Write-Host "PawPass Directory: $PawPassDir"
Write-Host "Backups Directory: $BackupsDir"
Write-Host "Temp Directory: $TempDir"
Write-Host "Vault File: $VaultFile"
Write-Host ""



if (Test-Path $VaultFile) {
    Write-Host "Removing vault.dat file..."
    Remove-Item $VaultFile -Force
    Write-Host "vault.dat removed successfully."
} else {
    Write-Host "No vault.dat file found."
}
Write-Host ""


if (Test-Path $BackupsDir) {
    Write-Host "Removing backups directory..."
    Remove-Item $BackupsDir -Recurse -Force
    Write-Host "Backups directory removed."
} else {
    Write-Host "No backups directory found."
}
Write-Host ""


if (Test-Path $TempDir) {
    Write-Host "Removing temp directory..."
    Remove-Item $TempDir -Recurse -Force
    Write-Host "Temp directory removed."
} else {
    Write-Host "No temp directory found."
}
Write-Host ""


if (Test-Path $PawPassDir) {
    Write-Host "Attempting to remove PawPass directory..."
    try {
        Remove-Item $PawPassDir -Recurse -Force
        Write-Host "PawPass directory removed."
    } catch {
        Write-Host "Unable to remove PawPass directory. It may not be empty or may be in use."
    }
} else {
    Write-Host "No PawPass directory found."
}

Write-Host "Cleanup complete."
