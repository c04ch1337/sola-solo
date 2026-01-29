# 📁 SOLA File Placement Quick Reference

**Print this and keep it visible while working!**

---

## 🚫 NEVER Add to Root Directory

Only these files belong in root:
- README.md, SECURITY.md, CONTRIBUTING.md, LICENSE
- DOCUMENTATION_INDEX.md, AUTONOMOUS_OPERATION.md
- REPOSITORY_STRUCTURE.md, PROJECT_ORGANIZATION.md
- Cargo.toml, .env.example, installer.iss

**Everything else goes in subdirectories!**

---

## 📝 Documentation Files

| What You're Writing | Where It Goes | Update This |
|---------------------|---------------|-------------|
| Setup guide | `docs/setup-guides/` | `docs/setup-guides/README.md` |
| Build instructions | `docs/build-guides/` | `docs/build-guides/README.md` |
| Test documentation | `docs/testing/` | `docs/testing/README.md` |
| Release notes | `docs/releases/` | `docs/releases/README.md` |
| Architecture doc | `docs/` | `DOCUMENTATION_INDEX.md` |
| Phase completion | `docs/phases/` | `docs/phases/README.md` |
| Integration guide | `docs/integration/` | `docs/integration/README.md` |

---

## 🔧 Script Files

| Script Type | Where It Goes | Update This |
|-------------|---------------|-------------|
| Build script | `scripts/build/` | `scripts/README.md` |
| Setup script | `scripts/setup/` | `scripts/README.md` |
| Test script | `tests/scripts/` | `tests/README.md` |

---

## 🧪 Test Files

| Test Type | Where It Goes |
|-----------|---------------|
| Unit test | `[crate]/tests/` |
| Integration test | `tests/scripts/` |
| Test documentation | `docs/testing/` |

---

## 💻 Code Files

| Code Type | Where It Goes |
|-----------|---------------|
| Rust crate | `[category]/[crate_name]/` |
| Frontend | `frontend_desktop/src/` |
| Tauri frontend | `phoenix-desktop-tauri/src/` |
| Tauri backend | `phoenix-desktop-tauri/src-tauri/` |

---

## ✅ Quick Checklist

Before committing, verify:

- [ ] File is NOT in root directory
- [ ] File is in correct subdirectory
- [ ] Directory README.md updated
- [ ] `DOCUMENTATION_INDEX.md` updated (if architecture doc)
- [ ] Naming conventions followed
- [ ] Related docs cross-referenced

---

## 🎯 Decision Tree

```
What are you adding?

Documentation?
├─ About setup? → docs/setup-guides/
├─ About building? → docs/build-guides/
├─ About testing? → docs/testing/
├─ About releases? → docs/releases/
├─ Architecture? → docs/
└─ Phase done? → docs/phases/

Script?
├─ Builds code? → scripts/build/
├─ Sets up env? → scripts/setup/
└─ Runs tests? → tests/scripts/

Code?
├─ Rust? → [crate category]/
├─ Frontend? → frontend_desktop/src/
└─ Tauri? → phoenix-desktop-tauri/
```

---

## 📛 Naming Conventions

**Documentation:**
- Major: `UPPERCASE_NAME.md`
- Specific: `lowercase-name.md`

**Scripts:**
- Build: `build-*.{cmd,sh,ps1}`
- Setup: `setup-*.{ps1,sh}`
- Test: `test-*.{sh,ps1,js}`

---

## 🆘 When Unsure

1. Check similar existing files
2. Read directory README.md
3. Review `CONTRIBUTING.md`
4. Ask before creating in root!

---

## 📚 Full Documentation

- **Contributing Guide**: [`CONTRIBUTING.md`](../CONTRIBUTING.md)
- **Documentation Index**: [`DOCUMENTATION_INDEX.md`](../DOCUMENTATION_INDEX.md)
- **Project Organization**: [`PROJECT_ORGANIZATION.md`](../PROJECT_ORGANIZATION.md)

---

**Remember: Keep root clean! Everything goes in subdirectories!**
