# Frontend Settings Configuration Guide

**For Frontend Developers**

This guide documents all settings that can be configured from the frontend through the Phoenix AGI API. Use this as a reference when building configuration UIs, settings pages, or preference panels.

---

## Table of Contents

1. [Core Configuration Settings](#core-configuration-settings)
2. [Relational State Settings](#relational-state-settings)
3. [Archetype Matching & Personality Configuration](#archetype-matching--personality-configuration)
4. [Memory Management Settings](#memory-management-settings)
5. [System Access Settings](#system-access-settings)
6. [Outlook COM Settings (Windows Only)](#outlook-com-settings-windows-only)
7. [Google OAuth Settings](#google-oauth-settings)
8. [Ecosystem Management Settings](#ecosystem-management-settings)
9. [Evolution/GitHub Settings](#evolutiongithub-settings)
10. [Settings UI Best Practices](#settings-ui-best-practices)

---

## Core Configuration Settings

### Endpoint: `GET /api/config`
**Purpose**: Retrieve current configuration values

**Response:**
```json
{
  "openrouter_api_key_set": true,
  "user_name": "John",
  "user_preferred_alias": "Dad"
}
```

**Fields:**
- `openrouter_api_key_set` (boolean): Whether OpenRouter API key is configured (value not returned for security)
- `user_name` (string | null): User's name
- `user_preferred_alias` (string | null): Preferred alias/nickname for the user

---

### Endpoint: `POST /api/config`
**Purpose**: Update core configuration settings

**Request Body:**
```json
{
  "openrouter_api_key": "sk-or-v1-...",  // Optional: Set or update API key
  "user_name": "John",                    // Optional: Set user name
  "user_preferred_alias": "Dad"           // Optional: Set preferred alias
}
```

**Response:**
```json
{
  "status": "ok",
  "openrouter_api_key_set": true,
  "user_name": "John",
  "user_preferred_alias": "Dad",
  "llm_status": "online"  // "online" | "offline"
}
```

**Notes:**
- All fields are optional - only include fields you want to update
- Setting a field to empty string (`""`) will remove it
- Updating `openrouter_api_key` will automatically reload the LLM orchestrator
- Changes are persisted to `.env` file and take effect immediately

**Example Usage:**
```typescript
// Update user name and alias
await fetch('/api/config', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    user_name: 'John',
    user_preferred_alias: 'Dad'
  })
});

// Update OpenRouter API key
await fetch('/api/config', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    openrouter_api_key: 'sk-or-v1-your-new-key-here'
  })
});

// Remove user name
await fetch('/api/config', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    user_name: ''
  })
});
```

---

## Relational State Settings

### Endpoint: `GET /api/relational-state`
**Purpose**: Get current relationship state (score and sentiment)

**Response:**
```json
{
  "score": 75,           // Integer: 0-100
  "sentiment": "positive"  // String: "positive" | "negative" | "neutral"
}
```

**Fields:**
- `score` (integer): Relationship score from 0-100 (default: 50)
- `sentiment` (string): Current sentiment - "positive", "negative", or "neutral" (default: "neutral")

---

### Endpoint: `POST /api/relational-state`
**Purpose**: Update relationship state

**Request Body:**
```json
{
  "score": 80,           // Optional: Integer 0-100 (will be clamped)
  "sentiment": "positive"  // Optional: "positive" | "negative" | "neutral"
}
```

**Response:**
```json
{
  "score": 80,
  "sentiment": "positive"
}
```

**Validation:**
- `score` is automatically clamped to 0-100 range
- `sentiment` must be one of: "positive", "negative", "neutral"
- Invalid sentiment values return 400 Bad Request

**Example Usage:**
```typescript
// Update relationship score
await fetch('/api/relational-state', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    score: 85
  })
});

// Update sentiment
await fetch('/api/relational-state', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    sentiment: 'positive'
  })
});

// Update both
await fetch('/api/relational-state', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    score: 90,
    sentiment: 'positive'
  })
});
```

---

## Archetype Matching & Personality Configuration

### Endpoint: `POST /api/archetype/match`
**Purpose**: Match a user profile against zodiac archetypes to find compatible personality matches

**Request Body:**
```json
{
  "personalInfo": {
    "name": "John",
    "ageRange": "30-35",
    "location": "New York"
  },
  "communicationStyle": {
    "style": "Direct",        // "Direct" | "Playful" | "Thoughtful" | "Warm" | "Reflective"
    "energyLevel": 0.8,        // Float: 0.0-1.0
    "openness": 0.7,          // Float: 0.0-1.0
    "assertiveness": 0.6,     // Float: 0.0-1.0
    "playfulness": 0.5        // Float: 0.0-1.0
  },
  "emotionalNeeds": {
    "affectionNeed": 0.7,           // Float: 0.0-1.0
    "reassuranceNeed": 0.5,          // Float: 0.0-1.0
    "emotionalAvailability": 0.8,    // Float: 0.0-1.0
    "intimacyDepth": 0.6,            // Float: 0.0-1.0
    "conflictTolerance": 0.7,        // Float: 0.0-1.0
    "impulsivity": 0.4               // Float: 0.0-1.0
  },
  "loveLanguages": {
    "wordsOfAffirmation": 0.8,   // Float: 0.0-1.0
    "qualityTime": 0.7,           // Float: 0.0-1.0
    "physicalTouch": 0.6,         // Float: 0.0-1.0
    "actsOfService": 0.5,         // Float: 0.0-1.0
    "gifts": 0.4                  // Float: 0.0-1.0
  },
  "attachmentStyle": {
    "style": "Secure",            // "Secure" | "Anxious" | "Avoidant" | "Disorganized"
    "description": "Comfortable with intimacy and independence"
  },
  "relationshipGoals": {
    "goals": ["Deep connection", "Growth", "Support"],
    "intimacyComfort": "Deep"     // "Light" | "Deep" | "Eternal"
  },
  "interests": {
    "hobbies": ["Hiking", "Reading", "Coding"],
    "favoriteTopics": ["Technology", "Philosophy", "Science"]
  }
}
```

**Response:**
```json
{
  "matches": [
    {
      "sign": "Leo",
      "name": "The Confident Leader",
      "description": "Bold, charismatic, and natural leader...",
      "compatibility": 0.85,        // Float: 0.0-1.0
      "traits": {
        "energy": 0.9,
        "affection_need": 0.8,
        // ... other trait values
      },
      "styleBias": "Direct",
      "moodPreferences": ["confident", "energetic"]
    }
    // ... up to 3 matches, sorted by compatibility
  ]
}
```

**Notes:**
- Returns top 3 matches sorted by compatibility score
- Compatibility is calculated based on communication style, emotional needs, and attachment style
- Use this to help users find the best archetype match before applying

---

### Endpoint: `POST /api/archetype/apply`
**Purpose**: Apply a zodiac archetype to Phoenix's personality, updating environment variables

**Request Body:**
```json
{
  "sign": "Leo",  // Zodiac sign: "Aries" | "Taurus" | "Gemini" | "Cancer" | "Leo" | "Virgo" | "Libra" | "Scorpio" | "Sagittarius" | "Capricorn" | "Aquarius" | "Pisces"
  "profile": {
    // Same structure as /api/archetype/match request
  }
}
```

**Response:**
```json
{
  "success": true,
  "message": "Sola's personality updated to Leo archetype",
  "updatedEnvVars": {
    "HOROSCOPE_SIGN": "Leo",
    "USER_NAME": "John",
    "USER_PREFERRED_ALIAS": "John",
    "RELATIONSHIP_TEMPLATE": "IntimatePartnership",
    "RELATIONSHIP_INTIMACY_LEVEL": "Deep",
    "RELATIONSHIP_ATTACHMENT_STYLE": "Secure",
    "PARTNER_MODE_ENABLED": "true",
    "PARTNER_AFFECTION_LEVEL": "0.85"
  }
}
```

**Automatically Configured Settings:**
When applying an archetype, the following environment variables are automatically set:
- `HOROSCOPE_SIGN`: The selected zodiac sign
- `USER_NAME`: From `personalInfo.name`
- `USER_PREFERRED_ALIAS`: From `personalInfo.name`
- `RELATIONSHIP_TEMPLATE`: Derived from relationship goals ("IntimatePartnership" | "GrowthOrientedPartnership" | "SupportivePartnership")
- `RELATIONSHIP_INTIMACY_LEVEL`: From `relationshipGoals.intimacyComfort`
- `RELATIONSHIP_ATTACHMENT_STYLE`: From `attachmentStyle.style`
- `PARTNER_MODE_ENABLED`: Set to "true" if intimacy comfort is "Deep" or "Eternal"
- `PARTNER_AFFECTION_LEVEL`: Calculated from `emotionalNeeds.affectionNeed` (0.60-0.95 range)

**Validation:**
- Invalid zodiac sign returns 400 Bad Request
- All changes are persisted to `.env` file

**Example Usage:**
```typescript
// Step 1: Match profile to find best archetype
const matchResponse = await fetch('/api/archetype/match', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify(datingProfile)
});
const { matches } = await matchResponse.json();

// Step 2: Apply the top match
const topMatch = matches[0];
const applyResponse = await fetch('/api/archetype/apply', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    sign: topMatch.sign,
    profile: datingProfile
  })
});
const result = await applyResponse.json();
console.log('Updated environment variables:', result.updatedEnvVars);
```

---

## Memory Management Settings

### Key-Value Memory Storage

**Endpoints:**
- `POST /api/memory/store` - Store a key-value memory
- `GET /api/memory/get/{key}` - Retrieve a memory
- `GET /api/memory/search?q={prefix}&limit={n}` - Search memories by prefix
- `DELETE /api/memory/delete/{key}` - Delete a memory

**Key Naming Conventions:**
- Use prefixes to organize memories: `user:`, `preference:`, `context:`, etc.
- Examples: `user:birthday`, `preference:theme`, `context:last_conversation`

**Example Usage:**
```typescript
// Store user preferences
await fetch('/api/memory/store', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    key: 'preference:theme',
    value: 'dark'
  })
});

// Search for all user preferences
const response = await fetch('/api/memory/search?q=preference:&limit=20');
const { items } = await response.json();
```

### Vector Memory (Semantic Search)

**Endpoints:**
- `POST /api/memory/vector/store` - Store vector memory (requires `VECTOR_KB_ENABLED=true`)
- `GET /api/memory/vector/search?q={query}&k={n}` - Semantic search
- `GET /api/memory/vector/all` - Get all vector memories

**Note**: Vector KB must be enabled via environment variable `VECTOR_KB_ENABLED=true`

---

## System Access Settings

### Endpoint: `GET /api/system/status`
**Purpose**: Check system access permissions

**Response:**
```json
{
  "full_access_granted": true,      // Boolean: Whether full system access is granted
  "self_modification_enabled": false // Boolean: Whether self-modification is enabled
}
```

**Note**: These are read-only settings controlled by environment variables or system configuration. They cannot be changed via API.

---

## Outlook COM Settings (Windows Only)

### Endpoint: `GET /api/outlook/status`
**Purpose**: Check Outlook COM availability

**Response:**
```json
{
  "enabled": true,      // Boolean: Whether Outlook COM is enabled
  "available": true,    // Boolean: Whether Outlook is available
  "platform": "windows" // String: "windows" | "not_windows"
}
```

**Configuration:**
- Enable via environment variable: `OUTLOOK_COM_ENABLED=true`
- Only works on Windows with Outlook 2010-2021/O365 installed
- On non-Windows platforms, `platform` will be `"not_windows"`

**Available Operations:**
- `GET /api/outlook/folders` - List all folders
- `GET /api/outlook/emails?folder={name}&max_count={n}` - Get emails
- `POST /api/outlook/send` - Send email
- `GET /api/outlook/contacts` - Get contacts
- `GET /api/outlook/calendar/events?start_date={date}&end_date={date}` - Get calendar events
- `POST /api/outlook/calendar/create-event` - Create calendar event

**Example Usage:**
```typescript
// Check if Outlook is available
const status = await fetch('/api/outlook/status');
const { enabled, available, platform } = await status.json();

if (platform === 'windows' && enabled && available) {
  // Show Outlook integration UI
} else {
  // Show "Windows only" message or hide Outlook features
}
```

---

## Google OAuth Settings

### Endpoint: `GET /api/google/auth/start`
**Purpose**: Start Google OAuth authentication flow

**Response:**
```json
{
  "auth_url": "https://accounts.google.com/o/oauth2/v2/auth?...",
  "state": "random-state-string"
}
```

**Configuration:**
Requires environment variables:
- `GOOGLE_OAUTH_CLIENT_ID`
- `GOOGLE_OAUTH_CLIENT_SECRET`
- `GOOGLE_OAUTH_REDIRECT_URL` (typically: `http://127.0.0.1:8888/api/google/oauth2/callback`)

**Example Usage:**
```typescript
// Start OAuth flow
const response = await fetch('/api/google/auth/start');
const { auth_url } = await response.json();

// Redirect user to auth_url
window.location.href = auth_url;

// After redirect, user will be sent to callback URL
// The callback will display success/failure HTML page
```

**Note**: If Google integration is not configured, the endpoint returns 400 Bad Request with an error message.

---

## Ecosystem Management Settings

### Endpoints:
- `POST /api/ecosystem/import` - Import a GitHub repository
- `GET /api/ecosystem/list` - List all imported repositories
- `GET /api/ecosystem/{id}` - Get repository metadata
- `POST /api/ecosystem/{id}/build` - Build a repository
- `POST /api/ecosystem/{id}/start` - Start a service
- `POST /api/ecosystem/{id}/stop` - Stop a service
- `DELETE /api/ecosystem/{id}` - Remove a repository

**Example Usage:**
```typescript
// Import a repository
const importResponse = await fetch('/api/ecosystem/import', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    owner: 'username',
    repo: 'repository-name',
    branch: 'main'
  })
});
const repo = await importResponse.json();

// List all repositories
const listResponse = await fetch('/api/ecosystem/list');
const repos = await listResponse.json();

// Build a repository
await fetch(`/api/ecosystem/${repo.id}/build`, {
  method: 'POST'
});

// Start a service
await fetch(`/api/ecosystem/${repo.id}/start`, {
  method: 'POST'
});
```

---

## Evolution/GitHub Settings

### Endpoint: `GET /api/evolution/status`
**Purpose**: Get GitHub evolution pipeline status (read-only)

**Response:**
```json
{
  "github_enabled": true,
  "github_org": "organization",
  "github_repo": "repository",
  "github_branch": "main"
  // Note: No token values are exposed
}
```

**Note**: This is a read-only endpoint. GitHub configuration must be set via environment variables:
- `GITHUB_TOKEN`
- `GITHUB_ORG`
- `GITHUB_REPO`
- `GITHUB_BRANCH`

---

## Settings UI Best Practices

### 1. Configuration Page Structure

Organize settings into logical sections:

```typescript
interface SettingsSections {
  core: {
    openrouterApiKey: string;
    userName: string;
    userPreferredAlias: string;
  };
  relational: {
    score: number;
    sentiment: 'positive' | 'negative' | 'neutral';
  };
  personality: {
    archetype: string | null;
    profile: DatingProfile | null;
  };
  integrations: {
    outlook: { enabled: boolean; available: boolean };
    google: { configured: boolean };
  };
}
```

### 2. Loading Current Settings

```typescript
async function loadAllSettings() {
  const [config, relational, outlook, evolution] = await Promise.all([
    fetch('/api/config').then(r => r.json()),
    fetch('/api/relational-state').then(r => r.json()),
    fetch('/api/outlook/status').then(r => r.json()),
    fetch('/api/evolution/status').then(r => r.json())
  ]);

  return {
    core: config,
    relational,
    outlook,
    evolution
  };
}
```

### 3. Saving Settings with Validation

```typescript
async function saveCoreSettings(settings: CoreSettings) {
  try {
    const response = await fetch('/api/config', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(settings)
    });

    if (!response.ok) {
      const error = await response.json();
      throw new Error(error.message || 'Failed to save settings');
    }

    const result = await response.json();
    
    // Check LLM status after API key update
    if (settings.openrouter_api_key && result.llm_status === 'offline') {
      throw new Error('LLM is offline. Please check your API key.');
    }

    return result;
  } catch (error) {
    console.error('Failed to save settings:', error);
    throw error;
  }
}
```

### 4. Relational State Updates

```typescript
async function updateRelationalState(score?: number, sentiment?: string) {
  const body: any = {};
  if (score !== undefined) body.score = Math.max(0, Math.min(100, score));
  if (sentiment !== undefined) {
    if (!['positive', 'negative', 'neutral'].includes(sentiment)) {
      throw new Error('Invalid sentiment value');
    }
    body.sentiment = sentiment;
  }

  const response = await fetch('/api/relational-state', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body)
  });

  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.message || 'Failed to update relational state');
  }

  return response.json();
}
```

### 5. Archetype Configuration Flow

```typescript
async function configureArchetype(profile: DatingProfile) {
  // Step 1: Find matches
  const matchResponse = await fetch('/api/archetype/match', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(profile)
  });
  const { matches } = await matchResponse.json();

  // Step 2: Show matches to user and let them select
  const selectedSign = await showArchetypeSelectionDialog(matches);

  // Step 3: Apply selected archetype
  const applyResponse = await fetch('/api/archetype/apply', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      sign: selectedSign,
      profile
    })
  });

  const result = await applyResponse.json();
  
  if (!result.success) {
    throw new Error(result.message);
  }

  return result.updatedEnvVars;
}
```

### 6. Platform-Specific Features

```typescript
function isWindows(): boolean {
  return navigator.platform.toLowerCase().includes('win');
}

async function checkOutlookAvailability() {
  if (!isWindows()) {
    return { available: false, reason: 'Windows only' };
  }

  const response = await fetch('/api/outlook/status');
  const status = await response.json();
  
  return {
    available: status.enabled && status.available,
    reason: status.platform !== 'windows' ? 'Not Windows' : 
            !status.enabled ? 'Not enabled' :
            !status.available ? 'Outlook not available' : null
  };
}
```

### 7. Error Handling

```typescript
async function handleSettingsError(error: any) {
  if (error.response) {
    const data = await error.response.json();
    switch (error.response.status) {
      case 400:
        return `Invalid request: ${data.message}`;
      case 404:
        return 'Setting not found';
      case 500:
        return 'Server error. Please try again later.';
      default:
        return data.message || 'Unknown error';
    }
  }
  return error.message || 'Failed to update settings';
}
```

### 8. Real-time Updates

```typescript
// Poll for settings changes (if needed)
function useSettingsSync() {
  const [settings, setSettings] = useState(null);

  useEffect(() => {
    const interval = setInterval(async () => {
      const updated = await loadAllSettings();
      setSettings(updated);
    }, 5000); // Poll every 5 seconds

    return () => clearInterval(interval);
  }, []);

  return settings;
}
```

---

## Summary of All Configurable Settings

| Setting Category | Endpoint | Read | Write | Platform |
|-----------------|----------|------|-------|----------|
| **Core Config** | `/api/config` | GET | POST | All |
| **Relational State** | `/api/relational-state` | GET | POST | All |
| **Archetype Match** | `/api/archetype/match` | - | POST | All |
| **Archetype Apply** | `/api/archetype/apply` | - | POST | All |
| **Memory (Key-Value)** | `/api/memory/*` | GET | POST/DELETE | All |
| **Memory (Vector)** | `/api/memory/vector/*` | GET | POST | All* |
| **System Status** | `/api/system/status` | GET | - | All |
| **Outlook Status** | `/api/outlook/status` | GET | - | Windows |
| **Google OAuth** | `/api/google/auth/start` | GET | - | All |
| **Ecosystem** | `/api/ecosystem/*` | GET | POST/DELETE | All |
| **Evolution Status** | `/api/evolution/status` | GET | - | All |

\* Vector memory requires `VECTOR_KB_ENABLED=true`

---

## Quick Reference: Setting Types

### String Settings
- `user_name`
- `user_preferred_alias`
- `openrouter_api_key`

### Numeric Settings
- `relational_state.score` (0-100)

### Enum Settings
- `relational_state.sentiment` ("positive" | "negative" | "neutral")
- `communication_style.style` ("Direct" | "Playful" | "Thoughtful" | "Warm" | "Reflective")
- `attachment_style.style` ("Secure" | "Anxious" | "Avoidant" | "Disorganized")
- `relationship_goals.intimacy_comfort` ("Light" | "Deep" | "Eternal")

### Float Settings (0.0-1.0)
- All `communicationStyle` values (energyLevel, openness, assertiveness, playfulness)
- All `emotionalNeeds` values
- All `loveLanguages` values

### Boolean Settings (Read-only)
- `openrouter_api_key_set`
- `full_access_granted`
- `self_modification_enabled`
- `outlook.enabled`
- `outlook.available`
- `google.configured`

---

**Last Updated**: 2024  
**Maintained By**: Phoenix AGI Development Team
