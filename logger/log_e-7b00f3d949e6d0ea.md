# Caption language list exceeds track capacity

Product: Nimble Streamer
Component: Caption and subtitle handling
Logging function: log_e
Log level: error
Documentation ID: log_e-7b00f3d949e6d0ea

## Log pattern

```text
cea708_langs: %zu languages given, only the first %zu (CC1-CC%zu) are used; the rest are dropped
```

## Parameters

- Argument 1 — Given: Number of supplied languages.
- Argument 2 — Retained: Maximum supported language tracks, four.
- Argument 3 — Last channel: Highest retained caption channel number, four.

## Log examples

Illustrative examples; these are not captured server logs.

```text
cea708_langs: 6 languages given, only the first 4 (CC1-CC4) are used; the rest are dropped
```

## Short description

Caption language list exceeds track capacity.

## Trigger condition

The configured CEA-708 language list contains more than four entries.

## Explanation

Only the first four languages are retained. This configuration path is available with caption-library support.

## Possible causes

- The configured CEA-708 language list contains more than four entries.

## What to check

1. Check the ordering of the configured caption languages.

## Recommended action

Place the intended four languages first or reduce the list to four entries.
