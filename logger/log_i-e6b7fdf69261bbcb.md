# NDI output sender created

Product: Nimble Streamer
Component: NDI media integration
Logging function: log_i
Log level: info
Documentation ID: log_i-e6b7fdf69261bbcb

## Log pattern

```text
ndi sender created, ndi_name="%s"
```

## Parameters

- Argument 1 — NDI name: Runtime-provided full output name, or the requested name if unavailable.

## Log examples

Illustrative examples; these are not captured server logs.

```text
ndi sender created, ndi_name="HOST (Output)"
```

## Short description

NDI output sender created.

## Trigger condition

The normal or SEI-timecode output mode successfully creates its NDI sender.

## Explanation

The runtime-provided source name is used when available, otherwise the requested name is retained. In SEI-timecode mode the sender thread is created afterward, so this message does not prove thread startup succeeded.

## Possible causes

- Successful NDI sender creation.

## Related log messages

- [NDI output sender creation failed](log_e-06ee6420c43d6dbe.md)
- [NDI SEI-timecode sender thread could not start](log_e-ab4f0562281460ff.md)
