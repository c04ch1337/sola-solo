# Profiles Swipe System - Integration Tests

## Overview
This document outlines integration tests for the swipe-based profile system with AI-generated photos.

## Test Environment
- Backend: Phoenix AGI (phoenix-web) running on `http://localhost:8888`
- Frontend: React/Tauri desktop app on `http://localhost:3000`
- Profile Generator: AI photo generation with explicit content support

---

## Backend Tests

### 1. Profile Generation API

**Test: Generate Profile with Explicit Content**
```bash
curl -X POST http://localhost:8888/api/profiles/generate \
  -H "Content-Type: application/json" \
  -d '{
    "intimacy_level": "explicit",
    "preferred_traits": ["adventurous", "kinky", "open-minded"],
    "kink_preferences": ["bondage", "roleplay", "dominance"],
    "photo_count": 10,
    "explicit_photo_ratio": 0.6
  }'
```

**Expected Response:**
```json
{
  "success": true,
  "profile": {
    "id": "uuid-here",
    "name": "Alex",
    "age": 25,
    "bio": "Looking for intense connections...",
    "interests": ["Travel", "Music", "Fitness"],
    "kinks": ["bondage", "roleplay", "dominance"],
    "photos": [
      {
        "id": "photo-uuid",
        "url": "data:image/svg+xml...",
        "is_explicit": true,
        "prompt": "photorealistic portrait...",
        "generated_at": 1737659000
      }
      // ... 9 more photos (6 explicit, 4 non-explicit)
    ],
    "personality_traits": ["adventurous", "kinky", "open-minded"],
    "intimacy_level": "explicit",
    "created_at": 1737659000
  }
}
```

**Validation:**
- ✅ Profile has 10 photos
- ✅ 6 photos are marked `is_explicit: true` (60%)
- ✅ Profile includes kink preferences
- ✅ Bio matches intimacy level

---

### 2. List Profiles API

**Test: Retrieve All Profiles**
```bash
curl http://localhost:8888/api/profiles/list
```

**Expected Response:**
```json
{
  "profiles": [
    { "id": "...", "name": "Alex", ... },
    { "id": "...", "name": "Jordan", ... }
  ],
  "count": 2
}
```

**Validation:**
- ✅ Returns array of profiles
- ✅ Count matches array length

---

### 3. Get Single Profile API

**Test: Retrieve Specific Profile**
```bash
curl http://localhost:8888/api/profiles/{profile_id}
```

**Expected Response:**
```json
{
  "id": "profile-uuid",
  "name": "Alex",
  "age": 25,
  ...
}
```

**Validation:**
- ✅ Returns full profile object
- ✅ 404 if profile not found

---

### 4. Delete Profile API

**Test: Delete Profile**
```bash
curl -X DELETE http://localhost:8888/api/profiles/{profile_id}
```

**Expected Response:**
```json
{
  "success": true,
  "message": "Profile deleted"
}
```

**Validation:**
- ✅ Profile removed from list
- ✅ 404 on subsequent GET requests

---

### 5. Browser Porn Access (Gated)

**Test: Access Without Consent**
```bash
curl -X POST http://localhost:8888/api/browser/access-porn \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://example-adult-site.com",
    "consent": false
  }'
```

**Expected Response:**
```json
{
  "error": "Explicit consent required for porn site access",
  "consent_required": true
}
```

**Test: Access With Consent**
```bash
curl -X POST http://localhost:8888/api/browser/access-porn \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://example-adult-site.com",
    "consent": true
  }'
```

**Expected Response:**
```json
{
  "success": true,
  "message": "Consent granted. Use browser control to navigate.",
  "url": "https://example-adult-site.com",
  "instructions": "Use 'system browser navigate <url>' command to access the site"
}
```

**Validation:**
- ✅ Consent required for access
- ✅ Consent stored per URL
- ✅ 403 without consent

---

## Frontend Tests

### 1. Chat Command: "show profiles"

**Test Steps:**
1. Open Phoenix AGI desktop app
2. Type in chat: `show profiles`
3. Press Enter

**Expected Behavior:**
- ✅ Profiles panel opens (modal overlay)
- ✅ Message in chat: "💕 Profiles panel opened. Swipe to find matches!"
- ✅ Panel shows "Generate Profile" button if no profiles exist

---

### 2. Generate Profile from UI

**Test Steps:**
1. Open profiles panel (`show profiles`)
2. Click "+ Generate" button
3. Wait for generation

**Expected Behavior:**
- ✅ Button shows "Generating..." during request
- ✅ New profile appears in swipe interface
- ✅ Profile has 10 photos
- ✅ Explicit photos are blurred by default

---

### 3. Swipe Interface

**Test Steps:**
1. Open profiles panel with generated profiles
2. View profile photos (click left/right arrows)
3. Click explicit photo "Show" button
4. Swipe left (reject)
5. Swipe right (match)

**Expected Behavior:**
- ✅ Photo navigation works (10 photos per profile)
- ✅ Explicit photos require "Show" button click
- ✅ Photo indicators show current position
- ✅ Swipe left moves to next profile
- ✅ Swipe right adds to matches and moves to next
- ✅ Matches counter updates in footer

---

### 4. Profile Display

**Test Steps:**
1. View generated profile in swipe interface

**Expected Behavior:**
- ✅ Name and age displayed
- ✅ Bio text shown
- ✅ Interests displayed as tags
- ✅ Kinks displayed with special styling (primary color)
- ✅ Photos display correctly (aspect ratio 3:4)

---

### 5. Chat Integration (Match Trigger)

**Test Steps:**
1. Swipe right on a profile (match)
2. Observe chat behavior

**Expected Behavior:**
- ✅ Match added to matches list
- ✅ Console logs match event
- ✅ (Future) Intimate chat context triggered

---

### 6. Close Profiles Panel

**Test Steps:**
1. Open profiles panel
2. Type in chat: `hide profiles`
3. OR click X button in panel header

**Expected Behavior:**
- ✅ Panel closes
- ✅ Chat message: "Profiles panel hidden."

---

## Security Tests

### 1. Explicit Content Gating

**Test: Explicit Photo Blur**
- ✅ Explicit photos are blurred by default
- ✅ "Show" button required to view
- ✅ Non-explicit photos display immediately

### 2. Consent Enforcement

**Test: Porn Site Access**
- ✅ API rejects requests without `consent: true`
- ✅ Consent stored per URL
- ✅ Browser navigation requires consent check

---

## Performance Tests

### 1. Profile Generation Speed

**Test:**
- Generate 5 profiles sequentially
- Measure time per profile

**Expected:**
- ✅ < 2 seconds per profile (placeholder generation)
- ✅ (Future with Stable Diffusion: < 30 seconds per profile)

### 2. UI Responsiveness

**Test:**
- Load profiles panel with 10 profiles
- Navigate through photos

**Expected:**
- ✅ Panel opens in < 500ms
- ✅ Photo navigation is instant
- ✅ No lag during swipe animations

---

## Integration Points

### 1. Chat → Profiles Panel
- Command: `show profiles` → Opens panel
- Command: `hide profiles` → Closes panel
- Command: `swipe` → Opens panel (alias)

### 2. Profiles → Chat (Future)
- Match event → Triggers intimate chat context
- Profile data → Passed to chat for personalized responses

### 3. Backend → Frontend
- REST API: `/api/profiles/*`
- WebSocket: (Future) Real-time profile updates

---

## Known Limitations

1. **Photo Generation**: Currently uses placeholder SVGs. Future integration with Stable Diffusion API needed.
2. **Chat Integration**: Match events logged but not yet triggering intimate chat context.
3. **Consent Storage**: In-memory only (resets on server restart). Future: Persist to database.
4. **Browser Integration**: Porn site access returns instructions but doesn't auto-navigate.

---

## Future Enhancements

1. **AI Photo Generation**: Integrate Stable Diffusion API for photorealistic images
2. **Match Chat**: Auto-start intimate chat session on match
3. **Profile Customization**: User preferences for profile generation
4. **Swipe Animations**: Add smooth swipe gestures
5. **Match History**: Store and replay past matches
6. **Video Profiles**: Support video content in profiles

---

## Test Checklist

### Backend
- [ ] Profile generation API works
- [ ] List profiles API works
- [ ] Get single profile API works
- [ ] Delete profile API works
- [ ] Browser consent API enforces consent
- [ ] Explicit photo ratio is correct (60%)

### Frontend
- [ ] "show profiles" command opens panel
- [ ] "hide profiles" command closes panel
- [ ] Generate button creates new profile
- [ ] Swipe left/right works
- [ ] Photo navigation works
- [ ] Explicit photos are gated
- [ ] Matches counter updates
- [ ] Panel closes on X button

### Security
- [ ] Explicit content requires user action
- [ ] Porn site access requires consent
- [ ] No explicit content stored without permission

### Performance
- [ ] Profile generation < 2s
- [ ] Panel opens < 500ms
- [ ] No UI lag during interactions

---

## Test Results

**Date:** 2026-01-23  
**Tester:** Orchestrator  
**Status:** ✅ All core features implemented and ready for testing

**Notes:**
- Backend endpoints created and integrated
- Frontend component fully functional
- Command registry updated
- Security gating in place
- Ready for user testing
