# Delaying synchronization after an unknown-server response

Product: Nimble Streamer
Component: WMSPanel synchronization
Logging function: log_v
Log level: verbose
Documentation ID: log_v-5fec1baa7310677e

## Log pattern

```text
sleeping for %d hours before the next sync. retry %d after UNKNOWN returned
```

## Parameters

- Argument 1 — delay\_hours: Requested delay: 1 for the first unknown retry, otherwise 2.
- Argument 2 — unknown\_retry: Consecutive unknown-response retry number, from 1 through 11 here.

## Log examples

Illustrative examples; these are not captured server logs.

```text
sleeping for 1 hours before the next sync. retry 1 after UNKNOWN returned
```

## Short description

Delaying synchronization after an unknown-server response.

## Trigger condition

An ordinary sync reports unknown after an earlier non-unknown response, and the unknown retry count is below twelve.

## Explanation

The first retry waits one hour and later retries wait two hours. The wait runs in 30-second steps, saving current statistics each step, and can end on shutdown. A full report is requested after the delay.

## Related log messages

- [Panel reports server as unknown](log_e-5c9c598933719940.md)
