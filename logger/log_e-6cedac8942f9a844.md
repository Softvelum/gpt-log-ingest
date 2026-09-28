# Moved origin response cannot be parsed

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_e
Log level: error
Documentation ID: log_e-6cedac8942f9a844

## Log pattern

```text
failed to reparse origin response after buffer reallocation
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to reparse origin response after buffer reallocation
```

## Short description

Moved origin response cannot be parsed.

## Trigger condition

Buffered response bytes fail reparsing after the playlist response buffer is enlarged and copied.

## Explanation

HTTP 500 is prepared; playlist reception stops.

## Possible causes

- Buffered response bytes fail reparsing after the playlist response buffer is enlarged and copied.

## What to check

1. Inspect the preceding origin response and allocation messages; this indicates inconsistent buffered HTTP data.

## Recommended action

Inspect the preceding origin response and allocation messages; this indicates inconsistent buffered HTTP data.
