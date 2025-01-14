# Musket

With __Musket__ a “shooter” “fires” a URL at a “target” :D.

__Musket__ is a command line interface to send a URL to several destinations. Each destination handle the URL depending the nature of the destination, for example, Bluesky, Mastodon and LinkedIn destinations post the URL in the user's feed whereas Turso destination stores the URL in Turso Service (a SQLite database SaaS).

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

All destinations have to be configured from the [configuration file](#2--create-the-configuration-file). The configuration is a .toml file, therefore, strings must be in quotes and booleans must be the word _true_ or _false_.

#### Bluesky

Before sending a URL to Bluesky destination you must:

1. Create a [Bluesky](https://bsky.app/) account. For a while, __Musket__ only suports the _Bluesky Social_ provider.
2. Fill the `bluesky` section in the __Musket__ [configuration file](#2--create-the-configuration-file). You must provide:
   - the `identifier` is the account's username or email.
   - the `password` of the account.
   - `commentary` is the default text that will be shown in the post along the link.
   - `enabled` to set whether the destination can be selected.

#### Mastodon

Before sending a URL to Mastodon destination you must:

1. Create a [Mastodon](https://docs.joinmastodon.org/) account.
2. Create a [Mastodon Application](https://www.linkedin.com/developers) with the _Write_ and _Profile_ scopes added. Once the Application has been created an access token will be generated. 
3. Fill the `mastodon` section in the __Musket__ [configuration file](#2--create-the-configuration-file). You must provide:
   - the `server` is the URL of your Mastodon account provider, for example: https://mastodon.online.
   - the `token` used as a authentication.
   - `commentary` is the default text that will be shown in the post along the link.
   - `enabled` to set whether the destination can be selected.

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
   - `enabled` to set whether the destination can be selected.

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
4. Fill the `turso` section in the __Musket__ [configuration file](#2--create-the-configuration-file). You must provide:
   - the url of the `database` where store the data.
   - the `token` used as a authentication.
   - `enabled` to set whether the destination can be selected.


### 4.- Using de CLI

Run `musket -h` to get the details of each command and option.

#### Init command

Use this command to initialize __Musket__ creating the configuration file:

```bash
$ musket init
```

The _init_ command have one options:

- __-f, --force__: Use this option to create the configuration file by overwriting the existing one.

#### Fire command

Use this command to send a URL:

```bash
$ musket fire
```

The _fire_ command have several options:

- __-u, --url__: Use this option to set the URL to send to the destinations. Url is mandatory.
- __-d, --destination__: Use this option to set where the URL will be send. At least, one destination must be specified.
- __-t, --tags__: Use this option to set the tags to be used in the destinations. Tags are optional.
- __-c, --commentary__: Use this option to set the text that will be published along with the URL. Commentary is optional. If no text is specified, then the text set in the [configuration file](#2--create-the-configuration-file) will be used. _Turso_ destination not uses commentaries.

```bash
$ musket fire --url <URL> --destination <DESTINATION> --tags <tags> --commentary <text>
```

For example:

```bash
$ musket fire --url wikipedia.org --destination bluesky,mastodon,linked-in,turso --tags one,two,three --commentary "I've just discover this amazing website!"
```

or

```bash
$ musket fire --url wikipedia.org -d bluesky -d mastodon -d linked-in -d turso -t one -t two -t three -c "I've just discover this amazing website!"
```

## Contributing

If you want to contribute to __Musket__, please read the [CONTRIBUTING.md](./CONTRIBUTING.md) document.