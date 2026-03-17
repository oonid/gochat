## ADDED Requirements

### Requirement: Single instance enforcement

The system SHALL prevent multiple instances of the application from running simultaneously.

#### Scenario: Second instance launched
- **WHEN** a second instance of the application is launched
- **THEN** the second instance SHALL exit immediately
- **AND** the existing instance window SHALL be shown and focused

#### Scenario: Second instance with deep link
- **WHEN** a second instance is launched with a `gchat://` URL argument
- **THEN** the existing instance SHALL navigate to the URL
- **AND** the existing instance window SHALL be shown and focused

### Requirement: Window state persistence

The system SHALL persist window position, size, and maximize state between sessions.

#### Scenario: User closes window in normal state
- **WHEN** user closes the application AND window is not maximized
- **THEN** the window bounds (x, y, width, height) SHALL be saved to configuration

#### Scenario: User closes maximized window
- **WHEN** user closes the application AND window is maximized
- **THEN** the system SHALL save the maximized state
- **AND** the system SHALL save the restored (non-maximized) bounds

#### Scenario: Application restarts
- **WHEN** application is launched AND saved window state exists
- **THEN** the window SHALL be restored to the saved position and size
- **AND** the window SHALL be maximized if it was maximized when last closed

### Requirement: Close to tray behavior

The system SHALL minimize to tray instead of quitting when window close button is clicked.

#### Scenario: User clicks close button
- **WHEN** user clicks the window close button (X)
- **THEN** the window SHALL be hidden (not closed)
- **AND** the application SHALL remain running in the tray

#### Scenario: User quits from tray
- **WHEN** user selects "Quit" from tray context menu
- **THEN** the application SHALL exit completely
- **AND** window state SHALL be saved before exit

### Requirement: Start hidden option

The system SHALL support starting the application hidden (minimized to tray).

#### Scenario: Start hidden enabled
- **WHEN** configuration has `startHidden: true`
- **THEN** the application SHALL launch without showing the window
- **AND** the tray icon SHALL be visible

#### Scenario: Start hidden disabled
- **WHEN** configuration has `startHidden: false` or not set
- **THEN** the application SHALL launch with the window visible
