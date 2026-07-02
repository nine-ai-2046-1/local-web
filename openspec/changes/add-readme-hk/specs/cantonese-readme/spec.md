## ADDED Requirements

### Requirement: Cantonese README
The system SHALL provide a Cantonese README file at project root.

#### Scenario: README-HK exists
- **WHEN** user clones the repository
- **THEN** `README-HK.md` exists at project root with Cantonese content

#### Scenario: README-HK contains one-line description
- **WHEN** user opens `README-HK.md`
- **THEN** first line describes what the tool does in Cantonese

#### Scenario: README-HK contains key features
- **WHEN** user reads the features section
- **THEN** 3-5 key features are listed with brief Cantonese explanations

#### Scenario: README-HK contains installation instructions
- **WHEN** user wants to install the tool
- **THEN** installation commands are provided in Cantonese context

#### Scenario: README-HK contains usage examples
- **WHEN** user wants to use the tool
- **THEN** at least 3 usage examples are provided with Cantonese explanations

#### Scenario: README-HK contains command line options
- **WHEN** user wants to see all options
- **THEN** a table lists all command line options with Cantonese descriptions

#### Scenario: README-HK contains supported file types
- **WHEN** user wants to know what files can be served
- **THEN** file types are listed by category with Cantonese labels

### Requirement: Cantonese translation style
The README SHALL use professional Cantonese with English technical terms.

#### Scenario: Technical terms in English
- **WHEN** user reads technical terms
- **THEN** terms like "HTTPS", "CLI", "port", "directory" remain in English

#### Scenario: Instructions in Cantonese
- **WHEN** user reads instructions
- **THEN** instructions are written in natural Cantonese

#### Scenario: Code examples unchanged
- **WHEN** user views code examples
- **THEN** code blocks remain in English (unchanged from English version)

### Requirement: README-HK content consistency
The Cantonese README SHALL maintain same structure as English version.

#### Scenario: Same section order
- **WHEN** user compares both READMEs
- **THEN** sections appear in same order

#### Scenario: Same information coverage
- **WHEN** user reads both READMEs
- **THEN** same information is covered in both versions