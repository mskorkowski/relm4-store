# Debugging rust applications in vscode

I assume you have rust and lldb plugins installed already. If not go grab them

For rust either `Rust` or `rust-analyzer`. For debugging `CodeLLDB`.

Open the `Run and Debug`. It's under this icon in toolbar

![Run and debug icon](./assets/run-and-debug.png)

Now either click `Add config` or edit existing configuration. To open existing configuration click cog wheel ![open run configuration](./assets/cog.png).

Now you can add the run configuration:

```json
        {
            "type": "lldb",
            "request": "launch",
            "name": "{{name}}",
            "cargo": {
                "args": [
                    "build",
                    "--bin={{binary_name}}",
                    "--package={{package_name}}",
                ],
                "filter": {
                    "name": "{{binary_name}}",
                    "kind": "{{artifact}}"
                },
            },
            "args": [],
            "cwd": "${workspaceFolder}",
            "env": {
                "RUST_BACKTRACE": "full",
                "RUSTFLAGS" : "-Z macro-backtrace"
            }
        }
```

in the `configurations` section. Now let's replace all sections marked by `{{...}}` accordingly

| Section | What to place there |
|:--------|:--------------------|
| name    | Description shown in the VSCode run and debug. It should be short but distinct enough that you know what you do with given task. |
| binary_name | Name of the application produced by cargo. Based on your project configuration. |
| package_name | Rust crate in which binary code lives. |
| artifact | Type of the artifact. `bin` for applications/libraries/tests. `example` for examples. |

`env` section allows you to set extra flags for your application. You can set there any rust flags. The proposed one are

| Flag | Value | Meaning |
|:-----|:--------|:------|
|RUST_BACKTRACE| full | Most comprehensive backtrace. If it's too much for you consider setting it to `1`. |
|RUSTFLAGS| "-Z macro-backtrace" | When macro fails it produces slightly better error. Useful when you write your own macros. Otherwise you can safely ignore it. |
