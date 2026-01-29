# Outlook COM Automation Integration

**Windows-only feature for direct Outlook integration**

## Overview

The Outlook COM Automation module provides Phoenix AGI with direct access to Microsoft Outlook via COM (Component Object Model) automation. This enables full integration with Outlook's email, contacts, calendar, and tasks without requiring SMTP/IMAP credentials.

## Features

✅ **Read all Outlook folders** (including subfolders)  
✅ **Send/receive emails** directly through Outlook  
✅ **Access contacts** from Outlook address book  
✅ **Access calendar** appointments and create new ones  
✅ **Parse email bodies** and attachments  
✅ **Works with cached Exchange/365 data** (no network credentials needed)  
✅ **Full folder access** including custom folders  

## Platform Support

- **Windows Only**: Requires Windows OS
- **Outlook Versions**: Outlook 2010, 2013, 2016, 2019, 2021, and Office 365
- **Requirements**: Outlook must be installed and configured on the system

## Architecture

The implementation uses a **PowerShell bridge** to interact with Outlook COM objects:

```
Frontend → POST /api/outlook/send
         ↓
Phoenix Web Backend → OutlookComManager
         ↓
PowerShell Script → Outlook.Application COM Object
         ↓
Outlook (Running Instance)
```

### Why PowerShell Bridge?

- **Reliability**: PowerShell has excellent COM interop support
- **Simplicity**: Avoids complex Rust COM interop libraries
- **Maintainability**: Easier to debug and extend
- **Performance**: PowerShell scripts execute quickly for COM operations

## Configuration

### Enable Outlook COM

Add to `.env`:

```bash
# Enable Outlook COM integration (Windows only)
OUTLOOK_COM_ENABLED=true
```

### Requirements

1. **Outlook Installed**: Outlook must be installed and configured
2. **Outlook Running**: Outlook should be running (or will be started automatically)
3. **Windows OS**: Only works on Windows
4. **User Context**: Runs under current user account (no admin needed)

## API Endpoints

### `GET /api/outlook/status`

Check if Outlook COM is available.

**Response:**
```json
{
  "enabled": true,
  "available": true,
  "platform": "windows"
}
```

### `GET /api/outlook/folders`

Get list of all Outlook folders.

**Response:**
```json
[
  {
    "name": "Inbox",
    "entry_id": "...",
    "item_count": 42,
    "unread_count": 5,
    "subfolders": []
  }
]
```

### `GET /api/outlook/emails?folder=Inbox&max_count=50`

Get emails from a folder.

**Query Parameters:**
- `folder`: Folder name (default: "Inbox")
- `max_count`: Maximum number of emails (default: 50)

**Response:**
```json
[
  {
    "entry_id": "...",
    "subject": "Hello",
    "from": "sender@example.com",
    "to": "recipient@example.com",
    "cc": null,
    "bcc": null,
    "body": "Email body text",
    "body_html": "<html>...</html>",
    "received_time": "2024-01-01T12:00:00Z",
    "sent_time": "2024-01-01T11:00:00Z",
    "importance": "Normal",
    "is_read": false,
    "has_attachments": true,
    "attachments": [],
    "categories": []
  }
]
```

### `POST /api/outlook/send`

Send an email via Outlook.

**Request:**
```json
{
  "to": "recipient@example.com",
  "subject": "AI Generated Email",
  "body": "Email content here",
  "html_body": "<html><body>Email content</body></html>",
  "cc": "cc@example.com",
  "bcc": "bcc@example.com",
  "attachments": ["C:\\path\\to\\file.pdf"]
}
```

**Response:**
```json
{
  "status": "sent"
}
```

### `GET /api/outlook/contacts`

Get all Outlook contacts.

**Response:**
```json
[
  {
    "entry_id": "...",
    "first_name": "John",
    "last_name": "Doe",
    "full_name": "John Doe",
    "email_addresses": ["john@example.com"],
    "phone_numbers": ["+1234567890"],
    "company": "Example Corp",
    "job_title": "Developer"
  }
]
```

### `GET /api/outlook/appointments?start_date=2024-01-01&end_date=2024-12-31`

Get calendar appointments.

**Query Parameters:**
- `start_date`: Start date (ISO 8601 format, optional)
- `end_date`: End date (ISO 8601 format, optional)

**Response:**
```json
[
  {
    "entry_id": "...",
    "subject": "Meeting",
    "start_time": "2024-01-01T10:00:00Z",
    "end_time": "2024-01-01T11:00:00Z",
    "location": "Conference Room A",
    "body": "Meeting notes",
    "organizer": "organizer@example.com",
    "required_attendees": ["attendee1@example.com"],
    "optional_attendees": [],
    "is_all_day": false,
    "reminder_minutes": 15
  }
]
```

### `POST /api/outlook/appointments`

Create a new calendar appointment.

**Request:**
```json
{
  "subject": "New Meeting",
  "start_time": "2024-01-01T10:00:00Z",
  "end_time": "2024-01-01T11:00:00Z",
  "location": "Conference Room A",
  "body": "Meeting description",
  "required_attendees": ["attendee1@example.com"],
  "optional_attendees": ["attendee2@example.com"],
  "reminder_minutes": 15
}
```

**Response:**
```json
{
  "status": "created",
  "entry_id": "..."
}
```

## Usage Examples

### Frontend Integration

```typescript
// Check Outlook status
const status = await fetch('/api/outlook/status').then(r => r.json());
if (status.available) {
  // Outlook is ready
}

// Send email
await fetch('/api/outlook/send', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    to: 'recipient@example.com',
    subject: 'Hello from Phoenix',
    body: 'This email was sent via Outlook COM automation.'
  })
});

// Get inbox emails
const emails = await fetch('/api/outlook/emails?folder=Inbox&max_count=10')
  .then(r => r.json());
```

### Command Line Usage

```bash
# Via API
curl -X POST http://127.0.0.1:8888/api/outlook/send \
  -H "Content-Type: application/json" \
  -d '{
    "to": "recipient@example.com",
    "subject": "Test",
    "body": "Test email"
  }'
```

## Security Considerations

### User Context

- **Runs as current user**: No domain admin or elevated privileges needed
- **Uses existing Outlook session**: Leverages user's already-authenticated Outlook
- **No credential storage**: No passwords or tokens stored

### Permissions

- **Same permissions as Outlook**: If you can access it in Outlook, Phoenix can access it
- **Sandboxed**: Runs in user context, not system-wide
- **No network access**: Uses local COM interface, not network protocols

### Best Practices

1. **Enable only when needed**: Set `OUTLOOK_COM_ENABLED=true` only if you need it
2. **Monitor usage**: Check logs for Outlook COM operations
3. **User consent**: Ensure users understand Outlook integration is active
4. **Error handling**: Handle cases where Outlook is not available gracefully

## Troubleshooting

### Outlook Not Available

**Error**: `"Outlook is not installed or not accessible"`

**Solutions**:
1. Ensure Outlook is installed
2. Try opening Outlook manually first
3. Check if Outlook is configured with an email account
4. Verify Outlook is not blocked by antivirus

### PowerShell Errors

**Error**: `"PowerShell error: ..."`

**Solutions**:
1. Ensure PowerShell is available (Windows 7+)
2. Check execution policy: `Get-ExecutionPolicy`
3. If restricted, run: `Set-ExecutionPolicy RemoteSigned -Scope CurrentUser`

### COM Initialization Failed

**Error**: `"COM init failed"`

**Solutions**:
1. Ensure Outlook is not in "safe mode"
2. Try restarting Outlook
3. Check Windows Event Viewer for COM errors

## Comparison with Email ORCH

| Feature | Email ORCH (SMTP/IMAP) | Outlook COM |
|---------|------------------------|-------------|
| **Platform** | Cross-platform | Windows only |
| **Credentials** | Required (SMTP/IMAP) | Not needed (uses Outlook session) |
| **Contacts** | Not accessible | Full access |
| **Calendar** | Not accessible | Full access |
| **Folders** | IMAP folders only | All Outlook folders |
| **Exchange/365** | Requires credentials | Uses cached data |
| **Setup** | Configure SMTP/IMAP | Just enable flag |

## Implementation Details

### PowerShell Scripts

The implementation uses PowerShell scripts that:
1. Create Outlook.Application COM object
2. Get MAPI namespace
3. Access folders, items, contacts, calendar
4. Return JSON results

### Error Handling

- **Platform checks**: Returns `PlatformNotSupported` on non-Windows
- **Availability checks**: Tests Outlook before operations
- **Graceful degradation**: Falls back to error messages if Outlook unavailable

### Performance

- **Async operations**: All operations are async
- **PowerShell execution**: Runs in `spawn_blocking` to avoid blocking
- **Caching**: Outlook COM objects are reused when possible

## Future Enhancements

Potential improvements:
- [ ] Attachment download/save
- [ ] Email rules management
- [ ] Task management
- [ ] Notes access
- [ ] Journal entries
- [ ] Categories management
- [ ] Search functionality
- [ ] Real-time event notifications

## Code Structure

```
outlook_com/
├── src/
│   ├── lib.rs              # Public API
│   ├── windows_impl.rs     # Windows PowerShell bridge
│   └── stub_impl.rs        # Non-Windows stub
└── Cargo.toml
```

## References

- [Outlook Object Model Reference](https://learn.microsoft.com/en-us/office/vba/api/overview/outlook/object-model)
- [PowerShell COM Automation](https://learn.microsoft.com/en-us/powershell/scripting/learn/ps101/09-com-objects)
- [Outlook Folder Types](https://learn.microsoft.com/en-us/office/vba/api/outlook.oldefaultfolders)

---

**Note**: This feature is Windows-only. On non-Windows platforms, the API endpoints will return `"platform": "not_windows"` and operations will fail with `PlatformNotSupported` errors.
