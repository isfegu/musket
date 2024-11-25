# Musket

__Musket__ is a command line interface to send a URL to several destinations. Each destination handle the URL depending the nature of the destination, for example, Turso destination stores the URL in Turso Service (a SQLite database SaaS) and LinkedIn destination publish the link in the user profile.

## Usage

### 1.- Install

```bash
cargo install musket
```

### 2.- Create the configuration file

Execute

```bash
$ musket load
```

to create the configuration file.

__Musket__ uses a configuration file named `config.toml`. This file is placed in the directory `musket` inside the users's home. This home depends of the operating system:

- [GNU/Linux](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)
- [MS Windows](https://learn.microsoft.com/es-es/windows/win32/shell/knownfolderid?redirectedfrom=MSDN)
- [macOS](https://developer.apple.com/library/archive/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html#//apple_ref/doc/uid/TP40010672-CH2-SW6)

> The `musket load` command will display the full path to the configuration file.

### 3.- Configure the destinations

#### LinkedIn

Before sending a URL to LinkedIn destination you must:

1. Create a [LinkedIn Application](https://www.linkedin.com/developers) with the _Share on LinkedIn_ and _Sign In with LinkedIn using OpenID Connect_ products added to the application.
2. Create an [access token](https://www.linkedin.com/developers/tools/oauth) with the _email_, _openid_, _profile_, _w_member_social_ permissions.
3. Get the [author identifier](https://learn.microsoft.com/es-es/linkedin/consumer/integrations/self-serve/sign-in-with-linkedin-v2#api-request-to-retreive-member-details) (doing a request to the userinfo endpoint using the access token).
4. Fill the `linkedin` section in the __Musket__ [configuration file](#2--create-the-configuration-file). You must provide:
   - the `token` used as a bearer authentication.
   - the `author` identifier.
   - `share_commentary` is the text that will be shown in the post along the link.
   - `visibility`, can be "PUBLIC" or "CONNECTIONS".

#### Turso

Before sending a URL to Turso destination you must:

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
4. Fill the `turso` section in the __Musket__ [configuration file](#2--create-the-configuration-file). You must provide the database `url` and the turso `token`.


### 4.- Sending a URL

```bash
$ musket fire --url <URL> --destination <DESTINATION> --tags <tags>
```

For example:

```bash
$ musket fire --url wikipedia.org --destination linked-in,turso --tags one,two,three
```

or

```bash
$ musket fire --url wikipedia.org -d linked-in -d turso -t one -t two -t three
```

Run `musket -h` to get the details of each subcommand and arguments.

## Contributing

### Requirements

Last stable Rust toolchain. Use [Rustup](https://rustup.rs/) to install it.

### Guidelines

* Use [Conventional Commits](https://www.conventionalcommits.org/).
* Use [Feature Branch](https://www.atlassian.com/git/tutorials/comparing-workflows/feature-branch-workflow) creating a pull request to main.
* Use [Semantic Versioning](https://semver.org/).

### Destinations

To add new destinations you must follow the next steps:

> Info: Use the Turso destination files to see the code of the following steps.

#### 1. Define the configuration

Add a `struct` to define the destination configuration to be placed in the configuration file.

```rust
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TursoConfiguration {
    pub url: String,
    pub token: String,
}
```

Add a field in the `Configuration` `struct` with the destination as a `name` and the destination configuration as a `type`:

```rust
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub turso: TursoConfiguration,
    // Add here the new destination configuration
}
```

#### 2. Create a module

Create a file with the name of the new destination inside [`destinations`](./src/destinations/) folder. 

This file must:

1. have a `struct` with the fields needed to configure the destination. This fields must be `pub`.
2. implement `Destination` trait. 

Once created, add the new module as a public module in the `destination` module inside the [`mod.rs`](./src/destinations/mod.rs) file.

```rust
pub mod turso;
```

#### 3. Manage new destination from the CLI

Add the new destination as a variant of the enum `Destinations` inside the [`cli.rs`](./src/cli.rs) file.

```rust
pub enum Destinations {
    All,
    Turso,
    // Add here the new destination
}
```

#### 4. Create a Command

Create a file with the name of the new destination inside [`commands`](./src/commands/) folder. 

This file must implement a function named `execute` in charge of perform the sending of the URL (and tags if needed) to the destination.

Once created, add the new module as a public module in the `commands` module inside the [`mod.rs`](./src/commands/mod.rs) file.

```rust
pub mod turso;
```

#### 5. Manage new destination from the main

Add the new destination as a pattern matching of the `Fire` command, and add a call to the command created above.

```rust
Destinations::Turso => {
    commands::turso::execute(&cfg, &url, &vector_of_tags).await?;
}
```