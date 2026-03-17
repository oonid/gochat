## ADDED Requirements

### Requirement: Tray icon displays application state

The system SHALL display a tray icon that reflects the current message state:
- Normal icon when no unread messages
- Badge/notification icon when unread messages exist
- Offline icon when disconnected or not logged in

#### Scenario: Normal state displayed
- **WHEN** Google Chat favicon is the standard chat icon (favicon_chat_r or favicon_chat_new_non_notif_r)
- **THEN** the tray icon SHALL display the normal state icon

#### Scenario: Unread notification displayed
- **WHEN** Google Chat favicon changes to notification state (favicon_chat_new_notif_r)
- **THEN** the tray icon SHALL display the badge/notification state icon

#### Scenario: Offline state displayed
- **WHEN** favicon does not match known patterns or network is unavailable
- **THEN** the tray icon SHALL display the offline state icon

### Requirement: Tray icon supports multiple themes

The system SHALL support multiple tray icon themes:
- Default theme (green Google Chat icons)
- Colored theme (Google's colored icons)
- Mono theme (monochrome icons for dark desktop themes)

#### Scenario: User selects icon theme
- **WHEN** user changes icon theme in configuration
- **THEN** the tray icon SHALL update to use the selected theme on next restart

### Requirement: Tray context menu provides quick actions

The system SHALL provide a context menu on right-click with the following options:
- Show/Hide window toggle
- Force reload
- Separator
- Toggle third-party auth mode (with restart)
- Separator
- Quit

#### Scenario: User clicks Show/Hide
- **WHEN** user clicks "Show/Hide" in tray context menu
- **THEN** the window SHALL toggle between visible and hidden states
- **AND** the menu item text SHALL update to reflect current state

#### Scenario: User clicks Force reload
- **WHEN** user clicks "Force reload" in tray context menu
- **THEN** the webview SHALL reload the Google Chat page

#### Scenario: User clicks Quit
- **WHEN** user clicks "Quit" in tray context menu
- **THEN** the application SHALL exit completely (not minimize to tray)

### Requirement: Single-click on tray toggles window

The system SHALL toggle window visibility when user single-clicks the tray icon.

#### Scenario: Window is hidden when tray clicked
- **WHEN** user single-clicks tray icon AND window is hidden or minimized
- **THEN** the window SHALL be shown and focused

#### Scenario: Window is visible when tray clicked
- **WHEN** user single-clicks tray icon AND window is visible and focused
- **THEN** the window SHALL be hidden
