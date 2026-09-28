# Server entered background mode

Product: Nimble Streamer
Component: Server startup
Logging function: log_i
Log level: info
Documentation ID: log_i-e11ef73592e6a2dc

## Log pattern

```text
process is daemonized
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
process is daemonized
```

## Short description

Server entered background mode.

## Trigger condition

The background-mode operation succeeds on a non-Windows build.

## Explanation

Server initialization and execution follow this message. The selected operation preserves the working directory and standard file descriptors.

## Related log messages

- [Could not enter background service mode](log_e-37eab7c599d5a12a.md)
