# Contributing to Musket

## Requirements
Last stable Rust toolchain. Use [Rustup](https://rustup.rs/) to install it.

## Guidelines

* Use [Conventional Commits](https://www.conventionalcommits.org/).
* Use [Feature Branch](https://www.atlassian.com/git/tutorials/comparing-workflows/feature-branch-workflow) creating a pull request to main.
* Use [Semantic Versioning](https://semver.org/).

## Adding destinations

To add new destinations you must follow the next steps:

> Info: Use the existing destinations code files as a source of information.

### 1. Define the configuration

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

### 2. Create a module

Create a file with the name of the new destination inside [`destinations`](./src/destinations/) folder. 

#### 2.1. Develop the destination logic

Add a public `struct` with the fields needed to configure the destination. This fields must be `pub`.

Add all the login needed to send the URL, and the tags, to the destination in the `Destination` trait implementation.

#### 2.2. Handle the errors

In the [`errors.rs`](./src/destinations/errors.rs) file, add the new destination as a variant of the enum `DestinationError` and add the new destination in the pattern matching in the `Display` trait implementation of the `DestinationError` .

Back in the module file, implement the `From` trait for `DestinationError`.

#### 2.3. Enable the module

Once created, add the new module as a public module in the `destination` module inside the [`mod.rs`](./src/destinations/mod.rs) file.

For example:

```rust
pub mod bluesky;
pub mod linkedin;
pub mod turso;
```
> Info: Add the modules in alphabetical order.

### 3. Manage new destination from the CLI

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

### 4. Create a Command

Create a file with the name of the new destination inside [`commands`](./src/commands/) folder. 

This file must implement a function named `execute` in charge of perform the sending of the URL (and tags if needed) to the destination.

Once created, add the new module as a public module in the `commands` module inside the [`mod.rs`](./src/commands/mod.rs) file.

For example:

```rust
pub mod bluesky;
pub mod linkedin;
pub mod turso;
```

### 5. Manage new destination from the lib

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

### 6. Update the documentation

In the [`README.md`](./README.md) file, add a documentation about how to configure the new destination inside the section [Configure the destinations](./README.md#3--configure-the-destinations).