# Help System Quick Reference

## Available Help Commands

### General Help
```
help          # Complete command reference
?             # Alias for help
commands      # Alias for help
```

### Topic-Specific Help
```
help voice      # Voice interaction & TTS/STT
help browser    # Browser control & automation
help dreams     # Dreams panel & emotional processing
help memory     # Memory system & vaults
help ecosystem  # Repository imports & integrations
help agents     # Agent spawning & management
help proactive  # Proactive communication
help theme      # UI customization
help ui         # Alias for help theme
```

## What Each Topic Covers

### `help voice`
- Voice commands (on/off, listen, speak)
- TTS engines (Coqui, ElevenLabs, Piper)
- Configuration examples
- Troubleshooting

### `help browser`
- Browser control setup
- Navigation & interaction commands
- CSS selectors guide
- Automation examples
- Troubleshooting

### `help dreams`
- Dream types (Lucid, Shared, Healing)
- Dream commands
- Soul vault storage
- Emotional processing

### `help memory`
- Memory vaults (Soul, Mind, Body)
- Cortex layers (STM, WM, LTM, EPM, RFM)
- Memory search
- Privacy information

### `help ecosystem`
- GitHub repository import
- Code analysis
- Configuration with PAT

### `help agents`
- Agent types (Research, Coding, Analysis, Task)
- Agent spawning & management
- Multi-agent coordination

### `help proactive`
- Proactive communication features
- Configuration options

### `help theme`
- Theme commands (dark/light)
- UI customization
- Branding options

## Features

✅ **Rich Markdown** - Beautiful formatting with headings, lists, code blocks, tables
✅ **Personalized** - Uses your name and AGI name from .env
✅ **Comprehensive** - Detailed guides for all major features
✅ **Examples** - Practical usage examples for every command
✅ **Tips** - Best practices and helpful tips
✅ **Screenshots** - Placeholders for visual aids
✅ **Chat-Centric** - Appears as normal chat message
✅ **Instant** - No backend call required

## Personalization

The help system uses environment variables for personalization:

```bash
# .env file
PHOENIX_NAME=Sola      # AGI name (default: Sola)
USER_NAME=User         # Your name (default: User)
```

These names appear throughout help content for a personalized experience.

## Testing

Quick test:
1. Type `help` in chat
2. Verify rich markdown response
3. Check your name appears in greeting
4. Try sub-topics: `help voice`, `help browser`, etc.

See `HELP_SYSTEM_TEST.md` for comprehensive testing guide.

## Implementation

- **Location:** `frontend_desktop/App.tsx`
- **Function:** `parseChatCommand()`
- **Rendering:** `react-markdown` with `remarkGfm`
- **Processing:** Local (no backend call)

## Documentation

- **Implementation:** `HELP_SYSTEM_IMPLEMENTATION.md`
- **Testing:** `HELP_SYSTEM_TEST.md`
- **Screenshots:** `docs/screenshots/README.md`
- **Quick Reference:** This file

## Support

For issues or questions:
1. Check help content: `help <topic>`
2. Review test guide: `HELP_SYSTEM_TEST.md`
3. Check implementation: `HELP_SYSTEM_IMPLEMENTATION.md`
4. Ask Sola directly in chat
