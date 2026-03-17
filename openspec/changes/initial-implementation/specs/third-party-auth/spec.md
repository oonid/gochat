## ADDED Requirements

### Requirement: Third-party OAuth authentication support

The system SHALL support authentication through external identity providers (SSO) that redirect to OAuth flows outside Google's domain.

#### Scenario: User authenticates with corporate SSO
- **WHEN** user's Google account requires SSO authentication AND user initiates login
- **THEN** the system SHALL allow navigation to external OAuth provider URLs within the webview
- **AND** the system SHALL complete the authentication flow without breaking

#### Scenario: OAuth redirect returns to Google Chat
- **WHEN** OAuth provider redirects back to Google Chat after successful authentication
- **THEN** the system SHALL navigate to the Google Chat interface
- **AND** the user SHALL be logged in

### Requirement: Third-party auth mode toggle

The system SHALL provide a mode toggle to enable/disable third-party auth handling, as this mode has trade-offs.

#### Scenario: User enables third-party auth mode
- **WHEN** user enables third-party auth mode from tray or menu
- **THEN** the system SHALL open all external URLs within the webview (not external browser)
- **AND** the application SHALL restart to apply changes

#### Scenario: User disables third-party auth mode
- **WHEN** user disables third-party auth mode (returns to regular mode)
- **THEN** the system SHALL open external URLs in the default browser
- **AND** the application SHALL restart to apply changes

### Requirement: Custom OAuth URL whitelist

The system SHALL support a configurable list of OAuth provider URLs via environment variable.

#### Scenario: Custom OAuth URLs configured
- **WHEN** environment variable `NO_REDIRECT_URL` contains comma-separated URLs
- **THEN** those URLs SHALL be treated as allowed internal navigation targets
- **AND** those URLs SHALL NOT open in external browser

#### Scenario: Default allowed URLs
- **WHEN** no custom URLs are configured
- **THEN** the system SHALL allow internal navigation for:
  - accounts.google.com
  - accounts.youtube.com
  - mail.google.com (login paths)
  - chat.google.com
