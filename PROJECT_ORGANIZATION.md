# SOLA Project Organization

**Last Updated**: 2026-01-22  
**Version**: 1.0.0

## Overview

This document provides a complete overview of the SOLA project organization after the comprehensive cleanup and restructuring.

## Root Directory Structure

```
pagi-twin-desktop/
├── README.md                      # Main project documentation
├── AUTONOMOUS_OPERATION.md        # Autonomous operation guide
├── REPOSITORY_STRUCTURE.md        # Project structure details
├── SECURITY.md                    # Security policies
├── DOCUMENTATION_INDEX.md         # Complete documentation index
├── CLEANUP_SUMMARY.md             # Cleanup details
├── PROJECT_ORGANIZATION.md        # This file
├── Cargo.toml                     # Rust workspace configuration
├── Cargo.lock                     # Dependency lock file
├── .env.example                   # Environment template
├── installer.iss                  # Inno Setup configuration
│
├── docs/                          # All documentation
├── tests/                         # Test scripts and utilities
├── scripts/                       # Build and setup scripts
├── phoenix-desktop-tauri/         # Desktop application (Tauri)
├── phoenix-web/                   # Backend server
├── frontend_desktop/              # Frontend application
├── [rust crates]/                 # Core Rust modules
└── [other directories]/           # Additional components
```

## Documentation Structure

### `docs/` - Main Documentation

```
docs/
├── README.md                      # Documentation overview
│
├── phases/                        # Development phases
│   ├── README.md
│   ├── PHASE_*.md                 # Phase completion docs
│   ├── FINAL_PHASE_AUDIT.md
│   └── [implementation docs]      # Feature completions
│
├── setup-guides/                  # Setup & configuration
│   ├── README.md
│   ├── QUICK_START.md
│   ├── SETUP.md
│   ├── ENV_SETUP_README.md
│   └── [other setup docs]
│
├── build-guides/                  # Build instructions
│   ├── README.md
│   ├── BUILD_INSTRUCTIONS.md
│   ├── BUILD_WINDOWS.md
│   └── BACKEND_STARTING.md
│
├── testing/                       # Test documentation
│   ├── README.md
│   ├── DEV_TEST_GUIDE.md
│   ├── TESTING_COMPLETE.md
│   └── [test status docs]
│
├── releases/                      # Release documentation
│   ├── README.md
│   ├── RELEASE_NOTES.md
│   ├── GITHUB_RELEASE_*.md
│   └── [deployment docs]
│
├── cursor-prompts/                # Cursor AI prompts
│   ├── README.md
│   ├── 00-autonomous-directive.md
│   └── [numbered prompts]
│
├── ecosystem/                     # Ecosystem documentation
│   └── [ecosystem docs]
│
├── integration/                   # Integration guides
│   └── [integration docs]
│
├── plans/                         # Planning documents
│   └── [planning docs]
│
├── reviews/                       # Code reviews & audits
│   └── [review docs]
│
├── screenshots/                   # Documentation images
│   └── [screenshots]
│
└── [architecture docs]            # Core architecture files
```

## Test Structure

### `tests/` - Test Scripts and Utilities

```
tests/
├── README.md                      # Test documentation
│
└── scripts/                       # Test scripts
    ├── test-browser.sh
    ├── test-browser-e2e.sh
    ├── test-browser-interactive.sh
    ├── test-browser-correct.sh
    ├── test-browser-command.sh
    ├── test-proactive.sh
    ├── test-proactive.ps1
    ├── test-proactive-ws.js
    ├── test-proactive-frontend.js
    └── test-memory-commands.md
```

## Scripts Structure

### `scripts/` - Build and Setup Scripts

```
scripts/
├── README.md                      # Scripts documentation
│
├── build/                         # Build & release scripts
│   ├── build_windows.cmd
│   ├── build_installer.cmd
│   ├── release-v1.0.0.ps1
│   └── release-v1.0.0.sh
│
├── setup/                         # Setup & launch scripts
│   ├── setup-env.ps1
│   ├── quick-setup.ps1
│   ├── start-backend.ps1
│   └── launcher.cmd
│
├── launch_phoenix_web.cmd         # Phoenix launcher (Windows)
├── launch_phoenix_web.sh          # Phoenix launcher (Unix)
├── launch_phoenix.sh              # Phoenix launcher
├── auto_setup_orchs.py            # ORCH setup automation
├── clone_orch.sh                  # ORCH cloning
└── refactor-prompts.js            # Prompt refactoring
```

## Core Rust Modules

### Backend Crates

```
Core Orchestration:
├── cerebrum_nexus/                # Central orchestrator
├── llm_orchestrator/              # LLM provider management
├── agent_spawner/                 # Agent creation
└── ecosystem_manager/             # Repository orchestration

Memory Systems:
├── neural_cortex_strata/          # 5-layer memory
├── vital_organ_vaults/            # Knowledge bases
├── context_engine/                # Context building
└── vector_kb/                     # Vector search

Automation:
├── browser_orch_ext/              # Browser automation
├── system_access/                 # System operations
└── desktop_capture_service/       # Screen capture

Intelligence:
├── emotion_detection/             # Emotion recognition
├── emotional_intelligence_core/   # EQ processing
├── voice_io/                      # Voice I/O
└── multi_modal_perception/        # Multi-modal AI

Security & Monitoring:
├── vascular_integrity_system/     # Audit trail
├── vital_pulse_monitor/           # Health monitoring
└── self_preservation_instinct/    # Backup management

Additional Modules:
├── skill_system/                  # Skills framework
├── code_analysis/                 # Code understanding
├── privacy_framework/             # Privacy controls
└── [many more]/                   # See Cargo.toml
```

## Frontend Applications

### Desktop Applications

```
phoenix-desktop-tauri/             # Tauri desktop app
├── README.md
├── QUICK_START.md
├── BUILD.md
├── ICON_GENERATION.md
├── generate-icons.ps1
├── src/                           # TypeScript source
├── src-tauri/                     # Rust backend
└── [other files]

frontend_desktop/                  # Web frontend
├── README.md
├── INTEGRATION.md
├── src/                           # React source
└── [other files]
```

## Navigation Guide

### Quick Access

**Essential Files:**
- Start: [`README.md`](README.md)
- Documentation: [`DOCUMENTATION_INDEX.md`](DOCUMENTATION_INDEX.md)
- Security: [`SECURITY.md`](SECURITY.md)
- Structure: [`REPOSITORY_STRUCTURE.md`](REPOSITORY_STRUCTURE.md)

**Setup & Configuration:**
- Quick Start: [`docs/setup-guides/QUICK_START.md`](docs/setup-guides/QUICK_START.md)
- Full Setup: [`docs/setup-guides/SETUP.md`](docs/setup-guides/SETUP.md)
- Environment: [`docs/setup-guides/ENV_SETUP_README.md`](docs/setup-guides/ENV_SETUP_README.md)

**Building:**
- Build Guide: [`docs/build-guides/BUILD_INSTRUCTIONS.md`](docs/build-guides/BUILD_INSTRUCTIONS.md)
- Windows Build: [`docs/build-guides/BUILD_WINDOWS.md`](docs/build-guides/BUILD_WINDOWS.md)

**Testing:**
- Test Guide: [`docs/testing/DEV_TEST_GUIDE.md`](docs/testing/DEV_TEST_GUIDE.md)
- Test Scripts: [`tests/scripts/`](tests/scripts/)

**Releases:**
- Release Notes: [`docs/releases/RELEASE_NOTES.md`](docs/releases/RELEASE_NOTES.md)
- Release Guide: [`docs/releases/GITHUB_RELEASE_GUIDE.md`](docs/releases/GITHUB_RELEASE_GUIDE.md)

### By Role

**New Users:**
1. [`README.md`](README.md) - Overview
2. [`docs/setup-guides/QUICK_START.md`](docs/setup-guides/QUICK_START.md) - Get started
3. [`docs/setup-guides/SETUP.md`](docs/setup-guides/SETUP.md) - Full setup

**Developers:**
1. [`REPOSITORY_STRUCTURE.md`](REPOSITORY_STRUCTURE.md) - Project structure
2. [`docs/BACKEND_ARCHITECTURE.md`](docs/BACKEND_ARCHITECTURE.md) - Backend design
3. [`docs/testing/DEV_TEST_GUIDE.md`](docs/testing/DEV_TEST_GUIDE.md) - Testing

**Contributors:**
1. [`README.md`](README.md) - Contributing section
2. [`docs/`](docs/) - Architecture docs
3. [`tests/`](tests/) - Test guidelines

**DevOps:**
1. [`scripts/build/`](scripts/build/) - Build scripts
2. [`docs/releases/GITHUB_RELEASE_GUIDE.md`](docs/releases/GITHUB_RELEASE_GUIDE.md) - Release process
3. [`docs/releases/CONSUMER_DEPLOYMENT_READY.md`](docs/releases/CONSUMER_DEPLOYMENT_READY.md) - Deployment

## File Naming Conventions

### Documentation
- `README.md` - Directory overview
- `UPPERCASE_NAME.md` - Major documentation
- `lowercase-name.md` - Specific guides

### Scripts
- `.ps1` - PowerShell scripts
- `.sh` - Bash scripts
- `.cmd` - Windows batch files
- `.py` - Python scripts
- `.js` - JavaScript/Node.js scripts

### Tests
- `test-*.sh` - Bash test scripts
- `test-*.ps1` - PowerShell test scripts
- `test-*.js` - JavaScript test scripts

## Directory Purposes

### Documentation Directories

| Directory | Purpose | Contents |
|-----------|---------|----------|
| `docs/phases/` | Development history | Phase completions, implementation docs |
| `docs/setup-guides/` | Setup & config | Installation, configuration guides |
| `docs/build-guides/` | Build instructions | Platform-specific build guides |
| `docs/testing/` | Test documentation | Test guides, status reports |
| `docs/releases/` | Release info | Release notes, deployment guides |
| `docs/cursor-prompts/` | AI prompts | Cursor IDE prompts |
| `docs/ecosystem/` | Ecosystem docs | ORCH and ecosystem info |
| `docs/integration/` | Integration guides | External service integration |
| `docs/plans/` | Planning docs | Design plans, matrices |
| `docs/reviews/` | Code reviews | Audit reports, reviews |

### Script Directories

| Directory | Purpose | Contents |
|-----------|---------|----------|
| `scripts/build/` | Build & release | Build scripts, release automation |
| `scripts/setup/` | Setup & launch | Environment setup, launchers |
| `tests/scripts/` | Test scripts | All test automation scripts |

## Maintenance Guidelines

### Adding New Documentation

1. **Determine Category**: Setup, build, testing, release, or architecture
2. **Place in Appropriate Directory**: Use existing structure
3. **Update README**: Add to directory's README.md
4. **Update Index**: Add to `DOCUMENTATION_INDEX.md`
5. **Link Related Docs**: Cross-reference related documentation

### Adding New Scripts

1. **Determine Purpose**: Build, setup, or test
2. **Place in Appropriate Directory**: `scripts/build/`, `scripts/setup/`, or `tests/scripts/`
3. **Update README**: Add to `scripts/README.md` or `tests/README.md`
4. **Set Permissions**: Make executable if needed
5. **Document Usage**: Add usage instructions

### Archiving Completed Work

1. **Phase Completions**: Move to `docs/phases/`
2. **Implementation Docs**: Move to `docs/phases/`
3. **Test Results**: Move to `docs/testing/`
4. **Update Phase README**: Document in `docs/phases/README.md`

## Best Practices

### Documentation
- ✅ Use clear, descriptive names
- ✅ Include README in each directory
- ✅ Cross-reference related docs
- ✅ Keep root directory clean
- ✅ Update index when adding docs

### Scripts
- ✅ Add documentation headers
- ✅ Include usage instructions
- ✅ Set appropriate permissions
- ✅ Test on target platforms
- ✅ Handle errors gracefully

### Organization
- ✅ Group related files
- ✅ Use consistent naming
- ✅ Archive completed work
- ✅ Maintain directory READMEs
- ✅ Review organization regularly

## Benefits of Current Organization

### Discoverability
- Clear directory structure
- README files for navigation
- Comprehensive documentation index
- Logical file grouping

### Maintainability
- Easy to find files
- Clear organization
- Consistent structure
- Simple to update

### Scalability
- Room for growth
- Flexible structure
- Easy to add new content
- Clear categorization

### User Experience
- Quick access to essentials
- Clear getting started path
- Easy navigation
- Comprehensive guides

## Related Documents

- [`DOCUMENTATION_INDEX.md`](DOCUMENTATION_INDEX.md) - Complete documentation index
- [`CLEANUP_SUMMARY.md`](CLEANUP_SUMMARY.md) - Cleanup details
- [`REPOSITORY_STRUCTURE.md`](REPOSITORY_STRUCTURE.md) - Technical structure
- [`README.md`](README.md) - Main project documentation

---

*This organization structure is maintained as of SOLA v1.0.0*
