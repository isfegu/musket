# Musket

@todo

## Contributing

@todo

### Requirements

Last stable Rust toolchain. Use [Rustup](https://rustup.rs/) to install it.

#### Turso

To contribute to the Turso destination, you must:

1. [Create a Turso account](https://app.turso.tech).
2. [Install Turso CLI](https://docs.turso.tech/quickstart).

### Guidelines

* Use [Conventional Commits](https://www.conventionalcommits.org/).
* Use [Feature Branch](https://www.atlassian.com/git/tutorials/comparing-workflows/feature-branch-workflow) creating a pull request to main.
* Use [Semantic Versioning](https://semver.org/).

## Usage

### Install

@todo

### Turso

If you want to send URLs to Turso, you must:

1. [Create a Turso account](https://app.turso.tech).
2. Create a Turso Database.
3. Create a Table with the following schema:
```sql
CREATE TABLE links (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  url TEXT,
  tags TEXT,
  created DATETIME
);
```
4. Create a Turso Database Token and use it to populate the TURSO_AUTH_TOKEN environment variable:
```bash
export TURSO_AUTH_TOKEN=<TOKEN>
```
5. Get the Turso Database URL and use it to populate the TURSO_DATABASE_URL environment variable:
```bash
export TURSO_DATABASE_URL=<URL>
```

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
