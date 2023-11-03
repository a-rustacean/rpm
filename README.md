# rpm

rpm (Rust project manager) is a open source tool for managing your rust project in an organized way

## Installation

```console
# make sure you have rust installed on your system
git clone https://github.com/a-rustacean/rpm.git
cd rpm
./install.sh
```

## Initial setup

Set working directory (optional):

```console
# default: $HOME/Devs
rpm set workdir <YOUR WORKING DIRECTORY>
```

Set templates directory (optional):

```console
# default: $HOME/Templates
rpm set templates-dir <YOUR TEMPLATES DIRECTORY>
```
> Note: every template folder name must end with `-template`

Analyze:

```console
rpm analyze
```

> This command generates `projects.json` file in your working directory. This helps rpm to know about your prijects.
> Note: you have to run this command every time you delete or manually create a project.

## Usage

create a project:

```console
# creates a bin project
rpm new <NAME>

# creates a lib project
rpm new <NAME> --template lib

# creates a project with the given template
rpm new <NAME> --template <TEMPLATE>
```

mark a project:

```console
# mark as completed
rpm mark <NAME> completed

# mark as incomplete
rpm mark <NAME> incomplete
```

list projects:

```console
# list all project
rpm list

# list completed projects
rpm list completed

# list incomplete projects
rpm list incomplete
```

## Uninstall

```console
# make sure you are in the directory you cloned from github
./uninstall.sh
```
