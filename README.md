# Musket

@todo

## Contributing

@todo

### Requirements

Last stable Rust toolchain. Use [Rustup](https://rustup.rs/) to install it.

### Guidelines

* Use [Conventional Commits](https://www.conventionalcommits.org/).
* Use [Feature Branch](https://www.atlassian.com/git/tutorials/comparing-workflows/feature-branch-workflow) creating a pull request to main.
* Use [Semantic Versioning](https://semver.org/).

## Usage

@todo

### Install

@todo

### Execute

```bash
$ musket fire --url <URL> --destination <DESTINATION> --tags <tags>
```

For example:

```bash
$ musket fire --url wikipedia.com --destination foo,bar --tags one,two,three
```

or

```bash
$ musket fire --url wikipedia.com -d foo -d bar -t one -t two -t three
```

Run `musket -h` to get the details of each subcommand and arguments.
