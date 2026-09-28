# VAAPI profile has no H.264 profile mapping

Product: Nimble Streamer
Component: VAAPI video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-ea6f7be4e9839fb1

## Log pattern

```text
failed to get profile_idc va_profile=%d
```

## Parameters

- Argument 1 — VA profile: VAAPI profile identifier, signed integer enumeration value; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get profile_idc va_profile=-1
```

## Short description

VAAPI profile has no H.264 profile mapping.

## Trigger condition

The chosen VA profile cannot be mapped to a supported H.264 profile identifier.

## Explanation

Encoder initialization fails before hardware configuration is created.

## Possible causes

- The selected VA profile is outside baseline, constrained baseline, main, and high mappings.

## What to check

1. Check the profile setting and the reported VA profile number.

## Recommended action

Select a supported H.264 profile.
