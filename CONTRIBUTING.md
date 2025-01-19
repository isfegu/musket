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

### 1. Create the destination module

Create a file with the name of the new destination inside [`destinations`](./src/destinations/) folder and import `Destination`, `DestinationError` and `Serde`.

```rust
use super::{Destination, DestinationError};
use serde::{Deserialize, Serialize};
```

#### 1.1. Define the configuration

Add a public `struct` with the name of the destination plus the `Configuration` word. Add as many `pub` fields as you want to be saved in the [configuration file](./README.md#2--create-the-configuration-file). This `struct` must derive `Clone`, `Default`, `Serialize` and `Deserialize`. For example:

```rust
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BlueskyConfiguration {
    pub identifier: String,
    pub password: String,
    pub commentary: String,
    pub language: String,
    pub enabled: bool,
}
```

#### 1.2. Develop the destination logic

Add a public `struct` with the name of the destination. Add as many `pub` fields as you needed to use the destination. One of the fields must contain the configuration created before. For example: 

```rust
pub struct Bluesky {
    pub configuration: BlueskyConfiguration,
    pub url: String,
    pub tags: Vec<String>,
    pub commentary: String,
    pub language: String,
}
```

Next, add all the logic needed to send the URL and the tags to the destination in the `fire` method through the `Destination` trait implementation.

```rust
impl Destination for Bluesky {
    async fn fire(&self) -> Result<(), DestinationError> {
        ...
```

Obviously, import as many crates as you need. For example:

```rust
use bsky_sdk::{api::types::string::Datetime, api::xrpc, rich_text::RichText, BskyAgent};
```

#### 1.3. Handle the errors

In the [`errors.rs`](./src/destinations/errors.rs) file, add the new destination as a variant of the enum `DestinationError` and add the new destination in the pattern matching in the `Display` trait implementation of the `DestinationError` .

Back in the module file, implement as many `From` traits for `DestinationError` as the destination needs. For example:

```rust
impl From<bsky_sdk::Error> for DestinationError {
    fn from(e: bsky_sdk::Error) -> Self {
        DestinationError::LinkedIn {
            message: format!("The url cannot be sent to Bluesky due to {e}."),
        }
    }
}
```

#### 1.4. Enable the destination module

Add the new module as a public module inside the [`mod.rs`](./src/destinations/mod.rs) file.

> Info: Add the modules in alphabetical order.

```rust
pub mod bluesky;
pub mod linkedin;
pub mod mastodon;
pub mod turso;
// Add the new destination module
```

Next, add the new destination as a variant of the enum `Destinations`.

> Info: Add the modules in alphabetical order.

```rust
pub enum Destinations {
    All,
    Bluesky,
    LinkedIn,
    Mastodon,
    Turso,
    // Add here the new destination
}
```

### 2. Define the configuration

In the [`config.rs`](./src/config.rs) file add a field in the `Configuration` `struct` with the destination as a `name` and the destination configuration as a `type`:

> Info: Add the field in alphabetical order.

```rust
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub bluesky: BlueskyConfiguration,
    pub linkedin: LinkedinConfiguration,
    pub turso: TursoConfiguration,
    pub mastodon: MastodonConfiguration,
    // Add the new destination configuration
}
```

Obviously, add the new destination configuration `struct` to the imports.

```rust
use crate::destinations::{
    bluesky::BlueskyConfiguration, linkedin::LinkedinConfiguration,
    mastodon::MastodonConfiguration, turso::TursoConfiguration, // Add the new destination configuration
};
```

### 3. Manage new destination from the lib

The ['lib.rs`](./src/lib.rs) file runs the main logic of the application.

Inside the `run` function, add the new destination as a pattern matching in the `Command::Fire` using the `Destinations` `enum` variants.

```rust
for target in destinations {
    match target {
        Destinations::Bluesky => {
        }
    ...
    }
}
```

Instead of put all the logic that calls the [`Fire` method of the `Destination`](#12-develop-the-destination-logic) inside the _match arm_, you must put it in a `pub` function inside [`shooters.rs`](./src/shooters.rs) file. Name this function using the name of the destination plus `shooter` word.

```rust
success_messages.push(
    bluesky_shooter(&cfg, &url, tags.clone(), commentary.as_ref()).await?,
);
```

Remember to add the command to the `Destinations::All` match as well.

> Info: Add the destinations in alphabetical order.

### 4. Update the documentation

In the [`README.md`](./README.md) file, add a documentation about how to configure the new destination inside the section [Configure the destinations](./README.md#3--configure-the-destinations).