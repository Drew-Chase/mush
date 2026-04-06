set windows-shell := ["pwsh.exe", "-NoLogo", "-NoProfile", "-Command"]
set shell := ["bash", "-c"]

dist_dir := "./target/dist"

# Build all workspace crates in release mode
build:
    cargo build --workspace --release

# Remove the dist directory
[windows]
clean:
    if (Test-Path "{{dist_dir}}") { Remove-Item -Recurse -Force "{{dist_dir}}" }

[unix]
clean:
    rm -rf "{{dist_dir}}"

# Build and package all workspace executables into dist_dir
[windows]
package: clean
    New-Item -ItemType Directory -Path "{{dist_dir}}" -Force | Out-Null
    cargo build --workspace --release --message-format=json | ForEach-Object { $o = $_ | ConvertFrom-Json; if ($o.reason -eq 'compiler-artifact' -and $o.executable) { Copy-Item $o.executable "{{dist_dir}}" } }

[unix]
package: clean
    mkdir -p "{{dist_dir}}"
    cargo build --workspace --release --message-format=json | jq -r 'select(.reason == "compiler-artifact" and .executable != null) | .executable' | xargs -I {} cp {} "{{dist_dir}}/"