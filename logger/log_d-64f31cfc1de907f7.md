# SDI input receiver created

Product: Nimble Streamer
Component: SDI media integration
Logging function: log_d
Log level: debug
Documentation ID: log_d-64f31cfc1de907f7

## Log pattern

```text
sdi receiver created for [%s]
```

## Parameters

- Argument 1 — Device stream: Configured SDI device or stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
sdi receiver created for [SDI input]
```

## Short description

SDI input receiver created.

## Trigger condition

The device decoder successfully creates a previously missing SDI receiver.

## Explanation

The input proceeds to read packets and forward decoded video, audio, or teletext. This message does not itself establish successful frame delivery.

## Possible causes

- Successful SDI receiver initialization.

## Related log messages

- [SDI input receiver creation failed](log_e-10c950877f303a2a.md)
