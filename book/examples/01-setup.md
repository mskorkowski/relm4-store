# Setup

## Cargo

All examples in this book are using this dependencies in `Cargo.toml`

```toml
[dependencies]
reexport = { package = "relm4-store-reexport", version="0.1.0-beta.2" }
record = { package = "relm4-store-record", version = "0.1.0-beta.2" }
store = { package = "relm4-store", version = "0.1.0-beta.2" }
store-view = { package = "relm4-store-view-implementation", version = "0.1.0-beta.2"}
backend_inmemory = { package = "relm4-store-backend-inmemory", version = "0.1.0-beta.2" }
components = { package = "relm4-store-components", version = "0.1.0-beta.2" }
log4rs = "1.0.0"
```

### Directly from github

```toml
[dependencies]
reexport = { git = "https://github.com/mskorkowski/relm4-store", branch = "main", package = "relm4-store-reexport" }
record = { git = "https://github.com/mskorkowski/relm4-store", branch = "main", package = "relm4-store-record" }
store = { git = "https://github.com/mskorkowski/relm4-store", branch = "main", package = "relm4-store" }
store-view = { git = "https://github.com/mskorkowski/relm4-store", branch = "main", package = "relm4-store-view-implementation"}
backend_inmemory = { git = "https://github.com/mskorkowski/relm4-store", branch = "main", package = "relm4-store-backend-inmemory" }
components = { git = "https://github.com/mskorkowski/relm4-store", branch = "main", package = "relm4-store-components" }
log4rs = "1.0.0"
```

1. API is rather stable but somebody might have an idea how to make it better/easier and things will change without a notice
2. Running against anything other then `main` branch is considered mental. We try our best to keep `main` working and tested (as far as tests are created). If you find there is some feature marked as `done` on the [goals list](../02-goals.md) and there are issues with it please fill the bug.
3. If feature isn't marked as complete you can also fill the bug report. This might (or might not) get it prioritized. Definitely you will get the answer as soon as I spot it. You are welcome to contribute by the way.

## log4rs

Internally `relm4_store` is using `log` crate. `reexport::log` reexports it. In examples we are setting up `log4rs` to manage the log output. In examples folder you can find any required configuration file in the `etc` directory. For example `log4rs.yaml` for `todo_1` is in `relm4-store-examples/examples/todo1/etc/log4rs.yaml`. At the end of any chapter all configuration filles will be posted.

```yaml
{{#include ../../relm4-store-examples/examples/todo_1/etc/log4rs.yaml}}
```
