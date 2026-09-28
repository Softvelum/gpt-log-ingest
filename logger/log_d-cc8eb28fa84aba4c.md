# DVR writer stopped

Product: Nimble Streamer
Component: DVR
Logging function: log_d
Log level: debug
Documentation ID: log_d-cc8eb28fa84aba4c

## Log pattern

```text
thread stopped
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
thread stopped
```

## Short description

DVR writer stopped.

## Trigger condition

The writer loop exits and pending changes and contexts have been cleared.

## Explanation

The worker marks itself finished.

## Possible causes

- The writer loop exits and pending changes and contexts have been cleared.
