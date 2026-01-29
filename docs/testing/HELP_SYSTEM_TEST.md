# Help System Test Guide

This document provides test cases for the enhanced help command system.

## Test Environment

- **Frontend:** React/TypeScript on port 3000
- **Backend:** Rust backend on port 8888
- **UI:** Chat-centric interface with react-markdown rendering

## Test Cases

### 1. General Help Command

**Test:** Type `help` in chat

**Expected Result:**
- Rich markdown response with complete command reference
- Personalized greeting using USER_NAME from .env
- AGI name (PHOENIX_NAME) used throughout
- Organized into categories with emojis
- Quick start section
- Links to detailed help topics
- Best practices section

**Variations to Test:**
- `help`
- `?`
- `commands`

**Verify:**
- [ ] Markdown renders correctly (headings, lists, code blocks)
- [ ] Emojis display properly
- [ ] USER_NAME appears in greeting
- [ ] PHOENIX_NAME appears throughout
- [ ] All command categories visible
- [ ] Links to sub-topics listed

---

### 2. Voice Help

**Test:** Type `help voice` in chat

**Expected Result:**
- Comprehensive voice interaction guide
- Commands section with all voice commands
- TTS/STT features explained
- UI controls section with screenshot placeholder
- Supported TTS engines (Coqui, ElevenLabs, Piper)
- Configuration examples with .env snippets
- Tips & best practices
- Examples section
- Troubleshooting section

**Verify:**
- [ ] All voice commands listed
- [ ] Configuration examples clear
- [ ] Screenshot placeholder present
- [ ] Troubleshooting tips helpful
- [ ] PHOENIX_NAME used in context

---

### 3. Browser Help

**Test:** Type `help browser` in chat

**Expected Result:**
- Complete browser control guide
- Quick setup instructions (3 steps)
- Commands organized by category (Navigation, Interaction, Data Extraction, Automation)
- CSS selectors guide with table
- Multiple examples (navigation, search, login, scraping)
- Tips & best practices
- Supported browsers list
- Configuration section
- Screenshot placeholders
- Troubleshooting section

**Verify:**
- [ ] Setup steps clear and actionable
- [ ] CSS selector table renders correctly
- [ ] Examples are practical
- [ ] Troubleshooting covers common issues
- [ ] PHOENIX_NAME used appropriately

---

### 4. Dreams Help

**Test:** Type `help dreams` in chat

**Expected Result:**
- Dreams panel comprehensive guide
- Commands for panel control and dream sessions
- Dream types explained (Lucid, Shared, Healing, Recordings)
- Storage & privacy section
- Tips & best practices
- Examples for each dream type
- Screenshot placeholders
- "How It Works" section

**Verify:**
- [ ] All dream types explained
- [ ] Privacy information clear
- [ ] Examples cover all use cases
- [ ] PHOENIX_NAME and USER_NAME used
- [ ] Soul vault encryption mentioned

---

### 5. Memory Help

**Test:** Type `help memory` in chat

**Expected Result:**
- Memory system comprehensive guide
- Commands section
- Memory vaults explained (Soul, Mind, Body)
- Cortex layers detailed (STM, WM, LTM, EPM, RFM)
- Memory Browser panel features
- Search & retrieval section
- Tips & best practices
- Examples
- Screenshot placeholders
- Memory retention information

**Verify:**
- [ ] All vaults explained clearly
- [ ] Cortex layers detailed
- [ ] Search examples practical
- [ ] Privacy information present
- [ ] PHOENIX_NAME used throughout

---

### 6. Ecosystem Help

**Test:** Type `help ecosystem` in chat

**Expected Result:**
- Ecosystem management guide
- Commands section
- Features (Repository Import, Code Analysis, Integration)
- Configuration with GitHub PAT
- Tips & best practices
- Examples (public/private repos)
- Screenshot placeholders

**Verify:**
- [ ] GitHub integration clear
- [ ] Configuration examples helpful
- [ ] Use cases explained
- [ ] PHOENIX_NAME used appropriately

---

### 7. Agents Help

**Test:** Type `help agents` in chat

**Expected Result:**
- Agent spawning comprehensive guide
- Commands section
- Agent types (Research, Coding, Analysis, Task)
- Agent capabilities explained
- Agent lifecycle detailed
- Tips & best practices
- Examples for each agent type
- Advanced usage section
- Configuration section
- Screenshot placeholders

**Verify:**
- [ ] Agent types clearly differentiated
- [ ] Lifecycle explained
- [ ] Examples practical
- [ ] Advanced usage helpful
- [ ] PHOENIX_NAME used throughout

---

### 8. Proactive Help

**Test:** Type `help proactive` in chat

**Expected Result:**
- Proactive communication guide
- Features section
- Commands
- Configuration with .env examples
- Tips & best practices
- Examples

**Verify:**
- [ ] Features explained
- [ ] Configuration clear
- [ ] PHOENIX_NAME used
- [ ] Related help links present

---

### 9. Theme Help

**Test:** Type `help theme` or `help ui` in chat

**Expected Result:**
- Theme & UI customization guide
- Theme commands
- Settings panel sections (Branding, Variables)
- Color customization
- Tips & best practices
- Examples
- Screenshot placeholders

**Verify:**
- [ ] Theme commands listed
- [ ] Settings panel explained
- [ ] Color examples clear
- [ ] Screenshot placeholders present
- [ ] PHOENIX_NAME used

---

## Integration Tests

### Test 1: Environment Variable Integration

**Setup:**
1. Edit `.env` file
2. Set `PHOENIX_NAME=Atlas`
3. Set `USER_NAME=Alex`
4. Restart backend

**Test:**
- Type `help` in chat

**Expected:**
- Greeting shows "Welcome, Alex!"
- AGI name "Atlas" used throughout
- All sub-topics use "Atlas" instead of "Sola"

**Verify:**
- [ ] USER_NAME appears in greeting
- [ ] PHOENIX_NAME used consistently
- [ ] No hardcoded "Sola" or "User" in personalized sections

---

### Test 2: Markdown Rendering

**Test:** Type `help voice` in chat

**Verify Markdown Elements:**
- [ ] H1 heading renders correctly
- [ ] H2 and H3 headings render
- [ ] Horizontal rules (---) display
- [ ] Code blocks with syntax highlighting
- [ ] Inline code with backticks
- [ ] Bold text (**text**)
- [ ] Lists (ordered and unordered)
- [ ] Tables (in browser help)
- [ ] Emojis display correctly

---

### Test 3: Screenshot Placeholders

**Test:** Type each help topic and verify screenshot placeholders

**Expected:**
- Screenshot markdown syntax: `![Alt Text](docs/screenshots/filename.png)`
- Images don't break rendering (graceful failure if missing)
- Alt text is descriptive

**Verify:**
- [ ] Voice help: voice-icons.png
- [ ] Browser help: browser-panel.png, browser-automation.png
- [ ] Dreams help: dreams-panel.png, lucid-dream.png, healing-session.png
- [ ] Memory help: memory-browser.png, memory-vaults.png, memory-search.png
- [ ] Theme help: theme-settings.png, dark-mode.png, light-mode.png
- [ ] Ecosystem help: ecosystem-panel.png, repo-import.png
- [ ] Agents help: agent-spawn.png, agents-list.png, agent-communication.png

---

## Manual Testing Checklist

### Pre-Test Setup
- [ ] Backend running on port 8888
- [ ] Frontend running on port 3000
- [ ] .env file configured with PHOENIX_NAME and USER_NAME
- [ ] Browser open to http://localhost:3000

### Test Execution
- [ ] Test general help command (`help`, `?`, `commands`)
- [ ] Test all sub-topics (`help voice`, `help browser`, etc.)
- [ ] Test with custom PHOENIX_NAME and USER_NAME
- [ ] Verify markdown rendering quality
- [ ] Check screenshot placeholders
- [ ] Test on different screen sizes
- [ ] Test in dark and light themes

### Post-Test Verification
- [ ] No console errors
- [ ] All markdown renders correctly
- [ ] Personalization works (names appear)
- [ ] Code blocks have copy buttons
- [ ] Links and references are accurate
- [ ] Help content is comprehensive and helpful

---

## Expected Behavior

### Chat Bubble Rendering
- Help appears as normal assistant message
- Uses react-markdown for rendering
- Supports GFM (GitHub Flavored Markdown)
- Code blocks have syntax highlighting and copy buttons
- Scrollable if content is long

### No New UI Components
- No modal dialogs
- No separate help panel
- All help in chat interface
- Consistent with existing UI

### Performance
- Help commands respond instantly (local processing)
- No backend call required for help
- Markdown rendering is smooth

---

## Troubleshooting

### Help Not Showing
- Check console for errors
- Verify `parseChatCommand` function in App.tsx
- Ensure react-markdown is installed

### Markdown Not Rendering
- Check react-markdown configuration
- Verify remarkGfm plugin loaded
- Check CodeBlock component

### Names Not Appearing
- Verify .env file has PHOENIX_NAME and USER_NAME
- Check envConfig state in App.tsx
- Restart backend after .env changes

### Screenshot Placeholders Breaking
- Verify markdown syntax: `![Alt](path)`
- Check that missing images don't break rendering
- Ensure graceful fallback for missing images

---

## Success Criteria

✅ **All help commands work**
✅ **Markdown renders beautifully**
✅ **Personalization with USER_NAME and PHOENIX_NAME**
✅ **Comprehensive content for all topics**
✅ **Examples are practical and clear**
✅ **Tips and best practices included**
✅ **Screenshot placeholders present**
✅ **No new UI components (chat-centric)**
✅ **No console errors**
✅ **Consistent with existing UI style**

---

## Next Steps

After testing:
1. Add actual screenshots to `docs/screenshots/`
2. Consider adding animated GIFs for complex workflows
3. Add more sub-topics as features expand
4. Gather user feedback on help content
5. Update help content as features evolve
