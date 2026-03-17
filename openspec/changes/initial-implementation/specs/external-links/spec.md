## ADDED Requirements

### Requirement: External links open in default browser

The system SHALL open external (non-Google Chat) links in the user's default web browser.

#### Scenario: User clicks external link
- **WHEN** user clicks a link that is NOT within chat.google.com or mail.google.com domains
- **THEN** the link SHALL open in the system default browser
- **AND** the Google Chat application SHALL remain open

#### Scenario: User clicks Google Meet link
- **WHEN** user clicks a Google Meet link (meet.google.com)
- **THEN** the link SHALL open in the default browser
- **AND** the meeting SHALL launch in the browser

### Requirement: Internal navigation preserved

The system SHALL allow navigation within Google Chat domains to function normally.

#### Scenario: User navigates within Google Chat
- **WHEN** user clicks a link within chat.google.com or mail.google.com/chat
- **THEN** navigation SHALL occur within the application webview
- **AND** no external browser shall open

### Requirement: Google redirect URLs cleaned

The system SHALL clean Google redirect URLs to extract the actual destination.

#### Scenario: User clicks Google redirect link
- **WHEN** user clicks a link starting with `https://www.google.com/url?`
- **THEN** the system SHALL extract the actual URL from the redirect parameters
- **AND** the extracted URL SHALL open in the default browser

#### Scenario: Redirect URL with tracking parameters
- **WHEN** redirect URL contains `&source=chat&`, `&uct=`, or `&usg=` parameters
- **THEN** those tracking parameters SHALL be removed before opening
