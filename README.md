# hostess
command-line utility for managing your /etc/hosts file built with rust

**! do not use it in production, just for local dev**

### Install (rust tool chain required)
`cargo install hostess` 

### Usage(temp)
name | age
---- | ---
`hostess list`| list all hosts 
`hostess add [domain] [ip]`|add a host
`hostess on [domain]`|enable a host
`hostess on [domain]`|disable a host
