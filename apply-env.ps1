function Set-TempEnvFromDotEnv {
    param (
        [string]$envFilePath
    )

    if (-Not (Test-Path $envFilePath)) {
        Write-Error "The .env file at path '$envFilePath' does not exist."
        return
    }

    $envContent = Get-Content $envFilePath

    foreach ($line in $envContent) {
        $trimmedLine = $line.Trim()

        if (-Not [string]::IsNullOrWhiteSpace($trimmedLine) -and -Not $trimmedLine.StartsWith("#")) {
            $keyValue = $trimmedLine -split "=", 2
            if ($keyValue.Length -eq 2) {
                $key = $keyValue[0].Trim()
                $value = $keyValue[1].Trim()
                [System.Environment]::SetEnvironmentVariable($key, $value, [System.EnvironmentVariableTarget]::Process)
                Write-Host "Set temporary environment variable: $key=$value"
            }
        }
    }

    Write-Host "All environment variables from '$envFilePath' have been set temporarily."
}

Set-TempEnvFromDotEnv -envFilePath ".env"
