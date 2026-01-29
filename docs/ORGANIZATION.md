# Repository Organization

This document describes the organization structure of the Phoenix AGI repository.

## Directory Structure

### Root Directory

The root directory contains only essential project files:
- `README.md` - Main project documentation
- `SETUP.md` - Setup instructions
- `REPOSITORY_STRUCTURE.md` - Repository structure documentation
- `Cargo.toml` - Root workspace configuration
- `Cargo.lock` - Dependency lock file
- `.gitignore` - Git ignore rules

### Documentation Organization

All documentation is organized in the `docs/` directory with the following structure:

```
docs/
├── README.md                              # Documentation index
├── integration/                           # Integration documentation
│   ├── README.md
│   ├── GITHUB_AGENT_INTEGRATION.md
│   ├── GOOGLE_ECOSYSTEM_WIRING_CONFIRMATION.md
│   ├── MASTER_ORCHESTRATOR_FRONTEND_WIRING_CONFIRMATION.md
│   ├── MASTER_ORCHESTRATOR_INTEGRATION.md
│   └── MEMORY_KB_INTEGRATION_REVIEW.md
├── ecosystem/                             # Ecosystem documentation
│   ├── README.md
│   ├── ECOSYSTEM_DESIGN.md
│   ├── ECOSYSTEM_IMPLEMENTATION_COMPLETE.md
│   ├── ECOSYSTEM_IMPLEMENTATION_PLAN.md
│   ├── ECOSYSTEM_IMPLEMENTATION_SUMMARY.md
│   └── orch_repos_docs.md
├── reviews/                               # Audit and review documentation
│   ├── README.md
│   ├── AUDIT_FINDINGS.md
│   ├── AUDIT_SUMMARY.md
│   └── VERIFICATION.md
├── plans/                                 # Planning documentation
│   ├── README.md
│   ├── process_management_plan.md
│   ├── system_access_plan.md
│   └── system_access_matrix.md
└── [Architecture Documents]               # Architecture and design docs
    ├── CONTEXT_ENGINEERING_ARCHITECTURE.md
    ├── MULTI_MODAL_ARCHITECTURE.md
    ├── FULL_CONTROL_ACCESS_ARCHITECTURE.md
    ├── SKILL_SYSTEM_ARCHITECTURE.md
    ├── GIRLFRIEND_FRAMEWORK_ARCHITECTURE.md
    ├── MASTER_ORCHESTRATION_ARCHITECTURE.md
    ├── LAYERED_KNOWLEDGE_BASE_ARCHITECTURE.md
    ├── LAYERED_MEMORY_ARCHITECTURE.md
    ├── TELEMETRY_HIVE_SWARM_ARCHITECTURE.md
    ├── IDENTITY_PERSISTENCE.md
    ├── PORTS.md
    └── SKILL.md
```

## Organization Principles

1. **Root Directory**: Contains only essential project files (README, SETUP, Cargo files)
2. **Documentation**: All documentation files are in `docs/` with logical subdirectories
3. **Categorization**: Documents are organized by purpose:
   - `integration/` - Integration and wiring confirmations
   - `ecosystem/` - Ecosystem Manager documentation
   - `reviews/` - Audit and verification documents
   - `plans/` - Planning and implementation plans
   - Root of `docs/` - Architecture and design documents
4. **Consistency**: Each subdirectory has a README.md explaining its contents

## Module Organization

All Rust modules follow a consistent structure:
- Each module is a separate crate with its own `Cargo.toml`
- Source files are in `src/` subdirectories
- Modules are organized by category (see REPOSITORY_STRUCTURE.md)

## Data Organization

- `data/` - Runtime data (vector databases, etc.)
- `models/` - ML model files (excluded from git)
- `ecosystem_repos/` - Cloned ecosystem repositories

## Build Artifacts

Build artifacts are excluded via `.gitignore`:
- `target/` - Rust build artifacts
- `**/node_modules/` - Node.js dependencies
- `**/dist/` - Frontend build outputs
- `*.db` - Database files
- `data/vector_db/` - Vector database files

## Finding Documentation

- **Architecture Docs**: `docs/*_ARCHITECTURE.md`
- **Integration Docs**: `docs/integration/`
- **Ecosystem Docs**: `docs/ecosystem/`
- **Review Docs**: `docs/reviews/`
- **Planning Docs**: `docs/plans/`

