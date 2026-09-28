# HTTP/2 queued reply type invalid

Product: Nimble Streamer
Component: HTTP/2 connection
Logging function: log_e
Log level: error
Documentation ID: log_e-357b35bc5bc6e1a9

## Log pattern

```text
Http2Connection wrong server stream type p=%p st=%d type=%d
```

## Parameters

- Argument 1 — Connection identity: In-process connection object address, pointer displayed in hexadecimal; useful only for correlating this process lifetime.
- Argument 2 — Connection state: Connection state code, integer: 0 awaiting preface, 1 awaiting settings, 2 sending settings/acknowledgment, 3 main processing, 4 sending GOAWAY, 5 shutting down, 6 stopped.
- Argument 3 — Reply type: Internal queued reply type code converted to integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
Http2Connection wrong server stream type p=0x12340000 st=3 type=99
```

## Short description

HTTP/2 queued reply type invalid.

## Trigger condition

The output scheduler encounters a queued stream type outside its supported request, options, header, memory, file, live, and SLDP categories.

## Explanation

Output-frame preparation fails and the main output caller shuts down the connection.

## Possible causes

- The queued reply contains an unsupported or inconsistent internal type.

## What to check

1. Correlate the connection and numeric reply type with preceding request processing.

## Recommended action

Investigate repeated occurrences as an internal reply-state defect.
