# HLS redirect target could not be resolved

Product: Nimble Streamer
Component: HLS and MPEG-TS HTTP input
Logging function: log_e
Log level: error
Documentation ID: log_e-e06dd5d06e40c9d7

## Log pattern

```text
failed to get absolute url, base url='%s', relative url='%s'
```

## Parameters

- Argument 1 — base\_url: Current URL directory used for redirect resolution.
- Argument 2 — relative\_url: Redirect Location value as held when resolution failed.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get absolute url, base url='https://', relative url='next.m3u8'
```

## Short description

HLS redirect target could not be resolved.

## Trigger condition

An HTTP redirect has a relative Location, but its directory base cannot be parsed into a nonempty protocol and host.

## Explanation

Unlike the playlist-item URL helper, this redirect path immediately requests shutdown and returns.

## Possible causes

- The directory base URL cannot be split into a nonempty protocol and host.
- A current URL with no slash after its authority can produce an empty-host directory base, for example https://origin.example becoming https://.

## What to check

1. Check the directory base URL and how the current request URL was selected.
2. Check whether the current origin URL has an explicit path after its host.

## Recommended action

Correct the malformed base URL.
