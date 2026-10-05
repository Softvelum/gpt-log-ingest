# My Nimble analysis instructions

Copy this file to `.local/user-instructions.md` and edit the preferences below,
or ask the agent to save your preferences there. The workflow reads this file
for each analysis task. Your current request overrides these saved defaults.

- Report in English.
- Prioritize stream `live/camera1` and SRT input failures.
- Group repeated messages and show measured counts.
- Include timestamps, source locations, and relevant catalog references.
- Distinguish observed service impact from diagnostic messages.
- Redact tokens and passwords from quoted excerpts.

Remove preferences that do not apply. You can add expected events to treat as
background noise, incident context, reporting style, or additional files to
include. Preferences about expected events should not conceal measured counts
or evidence that the current incident contradicts the expectation.

Save the log directory and timezone in `.local/settings.md`. Keep both files
local; `.local/` is ignored by Git.
