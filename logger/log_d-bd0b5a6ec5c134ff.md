# Management listener thread started

Product: Nimble Streamer
Component: HTTP management API
Logging function: log_d
Log level: debug
Documentation ID: log_d-bd0b5a6ec5c134ff

## Log pattern

```text
ServerManagementThread started
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
ServerManagementThread started
```

## Short description

Management listener thread started.

## Trigger condition

The management thread enters its run procedure.

## Explanation

The next step creates configured plain HTTP and HTTPS listening sockets. This message does not establish that any socket was opened successfully.

## Related log messages

- [Management listener thread stopped](log_d-513c7e9841541526.md)
- [No management listening interface opened](log_e-a419f663debd78fc.md)
