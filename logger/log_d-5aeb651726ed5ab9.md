# Saved-session JSON contains an unknown section

Product: Nimble Streamer
Component: Playback session tracking
Logging function: log_d
Log level: debug
Documentation ID: log_d-5aeb651726ed5ab9

## Log pattern

```text
unknown element: '%s'
```

## Parameters

- Argument 1 — section: Unrecognized top-level JSON member name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unknown element: 'extra'
```

## Short description

Saved-session JSON contains an unknown section.

## Trigger condition

The whole-file loader encounters a top-level member other than the active or inactive session arrays.

## Explanation

The member is ignored and loading continues with other members.
