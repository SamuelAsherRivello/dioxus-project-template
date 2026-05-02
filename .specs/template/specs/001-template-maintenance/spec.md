# Feature Specification: Template Project Maintenance

**Status**: Baseline

## Purpose

The Template Project exists to maintain a reusable Dioxus 0.7 workspace that can produce clean Generated Project repositories.

## Requirements

- **FR-001**: The repository MUST distinguish Template Project files from Generated Project starter files.
- **FR-002**: Template Project docs MUST explain how the template is maintained and copied.
- **FR-003**: Generated Project starter docs MUST be stored under `.specs/generated/` separately from root template-maintenance docs.
- **FR-004**: The project creation skill MUST apply Generated Project starter files after copying the template.
- **FR-005**: The project creation workflow MUST require explicit confirmation of both repository slug and display project name.

## Success Criteria

- **SC-001**: A maintainer can identify which files are for maintaining the template and which files seed a generated repo.
- **SC-002**: A generated repo starts with app-focused root docs instead of template-maintenance root docs.
