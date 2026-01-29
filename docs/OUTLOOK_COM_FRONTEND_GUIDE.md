# Outlook COM Frontend Customization Guide

**For Frontend Developers - Windows Outlook Integration**

This guide provides comprehensive information about all settings, configuration options, and customization capabilities available for the Outlook COM module in the frontend.

---

## Table of Contents

1. [Current Settings & Configuration](#current-settings--configuration)
2. [Frontend Customization Options](#frontend-customization-options)
3. [UI/UX Settings & Preferences](#uiux-settings--preferences)
4. [Recommended Settings to Add](#recommended-settings-to-add)
5. [API Endpoint Reference](#api-endpoint-reference)
6. [Frontend Implementation Examples](#frontend-implementation-examples)
7. [Settings UI Components](#settings-ui-components)
8. [Best Practices](#best-practices)

---

## Current Settings & Configuration

### Environment-Based Settings

These settings are configured via environment variables (`.env` file) and cannot be changed from the frontend:

#### `OUTLOOK_COM_ENABLED`
- **Type**: Boolean
- **Default**: `false`
- **Description**: Master switch to enable/disable Outlook COM integration
- **Frontend Access**: Read-only via `/api/outlook/status`
- **UI Display**: Show enabled/disabled status badge

```typescript
interface OutlookStatus {
  enabled: boolean;      // From OUTLOOK_COM_ENABLED env var
  available: boolean;   // Whether Outlook is actually available
  platform: "windows" | "not_windows";
  message?: string;     // Optional status message
}
```

---

## Frontend Customization Options

### 1. Email Display Settings

These can be configured in the frontend UI and stored in browser localStorage or sent to backend:

#### Email List View Preferences

```typescript
interface EmailDisplaySettings {
  // View preferences
  defaultFolder: string;           // "Inbox" | "Sent Items" | "Drafts" | custom
  itemsPerPage: number;            // 10 | 25 | 50 | 100
  sortBy: "date" | "subject" | "from" | "importance";
  sortOrder: "asc" | "desc";
  
  // Display options
  showPreview: boolean;            // Show email body preview
  previewLength: number;            // Characters in preview (default: 200)
  showAttachments: boolean;         // Show attachment indicators
  showCategories: boolean;          // Show category tags
  showImportance: boolean;          // Show importance indicators
  
  // Filtering
  defaultFilter: "all" | "unread" | "read" | "flagged";
  showOnlyWithAttachments: boolean;
  
  // Date range
  defaultDateRange: "today" | "week" | "month" | "all";
}
```

**Implementation Example:**
```typescript
// Store in localStorage
const defaultEmailSettings: EmailDisplaySettings = {
  defaultFolder: "Inbox",
  itemsPerPage: 25,
  sortBy: "date",
  sortOrder: "desc",
  showPreview: true,
  previewLength: 200,
  showAttachments: true,
  showCategories: true,
  showImportance: true,
  defaultFilter: "all",
  showOnlyWithAttachments: false,
  defaultDateRange: "week"
};

localStorage.setItem('outlook_email_settings', JSON.stringify(defaultEmailSettings));
```

#### Email Detail View Settings

```typescript
interface EmailDetailSettings {
  showHtmlBody: boolean;           // Prefer HTML over plain text
  showHeaders: boolean;              // Show full email headers
  autoLoadAttachments: boolean;     // Auto-load attachment metadata
  showRelatedEmails: boolean;       // Show conversation thread
  markAsReadOnOpen: boolean;        // Auto-mark as read when opened
}
```

---

### 2. Email Composition Settings

```typescript
interface EmailCompositionSettings {
  // Default values
  defaultFromAccount?: string;      // If multiple accounts
  defaultSignature: string;         // Auto-append signature
  useHtmlEditor: boolean;            // Use rich text editor
  
  // Auto-save
  autoSaveDrafts: boolean;          // Auto-save to Drafts folder
  autoSaveInterval: number;         // Seconds (default: 30)
  
  // Sending options
  defaultImportance: "Low" | "Normal" | "High";
  requestReadReceipt: boolean;       // Request read receipt by default
  requestDeliveryReceipt: boolean;   // Request delivery receipt
  
  // Attachments
  maxAttachmentSize: number;        // MB (default: 25)
  allowedAttachmentTypes: string[]; // File extensions
  
  // Security
  warnOnExternalRecipients: boolean; // Warn if recipient not in contacts
  encryptByDefault: boolean;         // Encrypt emails by default (if available)
}
```

---

### 3. Calendar View Settings

```typescript
interface CalendarViewSettings {
  // View type
  defaultView: "day" | "week" | "month" | "agenda";
  
  // Date range
  defaultStartDate?: string;        // ISO 8601 date
  defaultEndDate?: string;           // ISO 8601 date
  defaultRange: "today" | "week" | "month" | "custom";
  
  // Display options
  showAllDayEvents: boolean;
  showReminders: boolean;
  showAttendees: boolean;
  showLocation: boolean;
  showBody: boolean;
  
  // Time settings
  startHour: number;                // 0-23 (default: 8)
  endHour: number;                  // 0-23 (default: 18)
  timeSlotDuration: number;         // Minutes (15 | 30 | 60)
  
  // Colors
  eventColorScheme: "default" | "category" | "importance";
}
```

---

### 4. Contacts View Settings

```typescript
interface ContactsViewSettings {
  // View type
  viewMode: "list" | "grid" | "card";
  
  // Display options
  showCompany: boolean;
  showJobTitle: boolean;
  showPhoneNumbers: boolean;
  showEmailAddresses: boolean;
  
  // Sorting
  sortBy: "name" | "company" | "email";
  sortOrder: "asc" | "desc";
  
  // Filtering
  showOnlyWithEmail: boolean;
  showOnlyWithPhone: boolean;
  
  // Grouping
  groupBy: "none" | "company" | "firstLetter";
}
```

---

### 5. Notification & Alert Settings

```typescript
interface OutlookNotificationSettings {
  // Email notifications
  notifyOnNewEmail: boolean;
  notifyOnUnreadCount: boolean;
  unreadThreshold: number;          // Notify when unread count exceeds this
  
  // Calendar notifications
  notifyOnUpcomingEvents: boolean;
  reminderMinutes: number[];        // [15, 30, 60] - minutes before event
  
  // Sound settings
  playSoundOnNewEmail: boolean;
  soundFile?: string;               // Path to sound file
  
  // Visual indicators
  showUnreadBadge: boolean;
  badgeColor: string;               // Hex color
  showDesktopNotification: boolean;  // System notifications
}
```

---

### 6. Folder Management Settings

```typescript
interface FolderManagementSettings {
  // Default folders to show
  visibleFolders: string[];         // ["Inbox", "Sent Items", "Drafts"]
  
  // Folder tree
  expandAllFolders: boolean;
  showFolderCounts: boolean;        // Show unread count badges
  showSubfolders: boolean;
  
  // Custom folders
  favoriteFolders: string[];        // Pinned/favorite folders
  hiddenFolders: string[];          // Folders to hide from view
}
```

---

## UI/UX Settings & Preferences

### Theme & Appearance

```typescript
interface OutlookUITheme {
  // Color scheme
  theme: "light" | "dark" | "auto";
  primaryColor: string;             // Hex color
  accentColor: string;              // Hex color
  
  // Layout
  sidebarWidth: number;             // Pixels
  sidebarPosition: "left" | "right";
  compactMode: boolean;             // Dense layout
  
  // Typography
  fontSize: "small" | "medium" | "large";
  fontFamily: string;
  
  // Spacing
  itemSpacing: "compact" | "comfortable" | "spacious";
}
```

### Interaction Settings

```typescript
interface OutlookInteractionSettings {
  // Click behavior
  emailClickAction: "open" | "preview" | "markRead";
  doubleClickAction: "open" | "reply";
  
  // Keyboard shortcuts
  enableKeyboardShortcuts: boolean;
  customShortcuts: Record<string, string>; // key -> action mapping
  
  // Drag & drop
  enableDragDrop: boolean;
  dragDropToFolder: boolean;       // Drag emails to folders
  
  // Auto-refresh
  autoRefresh: boolean;
  refreshInterval: number;          // Seconds (default: 60)
}
```

---

## Recommended Settings to Add

### 1. Email Filtering & Search Settings

```typescript
interface EmailFilterSettings {
  // Saved filters
  savedFilters: Array<{
    name: string;
    criteria: {
      folder?: string;
      from?: string;
      subject?: string;
      hasAttachments?: boolean;
      dateRange?: { start: string; end: string };
      importance?: "Low" | "Normal" | "High";
      isRead?: boolean;
    };
  }>;
  
  // Quick filters
  quickFilters: string[];          // ["Unread", "Flagged", "Today"]
  
  // Search settings
  searchInBody: boolean;
  searchInAttachments: boolean;
  caseSensitive: boolean;
}
```

### 2. Email Template Settings

```typescript
interface EmailTemplateSettings {
  templates: Array<{
    name: string;
    subject: string;
    body: string;
    htmlBody?: string;
    defaultTo?: string[];
    defaultCc?: string[];
    defaultBcc?: string[];
  }>;
  
  // Auto-complete
  enableAutoComplete: boolean;
  autoCompleteDelay: number;       // Milliseconds
}
```

### 3. Calendar Integration Settings

```typescript
interface CalendarIntegrationSettings {
  // Sync settings
  syncInterval: number;             // Minutes
  syncOnStartup: boolean;
  
  // Event creation defaults
  defaultDuration: number;          // Minutes (default: 60)
  defaultReminder: number;           // Minutes before (default: 15)
  defaultLocation?: string;
  
  // Working hours
  workingHoursStart: string;        // "09:00"
  workingHoursEnd: string;          // "17:00"
  workingDays: number[];            // [1,2,3,4,5] = Mon-Fri
}
```

### 4. Privacy & Security Settings

```typescript
interface OutlookPrivacySettings {
  // Data handling
  cacheEmails: boolean;             // Cache emails locally
  cacheContacts: boolean;            // Cache contacts locally
  cacheCalendar: boolean;             // Cache calendar locally
  cacheExpiration: number;           // Days
  
  // Privacy
  hideEmailAddresses: boolean;       // Mask email addresses in UI
  hideContactDetails: boolean;        // Hide contact phone numbers
  logAccess: boolean;                // Log all Outlook access
  
  // Security
  requireConfirmationForSend: boolean;
  requireConfirmationForDelete: boolean;
  maxRecipients: number;             // Limit recipients per email
}
```

### 5. Performance Settings

```typescript
interface OutlookPerformanceSettings {
  // Loading
  lazyLoadEmails: boolean;          // Load emails on demand
  preloadCount: number;              // Emails to preload (default: 50)
  loadAttachmentsOnDemand: boolean;
  
  // Caching
  enableCache: boolean;
  cacheSize: number;                 // MB
  cacheStrategy: "aggressive" | "moderate" | "minimal";
  
  // Background sync
  backgroundSync: boolean;
  syncInterval: number;              // Minutes
}
```

---

## API Endpoint Reference

### Status & Configuration

#### `GET /api/outlook/status`
Get Outlook COM status and availability.

**Response:**
```typescript
interface OutlookStatusResponse {
  enabled: boolean;                 // From OUTLOOK_COM_ENABLED
  available: boolean;               // Whether Outlook is accessible
  platform: "windows" | "not_windows";
  message?: string;                 // Optional status message
}
```

**Usage:**
```typescript
async function checkOutlookStatus(): Promise<OutlookStatusResponse> {
  const response = await fetch('/api/outlook/status');
  return response.json();
}
```

### Email Operations

#### `GET /api/outlook/emails?folder={name}&max_count={n}`
Get emails from a folder.

**Query Parameters:**
- `folder` (optional): Folder name (default: "Inbox")
- `max_count` (optional): Maximum emails to return (default: 50)

**Response:**
```typescript
interface OutlookEmail {
  entry_id: string;
  subject: string;
  from: string;
  to: string;
  cc?: string;
  bcc?: string;
  body: string;
  body_html?: string;
  received_time?: string;
  sent_time?: string;
  importance: "Low" | "Normal" | "High";
  is_read: boolean;
  has_attachments: boolean;
  attachments: OutlookAttachment[];
  categories: string[];
}
```

#### `POST /api/outlook/send`
Send an email.

**Request:**
```typescript
interface SendEmailRequest {
  to: string;
  subject: string;
  body: string;
  html_body?: string;
  cc?: string;
  bcc?: string;
  attachments?: string[];            // File paths
}
```

### Calendar Operations

#### `GET /api/outlook/appointments?start_date={date}&end_date={date}`
Get calendar appointments.

**Query Parameters:**
- `start_date` (optional): ISO 8601 date string
- `end_date` (optional): ISO 8601 date string

#### `POST /api/outlook/calendar/create-event`
Create a calendar event.

**Request:**
```typescript
interface CreateAppointmentRequest {
  subject: string;
  start_time: string;               // ISO 8601
  end_time: string;                 // ISO 8601
  location?: string;
  body?: string;
  required_attendees?: string[];
  optional_attendees?: string[];
  reminder_minutes?: number;
}
```

---

## Frontend Implementation Examples

### 1. Settings Manager

```typescript
class OutlookSettingsManager {
  private storageKey = 'outlook_settings';
  
  // Load all settings
  loadSettings(): OutlookAllSettings {
    const stored = localStorage.getItem(this.storageKey);
    if (stored) {
      return JSON.parse(stored);
    }
    return this.getDefaultSettings();
  }
  
  // Save settings
  saveSettings(settings: Partial<OutlookAllSettings>): void {
    const current = this.loadSettings();
    const updated = { ...current, ...settings };
    localStorage.setItem(this.storageKey, JSON.stringify(updated));
  }
  
  // Get default settings
  getDefaultSettings(): OutlookAllSettings {
    return {
      email: {
        defaultFolder: "Inbox",
        itemsPerPage: 25,
        sortBy: "date",
        sortOrder: "desc",
        showPreview: true,
        previewLength: 200,
        showAttachments: true,
        showCategories: true,
        showImportance: true,
        defaultFilter: "all",
        showOnlyWithAttachments: false,
        defaultDateRange: "week"
      },
      composition: {
        defaultSignature: "",
        useHtmlEditor: true,
        autoSaveDrafts: true,
        autoSaveInterval: 30,
        defaultImportance: "Normal",
        requestReadReceipt: false,
        requestDeliveryReceipt: false,
        maxAttachmentSize: 25,
        allowedAttachmentTypes: [".pdf", ".doc", ".docx", ".xls", ".xlsx"],
        warnOnExternalRecipients: true,
        encryptByDefault: false
      },
      calendar: {
        defaultView: "week",
        defaultRange: "week",
        showAllDayEvents: true,
        showReminders: true,
        showAttendees: true,
        showLocation: true,
        showBody: false,
        startHour: 8,
        endHour: 18,
        timeSlotDuration: 30,
        eventColorScheme: "default"
      },
      contacts: {
        viewMode: "list",
        showCompany: true,
        showJobTitle: true,
        showPhoneNumbers: true,
        showEmailAddresses: true,
        sortBy: "name",
        sortOrder: "asc",
        showOnlyWithEmail: false,
        showOnlyWithPhone: false,
        groupBy: "none"
      },
      notifications: {
        notifyOnNewEmail: true,
        notifyOnUnreadCount: true,
        unreadThreshold: 5,
        notifyOnUpcomingEvents: true,
        reminderMinutes: [15, 30, 60],
        playSoundOnNewEmail: false,
        showUnreadBadge: true,
        badgeColor: "#FF0000",
        showDesktopNotification: true
      },
      theme: {
        theme: "auto",
        primaryColor: "#0078D4",
        accentColor: "#106EBE",
        sidebarWidth: 250,
        sidebarPosition: "left",
        compactMode: false,
        fontSize: "medium",
        fontFamily: "system-ui",
        itemSpacing: "comfortable"
      },
      interaction: {
        emailClickAction: "preview",
        doubleClickAction: "open",
        enableKeyboardShortcuts: true,
        customShortcuts: {},
        enableDragDrop: true,
        dragDropToFolder: true,
        autoRefresh: true,
        refreshInterval: 60
      }
    };
  }
  
  // Reset to defaults
  resetSettings(): void {
    localStorage.removeItem(this.storageKey);
  }
}

// Usage
const settingsManager = new OutlookSettingsManager();
const settings = settingsManager.loadSettings();

// Update email settings
settingsManager.saveSettings({
  email: {
    ...settings.email,
    itemsPerPage: 50
  }
});
```

### 2. Settings UI Component (React Example)

```typescript
import React, { useState, useEffect } from 'react';

interface OutlookSettingsPanelProps {
  onClose: () => void;
}

export const OutlookSettingsPanel: React.FC<OutlookSettingsPanelProps> = ({ onClose }) => {
  const [settings, setSettings] = useState(settingsManager.loadSettings());
  const [activeTab, setActiveTab] = useState<'email' | 'calendar' | 'contacts' | 'notifications' | 'theme'>('email');
  
  const handleSave = () => {
    settingsManager.saveSettings(settings);
    onClose();
  };
  
  return (
    <div className="outlook-settings-panel">
      <div className="settings-header">
        <h2>Outlook Settings</h2>
        <button onClick={onClose}>×</button>
      </div>
      
      <div className="settings-tabs">
        <button 
          className={activeTab === 'email' ? 'active' : ''}
          onClick={() => setActiveTab('email')}
        >
          Email
        </button>
        <button 
          className={activeTab === 'calendar' ? 'active' : ''}
          onClick={() => setActiveTab('calendar')}
        >
          Calendar
        </button>
        <button 
          className={activeTab === 'contacts' ? 'active' : ''}
          onClick={() => setActiveTab('contacts')}
        >
          Contacts
        </button>
        <button 
          className={activeTab === 'notifications' ? 'active' : ''}
          onClick={() => setActiveTab('notifications')}
        >
          Notifications
        </button>
        <button 
          className={activeTab === 'theme' ? 'active' : ''}
          onClick={() => setActiveTab('theme')}
        >
          Theme
        </button>
      </div>
      
      <div className="settings-content">
        {activeTab === 'email' && (
          <EmailSettings 
            settings={settings.email}
            onChange={(emailSettings) => 
              setSettings({ ...settings, email: emailSettings })
            }
          />
        )}
        {activeTab === 'calendar' && (
          <CalendarSettings 
            settings={settings.calendar}
            onChange={(calendarSettings) => 
              setSettings({ ...settings, calendar: calendarSettings })
            }
          />
        )}
        {/* ... other tabs */}
      </div>
      
      <div className="settings-footer">
        <button onClick={() => settingsManager.resetSettings()}>Reset to Defaults</button>
        <button onClick={handleSave}>Save</button>
      </div>
    </div>
  );
};
```

### 3. Status Check Component

```typescript
import React, { useState, useEffect } from 'react';

export const OutlookStatusIndicator: React.FC = () => {
  const [status, setStatus] = useState<OutlookStatusResponse | null>(null);
  const [loading, setLoading] = useState(true);
  
  useEffect(() => {
    checkStatus();
    const interval = setInterval(checkStatus, 30000); // Check every 30s
    return () => clearInterval(interval);
  }, []);
  
  const checkStatus = async () => {
    try {
      const response = await fetch('/api/outlook/status');
      const data = await response.json();
      setStatus(data);
    } catch (error) {
      console.error('Failed to check Outlook status:', error);
    } finally {
      setLoading(false);
    }
  };
  
  if (loading) {
    return <div className="outlook-status loading">Checking...</div>;
  }
  
  if (!status) {
    return <div className="outlook-status error">Status unavailable</div>;
  }
  
  if (status.platform !== 'windows') {
    return (
      <div className="outlook-status warning">
        Outlook COM is Windows-only
      </div>
    );
  }
  
  if (!status.enabled) {
    return (
      <div className="outlook-status disabled">
        Outlook COM disabled. Set OUTLOOK_COM_ENABLED=true
      </div>
    );
  }
  
  if (!status.available) {
    return (
      <div className="outlook-status unavailable">
        Outlook not available. Please ensure Outlook is installed and running.
      </div>
    );
  }
  
  return (
    <div className="outlook-status available">
      ✓ Outlook COM Available
    </div>
  );
};
```

---

## Settings UI Components

### Recommended UI Structure

```
Outlook Settings
├── General
│   ├── Default Folder
│   ├── Items Per Page
│   └── Auto-refresh Interval
├── Email
│   ├── Display Options
│   ├── Composition Settings
│   ├── Filtering & Search
│   └── Templates
├── Calendar
│   ├── View Preferences
│   ├── Default Duration
│   ├── Working Hours
│   └── Reminder Settings
├── Contacts
│   ├── View Mode
│   ├── Display Options
│   └── Sorting & Grouping
├── Notifications
│   ├── Email Notifications
│   ├── Calendar Reminders
│   └── Sound & Visual
└── Theme & Appearance
    ├── Color Scheme
    ├── Layout
    └── Typography
```

---

## Best Practices

### 1. Settings Storage

- **Use localStorage** for client-side preferences (UI settings, view preferences)
- **Consider backend storage** for settings that should sync across devices
- **Validate settings** before saving to prevent invalid configurations

### 2. Default Values

- Always provide sensible defaults
- Allow users to reset to defaults easily
- Document what each setting does

### 3. Performance

- **Lazy load settings** - Don't load all settings at once
- **Debounce saves** - Don't save on every keystroke
- **Cache settings** - Store in memory after first load

### 4. User Experience

- **Group related settings** - Use tabs or sections
- **Provide tooltips** - Explain what each setting does
- **Show previews** - For theme/appearance settings
- **Validate inputs** - Show errors for invalid values

### 5. Platform Awareness

- **Check platform** - Always verify Windows before showing Outlook features
- **Graceful degradation** - Hide Outlook features on non-Windows
- **Clear messaging** - Explain why features are unavailable

### 6. Error Handling

```typescript
async function handleOutlookOperation<T>(
  operation: () => Promise<T>
): Promise<T | null> {
  try {
    // Check status first
    const status = await checkOutlookStatus();
    if (!status.available) {
      showError('Outlook is not available. Please ensure Outlook is installed and running.');
      return null;
    }
    
    return await operation();
  } catch (error) {
    console.error('Outlook operation failed:', error);
    showError(`Outlook operation failed: ${error.message}`);
    return null;
  }
}
```

---

## Summary

### Current Settings (Read-only from Frontend)
- `OUTLOOK_COM_ENABLED` - Master enable/disable flag (env var only)

### Recommended Frontend Settings
1. **Email Display** - View preferences, sorting, filtering
2. **Email Composition** - Defaults, signatures, auto-save
3. **Calendar View** - View type, date range, display options
4. **Contacts View** - View mode, display options, sorting
5. **Notifications** - Email alerts, calendar reminders
6. **Theme & Appearance** - Colors, layout, typography
7. **Interaction** - Click behavior, keyboard shortcuts, drag & drop

### Storage Recommendations
- **localStorage** for UI preferences (client-side only)
- **Backend API** for settings that need to sync (future enhancement)
- **Default values** always available as fallback

---

**Last Updated**: 2024  
**Maintained By**: Phoenix AGI Development Team
