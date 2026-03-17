## ADDED Requirements

### Requirement: Desktop notifications for new messages

The system SHALL display native desktop notifications when new messages arrive in Google Chat.

#### Scenario: New message notification
- **WHEN** Google Chat receives a new message AND the application window is not focused
- **THEN** a native desktop notification SHALL be displayed with sender name and message preview

#### Scenario: Notification click focuses window
- **WHEN** user clicks on a desktop notification
- **THEN** the application window SHALL be shown and focused
- **AND** the relevant conversation SHALL be displayed (if supported by URL)

### Requirement: Notification permission handling

The system SHALL request notification permission on first launch if not already granted.

#### Scenario: Permission not granted on first launch
- **WHEN** application launches for the first time AND notification permission is not granted
- **THEN** the system SHALL request notification permission from the OS

#### Scenario: Permission denied
- **WHEN** user denies notification permission
- **THEN** the application SHALL continue to function without notifications
- **AND** no notification errors shall be thrown

### Requirement: Notification respects system settings

The system SHALL respect OS-level notification settings including Do Not Disturb mode.

#### Scenario: Do Not Disturb mode active
- **WHEN** OS Do Not Disturb mode is enabled
- **THEN** notifications SHALL be suppressed by the OS (no app-level override needed)
