# hostess

Command-line utility for managing your /etc/hosts file, built with rust

## Install

`cargo install hostess`

## Usage

| Command                     | Description    |
| :-------------------------- | :------------- |
| `hostess list`              | List all hosts |
| `hostess add [domain] [ip]` | Add a host     |
| `hostess rm [domain]`       | Add a host     |
| `hostess on [domain]`       | Enable a host  |
| `hostess off [domain]`      | Disable a host |

## Todo

- [ ] regex
