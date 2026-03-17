## ADDED Requirements

### Requirement: Custom CSS file loading

The system SHALL load and apply user-provided CSS from a `custom.css` file.

#### Scenario: Custom CSS file exists
- **WHEN** a `custom.css` file exists in the user config directory
- **THEN** the system SHALL inject the CSS into the Google Chat webview
- **AND** the CSS SHALL be applied on every page load

#### Scenario: Custom CSS file does not exist
- **WHEN** no `custom.css` file exists in the user config directory
- **THEN** the system SHALL create an empty template file with a comment
- **AND** no CSS injection errors shall occur

### Requirement: CSS file location

The system SHALL look for custom CSS in platform-specific locations:
- Linux/macOS: `~/.config/gochat/custom.css`
- Windows: `%APPDATA%\gochat\custom.css`

#### Scenario: User edits custom CSS
- **WHEN** user modifies the `custom.css` file
- **THEN** changes SHALL take effect on the next page reload or app restart

### Requirement: CSS injection timing

The system SHALL inject custom CSS after the page has started loading.

#### Scenario: Page loads with custom CSS
- **WHEN** Google Chat page loads
- **THEN** custom CSS SHALL be injected before the page becomes interactive
- **AND** there SHALL be no visible flash of unstyled content from custom rules
