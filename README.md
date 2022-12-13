# WEB-BOOKMARKs-CLI
Open website by name that you picked using cli.

## Installation
### Remote
```bash
cargo install --git https://github.com/momozahara/web-bookmarks-cli.git
```
### Local
```bash
cargo install --path .
```

## Command
### Open
Open bookmark in webbrowser.
```bash
open <NAME>
```
### Pwd
Print out location of json file.
```bash
open pwd
```
### Ls
List of bookmarks that you have been add.
```bash
open ls
```
### Add
Add bookmark to the lists.
``` bash
open add \
  <NAME> # github google
  <TARGET> # http(s)://github.com http(s)://www.google.com
```
### Remove
Remove bookmark from the lists.
``` bash
open remove \
  <NAME>
```