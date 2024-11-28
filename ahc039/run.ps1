param (
    [string]$filePath
)

if ($filePath) {
    $outputPath = Join-Path -Path .\out -ChildPath (Split-Path -Leaf $filePath)
    & cat $filePath | cargo run --release > $outputPath
} else {
    Get-ChildItem -Path .\in -Recurse | ForEach-Object {
        $outputPath = Join-Path -Path .\out -ChildPath $_.Name
        & cat $_.FullName | cargo run --release > $outputPath
    }
}
