# AAC decoder could not be opened

Product: Nimble Streamer
Component: FDK AAC audio decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-068ebb9c4be960ba

## Log pattern

```text
failed to open libfdk decoder
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to open libfdk decoder
```

## Short description

AAC decoder could not be opened.

## Trigger condition

In an FDK-enabled build, opening the AAC decoder returns no handle.

## Explanation

Decoder initialization fails. ADTS transport is requested without a supplied configuration; raw AAC is requested when configuration exists.

## Possible causes

- The AAC library could not create the requested decoder.

## What to check

1. Check AAC transport and library availability.

## Recommended action

Resolve the decoder initialization failure.
