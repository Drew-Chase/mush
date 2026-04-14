# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | Yes       |

Only the latest release receives security updates.

## Reporting a Vulnerability

**Please do not open public issues for security vulnerabilities.**

Instead, email **dcmanproductions@gmail.com** with the subject line **"SECURITY: mush"** and include:

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

## Response Timeline

- **Acknowledgment:** within 7 days
- **Assessment:** within 14 days
- **Fix (if applicable):** best-effort, depending on severity

## Scope

The following are in scope for security reports:

- Command injection or arbitrary code execution
- Path traversal in bundled utilities
- Unsafe memory access
- Credential or environment variable exposure
- Privilege escalation

## Out of Scope

- Denial of service against a local shell session (the user already has local access)
- Bugs in third-party dependencies (report those upstream, but let us know so we can update)
