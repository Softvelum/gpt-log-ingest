# NDI or OMT shared-library loading raised an exception

Product: Nimble Streamer
Component: NDI media integration
Logging function: log_d
Log level: debug
Documentation ID: log_d-922ce98beb260eaf

## Log pattern

```text
LoadNDILibraries exception what()='%s'
```

## Parameters

- Argument 1 — Exception detail: Diagnostic text returned by the library-loading exception.

The identical format is used by both NDI and OMT library loading.

## Log examples

Illustrative examples; these are not captured server logs.

```text
LoadNDILibraries exception what()='required library unavailable'
```

## Short description

NDI or OMT shared-library loading raised an exception.

## Trigger condition

Dynamic loading or required-symbol lookup for NDI or OMT throws a standard exception.

## Explanation

The corresponding library handle is closed and loading reports failure. The OMT occurrence retains the NDI wording in the original format, so this text alone does not identify which runtime failed.

## Possible causes

- Dynamic loading or required-symbol lookup for NDI or OMT throws a standard exception.

## What to check

1. Check the exception text and whether the failing operation was NDI or OMT initialization.
