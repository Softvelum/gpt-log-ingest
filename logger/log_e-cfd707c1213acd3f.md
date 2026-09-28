# HLS relative item URL could not be resolved

Product: Nimble Streamer
Component: HLS and MPEG-TS HTTP input
Logging function: log_e
Log level: error
Documentation ID: log_e-cfd707c1213acd3f

## Log pattern

```text
failed to get absolute url base url='%s' relative url='%s'
```

## Parameters

- Argument 1 — base\_url: Directory portion of the current URL including its last slash.
- Argument 2 — relative\_url: Relative reference supplied for expansion.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get absolute url base url='https://' relative url='next.m3u8'
```

## Short description

HLS relative item URL could not be resolved.

## Trigger condition

An item URL is relative, a directory base was extracted from the current URL, but that base cannot be parsed into a nonempty protocol and host.

## Explanation

This helper logs the failure yet still returns true at this point. Callers therefore may continue with an unchanged or incomplete item URL; this message alone does not cause an immediate shutdown. The helper is used for playlist, key, and initialization-segment references.

## Possible causes

- The directory base URL cannot be split into a nonempty protocol and host.
- A current URL with no slash after its authority can produce an empty-host directory base, for example https://origin.example becoming https://.

## What to check

1. Check the directory base URL and how the current request URL was selected.
2. Check whether the current origin URL has an explicit path after its host.

## Recommended action

Correct the malformed base URL.
