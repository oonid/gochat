## ADDED Requirements

### Requirement: Automatic update checking

The system SHALL check for updates automatically on startup.

#### Scenario: Update available on startup
- **WHEN** application starts AND a newer version is available on GitHub releases
- **THEN** the system SHALL display an update notification to the user

#### Scenario: No update available
- **WHEN** application starts AND current version is the latest
- **THEN** the system SHALL proceed normally without notification

### Requirement: Update installation

The system SHALL allow users to download and install updates through the application.

#### Scenario: User accepts update
- **WHEN** user clicks to install an available update
- **THEN** the update SHALL be downloaded in the background
- **AND** the user SHALL be prompted to restart when download completes

#### Scenario: Update download fails
- **WHEN** update download fails due to network or server error
- **THEN** an error message SHALL be displayed
- **AND** the application SHALL continue to function normally

### Requirement: Update configuration

The system SHALL allow users to disable automatic update checking via configuration.

#### Scenario: Auto-update disabled
- **WHEN** configuration has `autoUpdate: false`
- **THEN** the system SHALL NOT check for updates on startup
- **AND** manual update check SHALL still be available via menu

### Requirement: Update endpoint

The system SHALL check for updates from GitHub releases.

#### Scenario: Update check request
- **WHEN** the system checks for updates
- **THEN** it SHALL query the GitHub releases API for the latest version
- **AND** compare against the current application version
