# Panel requested server logs

Product: Nimble Streamer
Component: WMSPanel synchronization
Logging function: log_v
Log level: verbose
Documentation ID: log_v-dcc3b95d27ae2cf6

## Log pattern

```text
Logs requested by WMSPanel
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
Logs requested by WMSPanel
```

## Short description

Panel requested server logs.

## Trigger condition

The received synchronization response sets the log-request flag.

## Explanation

The server next prepares the configured log tail as an upload and attempts a panel connection. This records the request, not upload completion.

## Related log messages

- [Requested log file could not be opened](log_e-bf5ee5f6831ea326.md)
- [Log-upload HTTP request prepared](log_v-697b8887a44c742d.md)
