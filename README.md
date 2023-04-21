# Scheduler

Schedule and manage your tasks in a `TO-DO` or diary style.

**NOTE**: Running the application (binary) or running the unit tests will create a `db.json` file the root. Please ensure that this `db.json` file is deleted before subsequent runs, in order to avoid test failures and/or false positives.

## Setup

*Local installation*:

```bash
git clone https://github.com/kobby-pentangeli/scheduler
cd scheduler
cargo install --path .
```

*Remote installation*:

```bash
cargo install --git https://github.com/kobby-pentangeli/scheduler
```

## Running

- Add/schedule a task (a to-do item):

```bash
schapp add "YOUR TASK HERE"
```

- Mark a task as complete:

```bash
schapp complete "YOUR TASK HERE"
```

## Contributing

Thank you for considering to contribute to this project!

All contributions large and small are actively accepted.

- To get started, please read the [contribution guidelines](https://github.com/kobby-pentangeli/scheduler/blob/master/CONTRIBUTING.md).

- Browse [Good First Issues](https://github.com/kobby-pentangeli/scheduler/labels/good%20first%20issue).

## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this codebase by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
