## ADDED Requirements

### Requirement: Simplified English README
The system SHALL provide a simplified English README file at project root.

#### Scenario: README exists
- **WHEN** user clones the repository
- **THEN** `README.md` exists at project root with essential information

#### Scenario: README contains one-line description
- **WHEN** user opens `README.md`
- **THEN** first line describes what the tool does in simple terms

#### Scenario: README contains key features
- **WHEN** user reads the features section
- **THEN** 3-5 key features are listed with brief explanations

#### Scenario: README contains installation instructions
- **WHEN** user wants to install the tool
- **THEN** installation commands are provided (cargo build)

#### Scenario: README contains usage examples
- **WHEN** user wants to use the tool
- **THEN** at least 3 usage examples are provided (basic, HTTPS, verbose)

#### Scenario: README contains command line options
- **WHEN** user wants to see all options
- **THEN** a table lists all command line options with descriptions

#### Scenario: README contains supported file types
- **WHEN** user wants to know what files can be served
- **THEN** file types are listed by category (web, images, video, audio, etc.)

### Requirement: README content simplicity
The README SHALL be concise and easy to understand.

#### Scenario: README length
- **WHEN** user opens `README.md`
- **THEN** content is no more than 80 lines

#### Scenario: README uses simple language
- **WHEN** user reads the README
- **THEN** language is clear and avoids technical jargon where possible

#### Scenario: README has clear sections
- **WHEN** user scans the README
- **THEN** sections are clearly marked with headings