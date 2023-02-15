import re

#nice script to update version in Cargo.toml file for this project


# Read in the current version from Cargo.toml
with open('Cargo.toml', 'r') as f:
    cargo_toml = f.read()

current_version = re.search('version = "(.*?)"', cargo_toml).group(1)

# Parse the version using semver
major, minor, patch = current_version.split('.')
minor = int(minor) + 1

# Construct the new version string
new_version = f"{major}.{minor}.{patch}"

# Update the version in Cargo.toml
updated_cargo_toml = re.sub(f'version = "{current_version}"', f'version = "{new_version}"', cargo_toml)

with open('Cargo.toml', 'w') as f:
    f.write(updated_cargo_toml)

print(f"Version updated to {new_version}")