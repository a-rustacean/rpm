# RPM (Rust Project Manager)

RPM is an open-source tool designed for efficient management of Rust projects in a structured manner.

## Installation

To install RPM, ensure Rust is installed on your system, then run:

```shell
git clone https://github.com/a-rustacean/rpm.git
cd rpm
./install.sh
```

## Initial Setup

### Set Working Directory (Optional):

```shell
# Default: $HOME/Devs
rpm set workdir <YOUR WORKING DIRECTORY>
```

### Set Templates Directory (Optional):

```shell
# Default: $HOME/Templates
rpm set templates-dir <YOUR TEMPLATES DIRECTORY>
```
> Note: Ensure every template folder name ends with `-template`.

### Analyze:

```shell
rpm analyze
```

> This command generates the `projects.json` file in your working directory, aiding RPM in project management.
> 
> Note: Run this command whenever you delete or manually create a project.

## Usage

### Create a Project:

```shell
# Creates a bin project
rpm new <NAME>

# Creates a lib project
rpm new <NAME> --template lib

# Creates a project with the specified template
rpm new <NAME> --template <TEMPLATE>
```

### Mark a Project:

```shell
# Mark as completed
rpm mark <NAME> completed

# Mark as incomplete
rpm mark <NAME> incomplete
```

### List Projects:

```shell
# List all projects
rpm list

# List completed projects
rpm list completed

# List incomplete projects
rpm list incomplete
```

## Uninstall

```shell
# Ensure you are in the cloned directory from GitHub
./uninstall.sh
```

Feel free to enhance your Rust development workflow with RPM!
