# Contribution guidelines

## Commit naming guidelines

You should try to follow the [Conventional Commit Guidelines](https://www.conventionalcommits.org/en/v1.0.0/) as much as possible. You do not have to declare breaking changes.

## Development Setup

For a full Development setup you need the following:
- [Frontend server](#frontend)
- [Backend server](#backend)
- SurrealDB Database
- Reverse Proxy e.g. nginx (Optional)[^proxy]

## Frontend

Requirements:
- [node](https//nodejs.org) 20 (LTS is probably ok too)
- [pnpm](https://pnpm.io)

### Setup

Install the dependencies by running
```sh
pnpm i
```

Now you can start the dev server using
```sh
pnpm dev
```
You should probably also set `NODE_TLS_REJECT_UNAUTHORIZED=0` while using a local backend server, since the SSL certs will be rejected

### Commiting

Before commiting changes to the frontend, run
```sh
pnpm format
```

## Backend

Requirements:
- [Rust Nightly](https://www.rust-lang.org/) (best installed through [rustup](https://rustup.rs/))
- [rustfmt](https://github.com/rust-lang/rustfmt) (best installed through [rustup](https://rustup.rs/))
- [SurrealDB](https://surrealdb.com)
- (Optional) Reverse Proxy (i.e. [nginx](https://nginx.com))

### Setup

First, generate a passwordless SSL Certificate. You can do this yourself, or use the `generate-certs.sh` helper script if you have a POSIX compatible shell installed.

Set up a surrealdb server, either locally or on another server. Currently, you'll need to have root login to the database, but this will be changed to scope access very soon.

After that, configure your Envrionment variables by creating a `.env` file. You can use the [`.env.example`](backend/.env.example) as an example of required variables

> Note: The WHITELIST and REVERSE_PROXY variables are only required if you're planning to run a reverse proxy for local testing

Now you can run the Backend server using `cargo run`. Note that there are code differences between debug and release builds[^buildtypes], and that the `proxy` feature is enabled by default[^proxy].
```sh
cargo run # Debug build, proxy enabled
cargo run -r # Release build, proxy enabled
cargo run --no-default-features # Debug build, proxy disabled
cargo run --no-default-features -r # Release build, proxy disabled
```

[^buildtypes]: Debug and Release builds differ in security levels, but the debug build is mostly required for local testing.
e.g. the CORS allowed_origin in debug is localhost:3000 (the frontend dev server) while the release allowed_origin is theschedule.de.
There is also a difference in the strictness of reverse proxy checks. If you directly connect to the backend server from a different device in release mode, it will panic.

[^proxy]: The proxy feature enables reverse proxy support in the rate limiter, and therefore the server as a whole. With it enabled, the rate limiter uses a different type of `KeyExtractor`
that respects nginx's `X-Forwarded-For` header. For this to work, it requires the `REVERSE_PROXY` environment variable to be set to the IP of the proxy. This also makes the server panic in
release mode[^buildtypes] if you connect from a different device not through the proxy. The `WHITELIST` environment variable is optional, and if set is an IP Address that is not rate limited.

### Commiting

Before commiting backend code, run the following command and either fix or `#[allow()]` all warnings output
```sh
cargo clippy
```

After running clippy, format the code
```sh
cargo fmt
```
