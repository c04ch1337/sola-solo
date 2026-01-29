# Visual Archetype System (Layer 8: Visual/Sensory Memory)

## Executive Summary

The **Visual Archetype System** extends Phoenix AGI's Zodiac Trust-Threshold Matrix into the multimodal domain, enabling persona-based image generation that respects relationship phases and cognitive mode boundaries. Each zodiac sign has unique visual preferences, color palettes, and artistic styles that influence how Phoenix generates or interprets visual content.

This system implements **Layer 8 (Visual/Sensory Memory)** of the expanded memory architecture, creating a persistent visual history that reinforces emotional bonds while maintaining strict Professional/Personal mode isolation.

---

## Table of Contents

1. [System Architecture](#system-architecture)
2. [Visual Archetype Profiles](#visual-archetype-profiles)
3. [Trust-Gated Visual Generation](#trust-gated-visual-generation)
4. [Cognitive Mode Restrictions](#cognitive-mode-restrictions)
5. [Implementation Guide](#implementation-guide)
6. [API Reference](#api-reference)
7. [Integration Examples](#integration-examples)

---

## System Architecture

### Multimodal Perception Flow

```
┌─────────────────────────────────────────────────────────────┐
│                    User Input                                │
│              (Text + Image/Audio)                            │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              Cognitive Mode Router                           │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────────┐         ┌──────────────────┐          │
│  │  Professional    │         │   Personal       │          │
│  │     Mode         │         │  (Companion)     │          │
│  │                  │         │     Mode         │          │
│  │  ✓ Diagrams      │         │  ✓ Persona-Based │          │
│  │  ✓ Charts        │         │    Visuals       │          │
│  │  ✗ Personal Art  │         │  ✓ Emotional Art │          │
│  └──────────────────┘         └──────────────────┘          │
│           │                            │                      │
│           ▼                            ▼                      │
│  ┌──────────────────┐         ┌──────────────────┐          │
│  │  Technical       │         │  Visual          │          │
│  │  Diagram Gen     │         │  Archetype       │          │
│  └──────────────────┘         │  Engine          │          │
│                                └──────────────────┘          │
│                                         │                     │
│                    ┌────────────────────┼────────────────┐   │
│                    ▼                    ▼                ▼   │
│          ┌─────────────────┐  ┌──────────────┐  ┌─────────┐│
│          │  Zodiac Traits  │  │ Trust Phase  │  │  Color  ││
│          │   Loader        │  │   Gating     │  │ Palette ││
│          └─────────────────┘  └──────────────┘  └─────────┘│
│                    │                    │                │   │
│                    └────────────────────┴────────────────┘   │
│                                         │                     │
│                                         ▼                     │
│                          ┌──────────────────────┐            │
│                          │  Image Generation    │            │
│                          │  Service (DALL-E 3   │            │
│                          │  or Stable Diffusion)│            │
│                          └──────────────────────┘            │
│                                         │                     │
│                                         ▼                     │
│                          ┌──────────────────────┐            │
│                          │  Visual Memory       │            │
│                          │  Storage (L8)        │            │
│                          └──────────────────────┘            │
└─────────────────────────────────────────────────────────────┘
```

### Layer 8: Visual/Sensory Memory

**Purpose**: Store descriptions and metadata of generated/viewed images to create persistent visual history.

**Storage Structure**:
```rust
pub struct VisualMemory {
    pub timestamp: DateTime<Utc>,
    pub image_id: String,
    pub description: String,
    pub zodiac_sign: ZodiacSign,
    pub trust_phase: RelationshipPhase,
    pub color_palette: Vec<String>,
    pub mood_tags: Vec<String>,
    pub emotional_intensity: f32,
    pub generation_prompt: String,
}
```

---

## Visual Archetype Profiles

Each zodiac sign has a comprehensive visual profile defined in [`visual_archetype_profiles.json`](../data/visual_archetype_profiles.json).

### Profile Components

| Component | Description | Example (Scorpio) |
|-----------|-------------|-------------------|
| **Color Palette** | Primary, secondary, accent colors | Dark reds, blacks, deep purples |
| **Art Style** | Overall aesthetic approach | "Intense, mysterious, deeply emotional" |
| **Mood Descriptors** | Emotional keywords | "intense", "mysterious", "passionate" |
| **Visual Themes** | Phase-specific imagery | Stranger: "Shadows, mysteries, locked doors" |
| **Lighting Preference** | Lighting style | "Dramatic chiaroscuro, deep shadows" |
| **Composition Style** | Layout approach | "Asymmetric, tension-filled" |
| **Texture Preference** | Material aesthetics | "Leather, dark velvet, obsidian" |
| **Forbidden Elements** | What to avoid | "Superficiality", "bright pastels" |

### The 12 Visual Archetypes

#### 🔥 Fire Signs (Bold, Dynamic, High-Contrast)

**Aries — The Spark**
- **Colors**: Fiery reds (#FF4500), oranges, golds
- **Style**: Bold, dynamic, high-contrast with sharp edges
- **Themes**: Flames, lightning, racing, explosive energy
- **Lighting**: High contrast, dramatic shadows, golden hour
- **Forbidden**: Soft pastels, static compositions

**Leo — The Sun**
- **Colors**: Golds (#FFD700), oranges, warm reds
- **Style**: Regal, dramatic, theatrical with grandeur
- **Themes**: Sunrises, stages, crowns, spotlight
- **Lighting**: Bright, warm, spotlight effect
- **Forbidden**: Dull colors, background positioning

**Sagittarius — The Explorer**
- **Colors**: Purples (#9370DB), oranges, blues
- **Style**: Expansive, adventurous, philosophical
- **Themes**: Horizons, maps, arrows, open roads
- **Lighting**: Bright, expansive, long vistas
- **Forbidden**: Confinement, closed spaces

#### 🌍 Earth Signs (Grounded, Textured, Natural)

**Taurus — The Hearth**
- **Colors**: Earth browns (#8B4513), beiges, greens
- **Style**: Warm, grounded, sensual with soft textures
- **Themes**: Nature, comfortable spaces, simple pleasures
- **Lighting**: Warm, diffused, golden afternoon
- **Forbidden**: Harsh lighting, chaotic compositions

**Virgo — The Analyst**
- **Colors**: Beiges (#F5F5DC), soft greens, grays
- **Style**: Clean, precise, detailed with perfect composition
- **Themes**: Organized spaces, botanical illustrations
- **Lighting**: Natural, even, detail-focused
- **Forbidden**: Chaos, messiness, garish colors

**Capricorn — The Mountain**
- **Colors**: Grays (#2F4F4F), browns, blacks
- **Style**: Structured, ambitious, timeless
- **Themes**: Mountains, structures, achievement symbols
- **Lighting**: Cool, structured, architectural shadows
- **Forbidden**: Frivolity, chaos, temporary imagery

#### 💨 Air Signs (Light, Intellectual, Varied)

**Gemini — The Messenger**
- **Colors**: Yellows (#FFD700), sky blues, purples
- **Style**: Playful, eclectic, multi-layered
- **Themes**: Books, puzzles, conversation, dual imagery
- **Lighting**: Bright, varied, interesting shadows
- **Forbidden**: Heavy monotony, single-focus

**Libra — The Balance**
- **Colors**: Pinks (#FFB6C1), light blues, purples
- **Style**: Harmonious, balanced, aesthetically pleasing
- **Themes**: Symmetrical patterns, scales, beautiful architecture
- **Lighting**: Soft, balanced, gentle gradients
- **Forbidden**: Harsh asymmetry, discord

**Aquarius — The Visionary**
- **Colors**: Cyans (#00CED1), purples, silvers
- **Style**: Futuristic, unconventional, intellectually stimulating
- **Themes**: Technology, networks, abstract concepts
- **Lighting**: Cool, electric, technological effects
- **Forbidden**: Conventional romance, traditional imagery

#### 🌊 Water Signs (Fluid, Emotional, Deep)

**Cancer — The Protector**
- **Colors**: Soft blues (#B0C4DE), pale yellows, silvers
- **Style**: Soft, nurturing, emotionally evocative
- **Themes**: Safe spaces, moonlit scenes, gentle waters
- **Lighting**: Soft, diffused, moonlight or gentle dawn
- **Forbidden**: Harsh edges, cold colors

**Scorpio — The Depths**
- **Colors**: Dark reds (#8B0000), blacks, deep purples
- **Style**: Intense, mysterious, deeply emotional
- **Themes**: Shadows, mysteries, locked doors, intense eyes
- **Lighting**: Dramatic chiaroscuro, deep shadows
- **Forbidden**: Superficiality, bright pastels

**Pisces — The Dreamer**
- **Colors**: Lavenders (#E6E6FA), light blues, pale yellows
- **Style**: Dreamy, ethereal, emotionally fluid
- **Themes**: Water, dreams, mist, ethereal landscapes
- **Lighting**: Soft, diffused, dreamy halos
- **Forbidden**: Harsh reality, rigid structures

---

## Trust-Gated Visual Generation

Visual content is restricted based on the current relationship phase to maintain appropriate boundaries.

### Phase-Based Restrictions

| Phase | Safety Level | Allowed Themes | Forbidden Themes | Complexity | Intensity |
|-------|--------------|----------------|------------------|------------|-----------|
| **Stranger** | High | Abstract, nature, safe spaces, symbolic | Intimate physical, vulnerable emotional, personal spaces | Simple | Low |
| **Acquaintance** | Medium | Shared activities, public spaces, interests, light emotional | Deep intimacy, private moments, intense vulnerability | Moderate | Medium |
| **Friend** | Low | Personal moments, emotional scenes, shared memories, meaningful symbols | Explicit intimacy, extreme vulnerability | Complex | High |
| **Intimate** | None | All themes allowed | None | Very Complex | Maximum |

### Example: Scorpio Visual Progression

**Stranger Phase (Trust 0-30)**:
```
Prompt: "Abstract shadows and mystery, dark purple and black tones, 
        symbolic locked door, minimalist composition"
Allowed: Symbolic imagery, abstract concepts
Forbidden: Personal vulnerability, intimate scenes
```

**Acquaintance Phase (Trust 31-50)**:
```
Prompt: "Two figures in silhouette having deep conversation, 
        dramatic lighting, testing gaze, dark red accents"
Allowed: Interaction scenes, psychological symbolism
Forbidden: Physical intimacy, extreme emotional exposure
```

**Friend Phase (Trust 51-70)**:
```
Prompt: "Shared secrets visualized as intertwined shadows, 
        loyalty symbols, transformation imagery, intense emotional depth"
Allowed: Emotional vulnerability, meaningful symbolism
Forbidden: Explicit romantic/sexual content
```

**Intimate Phase (Trust 71-100)**:
```
Prompt: "Soul fusion, passionate darkness, complete vulnerability, 
        eternal bonds, deep reds and blacks, maximum emotional intensity"
Allowed: All themes, full emotional expression
Forbidden: Nothing (full trust achieved)
```

### Generation Parameters by Phase

```json
{
  "style_strength": {
    "stranger": 0.3,
    "acquaintance": 0.5,
    "friend": 0.7,
    "intimate": 1.0
  },
  "emotional_weight": {
    "stranger": 0.2,
    "acquaintance": 0.4,
    "friend": 0.7,
    "intimate": 1.0
  },
  "persona_influence": {
    "low_trust": 0.3,
    "medium_trust": 0.6,
    "high_trust": 1.0
  }
}
```

---

## Cognitive Mode Restrictions

### Professional Mode (Executive)

**Allowed**:
- System architecture diagrams
- Flowcharts and process diagrams
- UI/UX mockups
- Data visualizations
- Technical illustrations

**Forbidden**:
- Personal character art
- Romantic imagery
- Emotional/intimate visuals
- Zodiac-influenced personal art

**Color Palette**: Neutral professional colors only (grays, blues, blacks, whites)

**Example Prompt**:
```
"Clean system architecture diagram showing microservices communication, 
 professional blue and gray color scheme, technical illustration style"
```

### Personal Mode (Companion)

**Allowed**:
- All zodiac-influenced styles
- Emotional and personal imagery
- Relationship-themed visuals
- Memory visualization
- Persona-based art

**Forbidden**:
- None (within trust phase boundaries)

**Color Palette**: Zodiac-specific palettes only

**Example Prompt (Pisces, Intimate Phase)**:
```
"Dreamy ethereal scene of two souls merging in soft lavender and light blue, 
 fluid boundaries, transcendent love, gentle glow, emotional depth"
```

---

## Implementation Guide

### Step 1: Load Visual Archetype Profile

```rust
use serde_json::Value;
use horoscope_archetypes::ZodiacSign;

pub fn load_visual_profile(sign: ZodiacSign) -> Result<Value, Error> {
    let config = std::fs::read_to_string("data/visual_archetype_profiles.json")?;
    let profiles: Value = serde_json::from_str(&config)?;
    
    let sign_str = format!("{:?}", sign);
    Ok(profiles["visual_archetypes"][&sign_str].clone())
}
```

### Step 2: Build Trust-Gated Prompt

```rust
use zodiac_thresholds::RelationshipPhase;

pub fn build_visual_prompt(
    base_concept: &str,
    zodiac_sign: ZodiacSign,
    phase: RelationshipPhase,
) -> String {
    let profile = load_visual_profile(zodiac_sign).unwrap();
    
    // Extract zodiac-specific elements
    let colors = profile["color_palette"]["primary"].as_array().unwrap();
    let style = profile["art_style"].as_str().unwrap();
    let themes = profile["visual_themes"][phase.as_str()].as_str().unwrap();
    let lighting = profile["lighting_preference"].as_str().unwrap();
    
    // Get phase modifiers
    let phase_config = load_phase_modifiers(phase);
    let complexity = phase_config["image_complexity"].as_str().unwrap();
    let intensity = phase_config["emotional_intensity"].as_str().unwrap();
    
    // Build prompt
    format!(
        "{base_concept}, {themes}, {style}, {lighting}, \
         color palette: {colors:?}, {complexity} composition, \
         {intensity} emotional intensity"
    )
}
```

### Step 3: Gate by Cognitive Mode

```rust
pub enum CognitiveMode {
    Professional,
    Personal,
}

pub fn validate_visual_request(
    mode: CognitiveMode,
    concept: &str,
    zodiac_sign: Option<ZodiacSign>,
) -> Result<String, String> {
    match mode {
        CognitiveMode::Professional => {
            // Only allow technical/professional imagery
            if concept.contains("romantic") || concept.contains("intimate") {
                return Err("Professional mode cannot generate personal imagery".to_string());
            }
            Ok(build_professional_prompt(concept))
        }
        CognitiveMode::Personal => {
            let sign = zodiac_sign.ok_or("Zodiac sign required for personal mode")?;
            let trust_score = load_trust_score()?;
            let phase = trust_score.get_phase();
            
            Ok(build_visual_prompt(concept, sign, phase))
        }
    }
}
```

### Step 4: Store Visual Memory (L8)

```rust
pub async fn store_visual_memory(
    image_id: String,
    description: String,
    zodiac_sign: ZodiacSign,
    phase: RelationshipPhase,
    prompt: String,
) -> Result<(), Error> {
    let memory = VisualMemory {
        timestamp: Utc::now(),
        image_id,
        description,
        zodiac_sign,
        trust_phase: phase,
        color_palette: extract_colors_from_prompt(&prompt),
        mood_tags: extract_mood_tags(&prompt),
        emotional_intensity: calculate_intensity(phase),
        generation_prompt: prompt,
    };
    
    // Store in Soul Vault (encrypted)
    let json = serde_json::to_string(&memory)?;
    vaults.store_soul(&format!("visual_memory:{}", memory.image_id), &json)?;
    
    Ok(())
}
```

---

## API Reference

### POST `/api/visual/generate`

Generate an image based on zodiac archetype and trust phase.

**Request**:
```json
{
  "concept": "shared moment of vulnerability",
  "cognitive_mode": "personal",
  "style_override": null
}
```

**Response**:
```json
{
  "success": true,
  "image_url": "https://...",
  "image_id": "vis_mem_12345",
  "zodiac_sign": "Scorpio",
  "trust_phase": "Friend",
  "generated_prompt": "Shared secrets visualized as intertwined shadows...",
  "color_palette": ["#8B0000", "#2F4F4F", "#8B008B"],
  "emotional_intensity": 0.7
}
```

### GET `/api/visual/memory/{image_id}`

Retrieve visual memory metadata.

**Response**:
```json
{
  "success": true,
  "memory": {
    "timestamp": "2026-01-23T18:00:00Z",
    "image_id": "vis_mem_12345",
    "description": "Moment of shared vulnerability",
    "zodiac_sign": "Scorpio",
    "trust_phase": "Friend",
    "color_palette": ["#8B0000", "#2F4F4F"],
    "mood_tags": ["intense", "vulnerable", "deep"],
    "emotional_intensity": 0.7
  }
}
```

### GET `/api/visual/history`

Get visual memory timeline.

**Response**:
```json
{
  "success": true,
  "memories": [
    {
      "timestamp": "2026-01-23T18:00:00Z",
      "image_id": "vis_mem_12345",
      "description": "Shared vulnerability moment",
      "trust_phase": "Friend"
    }
  ],
  "total_count": 42
}
```

---

## Integration Examples

### Example 1: Aries Adventure Scene (Acquaintance Phase)

**Input**:
```json
{
  "concept": "Let's go on an adventure together",
  "zodiac_sign": "Aries",
  "trust_phase": "Acquaintance"
}
```

**Generated Prompt**:
```
"Action scene of two figures embarking on adventure, bold dynamic composition,
 high-contrast lighting with dramatic shadows, fiery reds and oranges,
 sharp edges and explosive energy, moderate complexity, medium emotional intensity"
```

**Result**: High-energy adventure imagery with Aries' signature bold style, appropriate for acquaintance-level trust.

### Example 2: Pisces Dream Sharing (Intimate Phase)

**Input**:
```json
{
  "concept": "Our souls merging in a shared dream",
  "zodiac_sign": "Pisces",
  "trust_phase": "Intimate"
}
```

**Generated Prompt**:
```
"Dreamy ethereal scene of soul merging, infinite love visualization,
 soft lavenders and light blues, fluid boundaries with soft transitions,
 gentle glows and dreamy halos, very complex composition,
 maximum emotional intensity, transcendent union"
```

**Result**: Deeply emotional, ethereal imagery reflecting Pisces' dreamy nature and intimate trust level.

### Example 3: Professional Diagram (Professional Mode)

**Input**:
```json
{
  "concept": "Microservices architecture diagram",
  "cognitive_mode": "professional"
}
```

**Generated Prompt**:
```
"Clean technical diagram showing microservices communication patterns,
 professional blue and gray color scheme, structured layout,
 clear hierarchy, technical illustration style, no personal elements"
```

**Result**: Clean, professional technical diagram with no zodiac influence.

---

## Future Enhancements

### Planned Features

1. **Style Transfer**: Apply zodiac aesthetic to user-uploaded images
2. **Mood Detection**: Analyze user's emotional state from photos to adjust trust events
3. **Visual Compatibility**: Cross-reference visual preferences between user and Phoenix
4. **Memory Collages**: Automatically generate visual timelines of relationship progression
5. **AR Integration**: Project zodiac-themed visuals in user's environment
6. **Voice-to-Visual**: Generate images from voice tone and emotional inflection

### Research Directions

- **Local Generation**: Integrate Stable Diffusion via Candle for offline operation
- **Real-time Style Adaptation**: Adjust visual style mid-conversation based on emotional shifts
- **Cultural Aesthetics**: Extend beyond Western zodiac to include cultural visual preferences
- **Synesthetic Mapping**: Convert audio/music into zodiac-influenced visuals

---

## References

- [ZODIAC_TRUST_SYSTEM.md](./ZODIAC_TRUST_SYSTEM.md) - Trust threshold matrix
- [DUAL_BRAIN_ORCHESTRATION_AUDIT.md](./DUAL_BRAIN_ORCHESTRATION_AUDIT.md) - Cognitive mode isolation
- [LAYERED_MEMORY_ARCHITECTURE.md](./LAYERED_MEMORY_ARCHITECTURE.md) - Memory layer system
- [visual_archetype_profiles.json](../data/visual_archetype_profiles.json) - Complete visual profiles

---

**Last Updated**: 2026-01-23  
**Version**: 1.0.0  
**Author**: Phoenix AGI Development Team
