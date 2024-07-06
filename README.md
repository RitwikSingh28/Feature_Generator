# FeatureGenerator

FeatureGenerator is a command-line tool designed to create a Clean Architecture folder structure for a specific feature in a Flutter project.

## Features

- Automates the creation of Clean Architecture folders for a new feature in a Flutter project.
- Ensures a consistent project structure.
- Supports custom base directory and feature directory name.

## Installation

- Just download the binary from the `releases`

## Usage

Navigate to the root of your Flutter project and run the binary with the appropriate arguments:

```
./feature_generator --feature <feature-name>
```

## Arguments

* `--feature, -f <feature_name>`: (**Required**) The name of the feature for which the folder structure will be created.
* `--base, -b <base_dir>`: (Optional) The base directory of the Flutter project. Default is the current directory.
* `--feature-dir <FEATURE_DIR_NAME>`: (Optional) The name of the base directory to contain all feature directories. Default is 'features'. This allows you to specify a custom directory name instead of using the default 'features' directory.

## Example

```
# Basic usage
./featurefolderizer --feature authentication

# Specifying a different base directory
./featurefolderizer --feature authentication --base /path/to/flutter/project

# Specifying a custom feature directory name
./featurefolderizer --feature authentication --feature-dir custom_features

# Enabling verbose output
./featurefolderizer --feature authentication --verbose

```

## Important

For this CLI to function correctly, the binary must be placed in the root folder of your Flutter project directory. The tool expects to find a lib folder in the current directory, which indicates that it is in the root of a Flutter project.
