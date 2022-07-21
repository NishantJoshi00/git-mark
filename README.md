# GIT Mark
An utility for configuring your git repository for development or any other situation that requires you to make temporary changes to you repositoy.

## Installation
- Run the following commands:
    - `make build`
    - `make install`

## Uninstallation
- Run the following commands:
    - `make uninstall`

## Usage
- Run the following commands:
    - `git mark help`
- To list all the marks:
    - `git mark list`
- To create a new mark (i.e. Snapshot of all the uncommited changes):
    - `git mark this <name>`
- To apply the snapshot to the current branch:
    - `git mark as <name>`
- To update the snapshot with current uncommited changes:
    - `git mark update <name>`
- To revert the snapshot: **⚠ This is not yet implemented.**
    - `git mark revert <name>`


## Upcoming features
- Add a `git mark revert` command. To revert the snapshot to the previous state.
- Encryption to the marks store with AES-256-GCM.
