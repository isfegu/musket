# Musket

__Musket__ is a command line interface to send a URL to several destinations. Each destination handle the URL depending the nature of the destination, for example, Bluesky destination post the URL in the user's feed, LinkedIn destination publish the link in the user profile and Turso destination stores the URL in Turso Service (a SQLite database SaaS).

## Usage

### 1.- Install

For a while, __Musket__ is provided as a cargo package, therefore you need _cargo_ installed in your machine.

```bash
cargo install musket
```

### 2.- Create the configuration file

To create the configuration file, execute:

```bash
$ musket init
```

__Musket__ uses a configuration file named `config.toml`. This file is placed in the directory `musket` inside the users's home. This home depends of the operating system:

- [GNU/Linux](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)
- [MS Windows](https://learn.microsoft.com/es-es/windows/win32/shell/knownfolderid?redirectedfrom=MSDN)
- [macOS](https://developer.apple.com/library/archive/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html#//apple_ref/doc/uid/TP40010672-CH2-SW6)

> The `musket init` command will display the full path to the configuration file.

### 3.- Configure the destinations

All destinations have to be configured from the [configuration file](#2--create-the-configuration-file).

#### Bluesky

Before sending a URL to Bluesky destination you must:

1. Create a [Bluesky](https://bsky.app/) account. For a while, __Musket__ only suports the _Bluesky Social_ provider.
2. Fill the `bluesky` section in the __Musket__ [configuration file](#2--create-the-configuration-file). You must provide:
   - the `identifier` is the account's username or email.
   - the `password` of the account.
   - `commentary` is the default text that will be shown in the post along the link.

#### LinkedIn

Before sending a URL to LinkedIn destination you must:

1. Create a [LinkedIn Application](https://www.linkedin.com/developers) with the _Share on LinkedIn_ and _Sign In with LinkedIn using OpenID Connect_ products added to the application.
2. Create an [access token](https://www.linkedin.com/developers/tools/oauth) with the _email_, _openid_, _profile_, _w_member_social_ permissions.
3. Get the [author identifier](https://learn.microsoft.com/es-es/linkedin/consumer/integrations/self-serve/sign-in-with-linkedin-v2#api-request-to-retreive-member-details) (doing a request to the userinfo endpoint using the access token).
4. Fill the `linkedin` section in the __Musket__ [configuration file](#2--create-the-configuration-file). You must provide:
   - the `token` used as a bearer authentication.
   - the `author` identifier.
   - `commentary` is the default text that will be shown in the post along the link.
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

To send a URL you have to use the _fire_ command.

```bash
$ musket fire
```

The _fire_ command have several options:

- __-u, --url__: To set the URL to send to the destinations. Url is mandatory.
- __-d, --destination__: To set where the URL will be send. At least, one destination must be specified.
- __-t, --tags__: To set the tags to be used in the destinations. Tags are optional.
- __-c, --commentary__: To set the text that will be published along with the URL. Commentary is optional. If no text is specified, then the text set in the [configuration file](#2--create-the-configuration-file) will be used. _Turso_ destination not uses commentaries.


```bash
$ musket fire --url <URL> --destination <DESTINATION> --tags <tags> --commentary <text>
```

For example:

```bash
$ musket fire --url wikipedia.org --destination bluesky,linked-in,turso --tags one,two,three --commentary "I've just discover this amazing website!"
```

or

```bash
$ musket fire --url wikipedia.org -d bluesky -d linked-in -d turso -t one -t two -t three -c "I've just discover this amazing website!"
```

Run `musket -h` to get the details of each command and option.

## Contributing

### Requirements

Last stable Rust toolchain. Use [Rustup](https://rustup.rs/) to install it.

### Guidelines

* Use [Conventional Commits](https://www.conventionalcommits.org/).
* Use [Feature Branch](https://www.atlassian.com/git/tutorials/comparing-workflows/feature-branch-workflow) creating a pull request to main.
* Use [Semantic Versioning](https://semver.org/).

### Adding destinations

To add new destinations you must follow the next steps:

> Info: Use the existing destinations code files as a source of information.

#### 1. Define the configuration

In the [`config.rs`](./src/config.rs) file, add a `struct` to define the new destination configuration.

> Info: Add the `struct` in alphabetical order.

For example:

```rust
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TursoConfiguration {
    pub url: String,
    pub token: String,
}
```

Add a field in the `Configuration` `struct` with the destination as a `name` and the destination configuration as a `type`:

> Info: Add the field in alphabetical order.

```rust
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub bluesky: BlueskyConfiguration,
    pub linkedin: LinkedinConfiguration,
    pub turso: TursoConfiguration,
    // Add the new destination configuration
}
```

#### 2. Create a module

Create a file with the name of the new destination inside [`destinations`](./src/destinations/) folder. 

##### 2.1. Develop the destination logic

Add a public `struct` with the fields needed to configure the destination. This fields must be `pub`.

Add all the login needed to send the URL, and the tags, to the destination in the `Destination` trait implementation.

##### 2.2. Handle the errors

In the [`errors.rs`](./src/destinations/errors.rs) file, add the new destination as a variant of the enum `DestinationError` and add the new destination in the pattern matching in the `Display` trait implementation of the `DestinationError` .

Back in the module file, implement the `From` trait for `DestinationError`.

##### 2.4. Enable the module

Once created, add the new module as a public module in the `destination` module inside the [`mod.rs`](./src/destinations/mod.rs) file.

For example:

```rust
pub mod bluesky;
pub mod linkedin;
pub mod turso;
```
> Info: Add the modules in alphabetical order.

#### 3. Manage new destination from the CLI

In the [`cli.rs`](./src/cli.rs) file, add the new destination as a variant of the enum `Destinations`.

```rust
pub enum Destinations {
    All,
    Bluesky,
    LinkedIn,
    Turso,
    // Add here the new destination
}
```

> Info: Add the modules in alphabetical order.

#### 4. Create a Command

Create a file with the name of the new destination inside [`commands`](./src/commands/) folder. 

This file must implement a function named `execute` in charge of perform the sending of the URL (and tags if needed) to the destination.

Once created, add the new module as a public module in the `commands` module inside the [`mod.rs`](./src/commands/mod.rs) file.

For example:

```rust
pub mod bluesky;
pub mod linkedin;
pub mod turso;
```

#### 5. Manage new destination from the lib

In the ['lib.rs`](./src/lib.rs) file, add the new destination as a pattern matching of the `Fire` command, and add a call to the command created above. Remember to add the command to the `Destinations::All` match.

For example:

```rust
Destinations::All => {
    bluesky::execute(&cfg, &url, &vector_of_tags).await?;
    linkedin::execute(&cfg, &url, &vector_of_tags).await?;
    turso::execute(&cfg, &url, &vector_of_tags).await?;
}
Destinations::Bluesky => {
    bluesky::execute(&cfg, &url, &vector_of_tags).await?;
}
Destinations::LinkedIn => {
    linkedin::execute(&cfg, &url, &vector_of_tags).await?;
}
Destinations::Turso => {
    turso::execute(&cfg, &url, &vector_of_tags).await?;
}
```

> Info: Add the destinations in alphabetical order.

#### 6. Update the documentation

In the [`README.md`](README.md) file, add a documentation about how to configure the new destination inside the section [Configure the destinations](#3--configure-the-destinations).