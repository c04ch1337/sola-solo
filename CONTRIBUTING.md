# Contributing to SOLA

Welcome to the SOLA project! This guide will help you understand how to contribute effectively and where to place your work.

## 📋 Table of Contents

- [Project Organization](#project-organization)
- [Documentation Guidelines](#documentation-guidelines)
- [Code Guidelines](#code-guidelines)
- [File Placement Rules](#file-placement-rules)
- [Naming Conventions](#naming-conventions)
- [Before You Commit](#before-you-commit)

---

## Project Organization

SOLA follows a strict organizational structure. **Always** place files in the correct directory.

### Quick Reference

| What You're Adding | Where It Goes | Update |
|-------------------|---------------|--------|
| Setup guide | `docs/setup-guides/` | Update `docs/setup-guides/README.md` |
| Build instructions | `docs/build-guides/` | Update `docs/build-guides/README.md` |
| Test documentation | `docs/testing/` | Update `docs/testing/README.md` |
| Test script | `tests/scripts/` | Update `tests/README.md` |
| Build script | `scripts/build/` | Update `scripts/README.md` |
| Setup script | `scripts/setup/` | Update `scripts/README.md` |
| Architecture doc | `docs/` | Update `DOCUMENTATION_INDEX.md` |
| Phase completion | `docs/phases/` | Update `docs/phases/README.md` |
| Release notes | `docs/releases/` | Update `docs/releases/README.md` |
| Integration guide | `docs/integration/` | Update `docs/integration/README.md` |

### Directory Structure

```
pagi-twin-desktop/
├── docs/
│   ├── phases/              # ✅ Phase completions, implementation docs
│   ├── setup-guides/        # ✅ Setup and configuration guides
│   ├── build-guides/        # ✅ Build instructions
│   ├── testing/             # ✅ Test documentation
│   ├── releases/            # ✅ Release notes and deployment
│   ├── cursor-prompts/      # ✅ Cursor AI prompts
│   ├── ecosystem/           # ✅ Ecosystem documentation
│   ├── integration/         # ✅ Integration guides
│   ├── plans/               # ✅ Planning documents
│   ├── reviews/             # ✅ Code reviews and audits
│   └── [architecture].md    # ✅ Core architecture docs
│
├── tests/
│   └── scripts/             # ✅ All test scripts
│
├── scripts/
│   ├── build/               # ✅ Build and release scripts
│   ├── setup/               # ✅ Setup and launch scripts
│   └── [utilities]          # ✅ Utility scripts
│
└── [root]/                  # ⚠️ ONLY essential files
    ├── README.md
    ├── SECURITY.md
    ├── DOCUMENTATION_INDEX.md
    └── [core config files]
```

---

## Documentation Guidelines

### 🚫 DON'T Place in Root Directory

**Never** add documentation files to the root directory unless they are:
- `README.md` (main project overview)
- `SECURITY.md` (security policies)
- `CONTRIBUTING.md` (this file)
- `LICENSE` (license file)
- `DOCUMENTATION_INDEX.md` (master index)
- `AUTONOMOUS_OPERATION.md` (autonomous guide)
- `REPOSITORY_STRUCTURE.md` (structure overview)

### ✅ DO Place in Appropriate Subdirectory

**Always** place documentation in the correct subdirectory:

#### Setup & Configuration → `docs/setup-guides/`
```markdown
Examples:
- Installation guides
- Environment configuration
- API key setup
- Quick start guides
- Troubleshooting setup issues
```

#### Build Instructions → `docs/build-guides/`
```markdown
Examples:
- Platform-specific builds
- Compiler setup
- Build troubleshooting
- CI/CD configuration
```

#### Testing → `docs/testing/`
```markdown
Examples:
- Test strategies
- Test results
- Coverage reports
- Development status
- Service status
```

#### Releases → `docs/releases/`
```markdown
Examples:
- Release notes
- Changelog
- Deployment guides
- Version history
- Migration guides
```

#### Architecture → `docs/`
```markdown
Examples:
- System design documents
- Component architecture
- API specifications
- Data flow diagrams
- Technical specifications
```

#### Phase Completions → `docs/phases/`
```markdown
Examples:
- Phase completion reports
- Implementation summaries
- Feature completion docs
- Refactoring notes
- Historical records
```

---

## Code Guidelines

### Rust Crates

**Creating a new crate:**
1. Place in appropriate category (see `Cargo.toml`)
2. Add to workspace in `Cargo.toml`
3. Include `README.md` in crate root
4. Add tests in `tests/` directory
5. Document public APIs

**Categories:**
- Core orchestration
- Memory systems
- Automation
- Intelligence
- Security & monitoring

### Frontend Code

**React/TypeScript:**
- Place in `frontend_desktop/src/`
- Follow existing component structure
- Add tests alongside components
- Update `App.tsx` if adding routes

**Tauri Desktop:**
- Frontend: `phoenix-desktop-tauri/src/`
- Backend: `phoenix-desktop-tauri/src-tauri/`
- Icons: `phoenix-desktop-tauri/src-tauri/icons/`

---

## File Placement Rules

### 📝 Documentation Files

#### Rule 1: Setup Guides
```
IF file explains how to install, configure, or set up SOLA
THEN place in docs/setup-guides/
AND update docs/setup-guides/README.md
```

#### Rule 2: Build Guides
```
IF file explains how to build or compile SOLA
THEN place in docs/build-guides/
AND update docs/build-guides/README.md
```

#### Rule 3: Test Documentation
```
IF file documents tests, test results, or dev status
THEN place in docs/testing/
AND update docs/testing/README.md
```

#### Rule 4: Release Documentation
```
IF file documents a release, deployment, or version
THEN place in docs/releases/
AND update docs/releases/README.md
```

#### Rule 5: Architecture Documentation
```
IF file describes system architecture or design
THEN place in docs/
AND update DOCUMENTATION_INDEX.md
```

#### Rule 6: Phase Completions
```
IF file documents completed work or implementation
THEN place in docs/phases/
AND update docs/phases/README.md
```

### 🔧 Script Files

#### Rule 7: Build Scripts
```
IF script builds, compiles, or creates releases
THEN place in scripts/build/
AND update scripts/README.md
```

#### Rule 8: Setup Scripts
```
IF script sets up environment or launches services
THEN place in scripts/setup/
AND update scripts/README.md
```

#### Rule 9: Test Scripts
```
IF script runs tests or test automation
THEN place in tests/scripts/
AND update tests/README.md
```

### 🧪 Test Files

#### Rule 10: Unit Tests
```
IF file is a unit test
THEN place in [crate]/tests/
```

#### Rule 11: Integration Tests
```
IF file is an integration test script
THEN place in tests/scripts/
AND update tests/README.md
```

---

## Naming Conventions

### Documentation Files

**Use UPPERCASE for major documents:**
```
✅ GOOD:
- README.md
- SETUP.md
- BUILD_INSTRUCTIONS.md
- RELEASE_NOTES.md

❌ BAD:
- readme.md
- Setup.md
- build-instructions.md
```

**Use lowercase for specific guides:**
```
✅ GOOD:
- quick-start.md
- api-reference.md
- troubleshooting.md

❌ BAD:
- Quick-Start.md
- API-REFERENCE.md
```

### Script Files

**Use descriptive names with hyphens:**
```
✅ GOOD:
- build-windows.cmd
- setup-env.ps1
- test-browser-e2e.sh

❌ BAD:
- build.cmd
- setup.ps1
- test.sh
```

### Test Files

**Prefix with `test-`:**
```
✅ GOOD:
- test-browser.sh
- test-proactive.ps1
- test-memory-commands.md

❌ BAD:
- browser-test.sh
- proactive.ps1
- memory.md
```

---

## Before You Commit

### ✅ Checklist

**For Documentation:**
- [ ] File is in correct directory
- [ ] Directory README.md is updated
- [ ] `DOCUMENTATION_INDEX.md` is updated (if architecture doc)
- [ ] Links to related documentation added
- [ ] File follows naming conventions

**For Scripts:**
- [ ] File is in correct directory (`scripts/build/`, `scripts/setup/`, or `tests/scripts/`)
- [ ] Directory README.md is updated
- [ ] Script has documentation header
- [ ] Usage instructions included
- [ ] Permissions set correctly (Unix: `chmod +x`)

**For Code:**
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] Linter passes
- [ ] All tests pass
- [ ] No breaking changes (or documented)

**For Tests:**
- [ ] Test script in `tests/scripts/`
- [ ] Test documentation in `docs/testing/`
- [ ] Both READMEs updated
- [ ] Test passes locally

---

## Examples

### Example 1: Adding Setup Documentation

**❌ Wrong:**
```bash
# Creating file in root
echo "Setup guide" > SETUP_GUIDE.md
```

**✅ Correct:**
```bash
# Create in proper directory
echo "Setup guide" > docs/setup-guides/SETUP_GUIDE.md

# Update directory README
# Add entry to docs/setup-guides/README.md

# Update master index
# Add entry to DOCUMENTATION_INDEX.md
```

### Example 2: Adding a Test Script

**❌ Wrong:**
```bash
# Creating in root
echo "#!/bin/bash" > test-new-feature.sh
```

**✅ Correct:**
```bash
# Create in tests directory
echo "#!/bin/bash" > tests/scripts/test-new-feature.sh
chmod +x tests/scripts/test-new-feature.sh

# Update tests README
# Add entry to tests/README.md

# Add documentation
echo "Test docs" > docs/testing/NEW_FEATURE_TEST.md

# Update testing README
# Add entry to docs/testing/README.md
```

### Example 3: Adding a Build Script

**❌ Wrong:**
```bash
# Creating in scripts root
echo "Build script" > scripts/build-new.sh
```

**✅ Correct:**
```bash
# Create in build directory
echo "Build script" > scripts/build/build-new.sh
chmod +x scripts/build/build-new.sh

# Update scripts README
# Add entry to scripts/README.md under "Build Scripts"
```

### Example 4: Documenting Phase Completion

**❌ Wrong:**
```bash
# Creating in root
echo "Phase complete" > PHASE_99_COMPLETE.md
```

**✅ Correct:**
```bash
# Create in phases directory
echo "Phase complete" > docs/phases/PHASE_99_COMPLETE.md

# Update phases README
# Add entry to docs/phases/README.md

# Update master index
# Add entry to DOCUMENTATION_INDEX.md under "Project History"
```

---

## Quick Decision Tree

```
Are you adding a file?
│
├─ Is it documentation?
│  │
│  ├─ Is it about setup/config? → docs/setup-guides/
│  ├─ Is it about building? → docs/build-guides/
│  ├─ Is it about testing? → docs/testing/
│  ├─ Is it about releases? → docs/releases/
│  ├─ Is it architecture? → docs/
│  ├─ Is it a phase completion? → docs/phases/
│  └─ Is it integration? → docs/integration/
│
├─ Is it a script?
│  │
│  ├─ Does it build/release? → scripts/build/
│  ├─ Does it setup/launch? → scripts/setup/
│  └─ Does it test? → tests/scripts/
│
├─ Is it a test?
│  │
│  ├─ Unit test? → [crate]/tests/
│  └─ Integration test? → tests/scripts/
│
└─ Is it code?
   │
   ├─ Rust crate? → [appropriate category]/
   ├─ Frontend? → frontend_desktop/src/
   └─ Tauri? → phoenix-desktop-tauri/
```

---

## For AI Agents & Cursor IDE

### Autonomous Operation Rules

If you're an AI agent working on this project:

1. **Always check** `DOCUMENTATION_INDEX.md` first
2. **Never create files** in the root directory (except those listed above)
3. **Always update READMEs** when adding files to a directory
4. **Follow the decision tree** above for file placement
5. **Update the master index** when adding architecture docs
6. **Preserve organization** - don't move files without reason

### Cursor AI Prompts

When using Cursor IDE:
- Reference `docs/cursor-prompts/00-autonomous-directive.md` for autonomous operation
- Follow numbered prompts in `docs/cursor-prompts/` for specific tasks
- Add new prompts to `docs/cursor-prompts/` with sequential numbering

---

## Getting Help

### Documentation
- **Master Index**: [`DOCUMENTATION_INDEX.md`](DOCUMENTATION_INDEX.md)
- **Project Organization**: [`PROJECT_ORGANIZATION.md`](PROJECT_ORGANIZATION.md)
- **Cleanup Summary**: [`CLEANUP_SUMMARY.md`](CLEANUP_SUMMARY.md)

### Questions?
- Check existing documentation first
- Review similar files for examples
- Open an issue if unclear

---

## Summary

**Golden Rules:**
1. 🚫 **Never** add loose files to root directory
2. ✅ **Always** place files in appropriate subdirectories
3. 📝 **Always** update directory README.md
4. 🔗 **Always** update `DOCUMENTATION_INDEX.md` for architecture docs
5. 📋 **Always** follow naming conventions
6. ✅ **Always** check the decision tree when unsure

**When in doubt:**
- Look at similar existing files
- Check the directory's README.md
- Follow the decision tree above
- Ask before creating in root

---

*Thank you for contributing to SOLA and maintaining our organization!*
