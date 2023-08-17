# Lumberjack

It chops up log files

## Local Development

After cloning the repository, make any changes required.

Execute `cargo build`

Check that the build completed (it will generate an executable in `./target/debug`).

From `/lumberjack` execute `./target/debug/lumberjack` with whatever arguments like `--help`
e.g. `./target/debug/lumberjack --file ./dummy.log -d "|"`

## Plans

No particular order:

Create something that can:

- Flatten logs
- Join logs
- Query logs
- Filter logs
- Reformat logs
