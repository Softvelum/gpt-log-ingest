# Stream-check response step limit reached

Product: Nimble Streamer
Component: Linux stream availability checks
Logging function: log_e
Log level: error
Documentation ID: log_e-cf4466beda91545e

## Log pattern

```text
max steps reached for %s task.
```

## Parameters

- Argument 1 — task: Panel-supplied stream-check task identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
max steps reached for check-a task.
```

## Short description

Stream-check response step limit reached.

## Trigger condition

A successful HTTP or RTSP response arrives after more than five follow-up steps.

## Explanation

The task shuts down before interpreting that response body.

## Possible causes

- Redirects or HLS playlist traversal exceeded the response-step limit.

## What to check

1. Inspect the monitored URL and its sequence of redirects and playlists.

## Recommended action

Shorten the route from the monitored URL to usable stream information.
