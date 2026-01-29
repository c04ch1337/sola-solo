# Sola Dual-Brain Backend Commands (Tauri)

This document lists the Rust-side Tauri commands exposed by the desktop backend.

## Dual Brain: Mode Gate

### `set_orchestrator_mode(mode)`

- Switches the global orchestrator mode.
- **Hard gate**: switching to `personal` requires the Soul Vault to be unlocked.
- Logs to `./logs/mode_audit.log`.

Source: [`set_orchestrator_mode()`](src/main.rs:267)

### `get_mode_context()`

Returns a mode-filtered view of higher-layer memory.

- `professional` → allowed layers: `L4`, `L5`
- `personal` → allowed layers: `L6`, `L7`, `L8`

Source: [`get_mode_context()`](src/main.rs:298)

### `set_persona_context(active_persona_id, trust_score, zodiac_sign)`

Sets the minimal L7 gating inputs needed for trust-based policies.

- `trust_score` is clamped to `[0.0, 1.0]`.
- Logs to `./logs/mode_audit.log`.

Source: [`set_persona_context()`](src/main.rs:250)

## Soul Vault (local-first encrypted storage)

### `unlock_soul_vault(passphrase)`

Derives an AES-256 key using Argon2id + local salt, stores it in managed state.

- Salt file: `./vault/vault_salt.bin`
- Logs to `./logs/vault_audit.log`

Source: [`unlock_soul_vault()`](src/main.rs:215)

### `lock_soul_vault()`

Zeroizes and drops the in-memory vault key.

Source: [`lock_soul_vault()`](src/main.rs:242)

### `load_vault_image(profile_id, image_index)`

Reads `./vault/profiles/**.sola`, decrypts in-memory, returns a `data:image/png;base64,...` URL.

**Trust/PII gate (L7):** if the file path looks NSFW and the trust score is below the zodiac threshold,
returns a blurred placeholder *without* decrypting.

Logs to `./logs/vault_audit.log`.

Source: [`load_vault_image()`](src/main.rs:336)

## Agentic Research Factory (optional feature)

Research is compiled behind the Cargo feature `research`.

- Enable: `cargo check --features research`

### `gather_academic_data(query)`

- **Mode gate**: Professional only.
- Returns a `MemoryInjection` targeting `L5`.
- Logs to `./logs/research_audit.log`.

Source: [`gather_academic_data()`](src/main.rs:291)

### `gather_companion_insights(target_kink)`

- **Mode gate**: Personal only.
- Returns a `MemoryInjection` targeting `L7`.
- Logs to `./logs/research_audit.log`.

Source: [`gather_companion_insights()`](src/main.rs:300)

## Notes

- All audit logs write under `./logs/` via [`audit::append_line()`](src/audit.rs:21).
- Vault crypto primitives live in [`vault.rs`](src/vault.rs:1).
- Global managed state lives in [`SolaState`](src/sola_state.rs:15).

