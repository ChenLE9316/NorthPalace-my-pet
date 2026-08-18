# Security Policy

## Development status

NorthPalace-my-pet is currently pre-release software. No 0.x build should be treated as a supported production-security release until the repository explicitly publishes a supported version policy.

Security-sensitive architecture is still taken seriously during development, especially around Tauri command boundaries, Windows sensors, local persistence, privacy rules and future local AI workers.

## Reporting a vulnerability

Please do **not** publish exploit details, secrets, private user data or reproducible sensitive traces in a public issue.

Preferred reporting path:

1. use GitHub private vulnerability reporting / a private Security Advisory for this repository when that feature is available;
2. otherwise contact the repository owner privately through GitHub before public disclosure.

A useful report includes:

- affected version, commit SHA or date;
- Windows version and relevant runtime environment;
- minimal reproduction steps;
- expected versus observed behavior;
- practical security/privacy impact;
- whether the issue crosses a permission, privacy or process boundary;
- sanitized logs or screenshots only when necessary.

## Sensitive data handling

Do not attach raw copies of user-local data unless they are strictly required and have been sanitized first. In particular, avoid sharing:

- `lenvu.sqlite3` or other local databases;
- `privacy-rules.json`;
- `.env` files or credentials;
- model prompts/conversation data containing secrets;
- screenshots containing private application content;
- signing keys or certificate private-key containers;
- local logs that include usernames, filesystem paths or private content.

The repository CI includes a tracked-data guard for common local/private artifacts, but that automated check is not a substitute for manual review before publishing logs or reports.

## Security boundaries in scope

Reports are especially valuable when they affect one of these boundaries:

- Tauri/WebView command or capability exposure;
- native Windows window/input behavior;
- fail-closed foreground-app and structured-context privacy gates;
- bounded Windows UI Automation access;
- SQLite/memory/history data integrity or unintended disclosure;
- launch-at-login or system-tray behavior;
- filesystem path handling;
- future LLM/vision worker process isolation and IPC;
- future screen capture / Computer Use permission boundaries.

## Disclosure and response

During the pre-release phase there is no guaranteed response SLA. Valid reports will be evaluated against the current `main` branch and the intended privacy/security boundaries. Fixes may land before a detailed public write-up when early disclosure would increase risk.

When a public release policy is established, this file should be updated with supported versions, response targets and any version-specific mitigation guidance.
