//! Windows-specific Outlook COM implementation
//!
//! Uses COM automation to interact with Outlook application.
//!
//! Note: Full COM interop requires proper type definitions.
//! This implementation uses a PowerShell bridge for COM operations
//! which is more reliable than raw COM interop in Rust.

use crate::*;
use serde_json;
use std::process::Command;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Windows implementation of Outlook COM manager
///
/// Uses PowerShell to interact with Outlook COM objects.
/// This approach is more reliable than raw COM interop in Rust.
pub struct OutlookComManagerImpl {
    _initialized: Arc<Mutex<bool>>,
}

impl OutlookComManagerImpl {
    pub fn new() -> Result<Self, crate::OutlookError> {
        // Test if Outlook is available
        if !Self::test_outlook_available() {
            return Err(crate::OutlookError::OutlookNotAvailable(
                "Outlook is not installed or not accessible".to_string(),
            ));
        }

        Ok(Self {
            _initialized: Arc::new(Mutex::new(true)),
        })
    }

    fn test_outlook_available() -> bool {
        // Test by trying to create Outlook.Application via PowerShell
        let script = r#"
            try {
                $outlook = New-Object -ComObject Outlook.Application
                $null = $outlook.GetNamespace("MAPI")
                $true
            } catch {
                $false
            }
        "#;

        let output = Command::new("powershell")
            .arg("-NoProfile")
            .arg("-Command")
            .arg(script)
            .output();

        match output {
            Ok(o) => {
                let result = String::from_utf8_lossy(&o.stdout);
                let trimmed = result.trim();
                trimmed == "True"
            }
            Err(_) => false,
        }
    }

    pub fn is_available(&self) -> bool {
        Self::test_outlook_available()
    }

    async fn execute_powershell_script(script: &str) -> Result<String, OutlookError> {
        let output = tokio::task::spawn_blocking({
            let script = script.to_string();
            move || {
                Command::new("powershell")
                    .arg("-NoProfile")
                    .arg("-Command")
                    .arg(&script)
                    .output()
            }
        })
        .await
        .map_err(|e| OutlookError::ComOperationFailed(format!("Task join failed: {e}")))?
        .map_err(|e| OutlookError::ComOperationFailed(format!("IO error: {e}")))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(OutlookError::ComOperationFailed(format!(
                "PowerShell error: {stderr}"
            )));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub async fn list_folders(&self) -> Result<Vec<crate::OutlookFolder>, crate::OutlookError> {
        let script = r#"
            $outlook = New-Object -ComObject Outlook.Application
            $namespace = $outlook.GetNamespace("MAPI")
            $inbox = $namespace.GetDefaultFolder(6)  # olFolderInbox
            
            $folders = @()
            function Get-Folders($folder) {
                $folderInfo = @{
                    Name = $folder.Name
                    EntryID = $folder.EntryID
                    ItemCount = $folder.Items.Count
                    UnreadCount = $folder.UnreadItemCount
                    Subfolders = @()
                }
                
                foreach ($subfolder in $folder.Folders) {
                    $folderInfo.Subfolders += (Get-Folders $subfolder)
                }
                
                return $folderInfo
            }
            
            $result = Get-Folders $inbox
            $result | ConvertTo-Json -Depth 10
        "#;

        let output = Self::execute_powershell_script(script).await?;

        // Parse JSON response
        let folder: serde_json::Value = serde_json::from_str(&output)
            .map_err(|e| OutlookError::ComOperationFailed(format!("JSON parse error: {e}")))?;

        // Convert to OutlookFolder
        Ok(vec![crate::OutlookFolder {
            name: folder["Name"].as_str().unwrap_or("Inbox").to_string(),
            entry_id: folder["EntryID"].as_str().unwrap_or("").to_string(),
            item_count: folder["ItemCount"].as_u64().unwrap_or(0) as u32,
            unread_count: folder["UnreadCount"].as_u64().unwrap_or(0) as u32,
            subfolders: vec![], // TODO: Parse subfolders recursively
        }])
    }

    pub async fn get_emails(
        &self,
        folder_name: &str,
        max_count: Option<usize>,
    ) -> Result<Vec<crate::OutlookEmail>, crate::OutlookError> {
        let max = max_count.unwrap_or(50);
        let folder_map = match folder_name {
            "Inbox" => "6",         // olFolderInbox
            "Sent Items" => "5",    // olFolderSentMail
            "Drafts" => "16",       // olFolderDrafts
            "Deleted Items" => "3", // olFolderDeletedItems
            _ => "6",               // Default to Inbox
        };

        let script = format!(
            r#"
            $outlook = New-Object -ComObject Outlook.Application
            $namespace = $outlook.GetNamespace("MAPI")
            $folder = $namespace.GetDefaultFolder({})
            $items = $folder.Items
            $items.Sort("[ReceivedTime]", $true)  # Sort by received time, descending
            
            $emails = @()
            $count = 0
            $max = {}
            
            foreach ($item in $items) {{
                if ($count -ge $max) {{ break }}
                if ($item.Class -eq "IPM.Note") {{
                    $email = @{{
                        EntryID = $item.EntryID
                        Subject = $item.Subject
                        From = $item.SenderEmailAddress
                        To = $item.To
                        CC = if ($item.CC) {{ $item.CC }} else {{ $null }}
                        BCC = if ($item.BCC) {{ $item.BCC }} else {{ $null }}
                        Body = $item.Body
                        BodyHTML = if ($item.HTMLBody) {{ $item.HTMLBody }} else {{ $null }}
                        ReceivedTime = $item.ReceivedTime.ToString("o")
                        SentTime = $item.SentOn.ToString("o")
                        Importance = $item.Importance.ToString()
                        IsRead = $item.UnRead -eq $false
                        HasAttachments = $item.Attachments.Count -gt 0
                        Categories = if ($item.Categories) {{ $item.Categories -split "," }} else {{ @() }}
                    }}
                    $emails += $email
                    $count++
                }}
            }}
            
            $emails | ConvertTo-Json -Depth 5
        "#,
            folder_map, max
        );

        let output = Self::execute_powershell_script(&script).await?;

        let emails_json: Vec<serde_json::Value> = serde_json::from_str(&output)
            .map_err(|e| OutlookError::ComOperationFailed(format!("JSON parse error: {e}")))?;

        let mut emails = Vec::new();
        for email_json in emails_json {
            emails.push(crate::OutlookEmail {
                entry_id: email_json["EntryID"].as_str().unwrap_or("").to_string(),
                subject: email_json["Subject"].as_str().unwrap_or("").to_string(),
                from: email_json["From"].as_str().unwrap_or("").to_string(),
                to: email_json["To"].as_str().unwrap_or("").to_string(),
                cc: email_json["CC"].as_str().and_then(|s| {
                    if s == "null" {
                        None
                    } else {
                        Some(s.to_string())
                    }
                }),
                bcc: email_json["BCC"].as_str().and_then(|s| {
                    if s == "null" {
                        None
                    } else {
                        Some(s.to_string())
                    }
                }),
                body: email_json["Body"].as_str().unwrap_or("").to_string(),
                body_html: email_json["BodyHTML"].as_str().map(|s| s.to_string()),
                received_time: email_json["ReceivedTime"].as_str().map(|s| s.to_string()),
                sent_time: email_json["SentTime"].as_str().map(|s| s.to_string()),
                importance: email_json["Importance"]
                    .as_str()
                    .unwrap_or("Normal")
                    .to_string(),
                is_read: email_json["IsRead"].as_bool().unwrap_or(false),
                has_attachments: email_json["HasAttachments"].as_bool().unwrap_or(false),
                attachments: vec![], // TODO: Parse attachments
                categories: email_json["Categories"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default(),
            });
        }

        Ok(emails)
    }

    pub async fn send_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
        html_body: Option<&str>,
        cc: Option<&str>,
        bcc: Option<&str>,
        attachments: Option<Vec<&str>>,
    ) -> Result<(), crate::OutlookError> {
        let html_body_str = html_body.unwrap_or("");
        let cc_str = cc.unwrap_or("");
        let bcc_str = bcc.unwrap_or("");

        let attachments_script = if let Some(atts) = attachments {
            atts.iter()
                .map(|path| format!("$mail.Attachments.Add('{}')", path.replace("'", "''")))
                .collect::<Vec<_>>()
                .join("; ")
        } else {
            String::new()
        };

        let script = format!(
            r#"
            $outlook = New-Object -ComObject Outlook.Application
            $mail = $outlook.CreateItem(0)  # olMailItem
            
            $mail.To = '{}'
            $mail.Subject = '{}'
            $mail.Body = '{}'
            {}
            {}
            {}
            {}
            
            $mail.Send()
            "Success"
        "#,
            to.replace("'", "''"),
            subject.replace("'", "''"),
            body.replace("'", "''"),
            if !html_body_str.is_empty() {
                format!("$mail.HTMLBody = '{}'", html_body_str.replace("'", "''"))
            } else {
                String::new()
            },
            if !cc_str.is_empty() {
                format!("$mail.CC = '{}'", cc_str.replace("'", "''"))
            } else {
                String::new()
            },
            if !bcc_str.is_empty() {
                format!("$mail.BCC = '{}'", bcc_str.replace("'", "''"))
            } else {
                String::new()
            },
            attachments_script
        );

        Self::execute_powershell_script(&script).await?;
        Ok(())
    }

    pub async fn get_contacts(&self) -> Result<Vec<crate::OutlookContact>, crate::OutlookError> {
        let script = r#"
            $outlook = New-Object -ComObject Outlook.Application
            $namespace = $outlook.GetNamespace("MAPI")
            $contacts = $namespace.GetDefaultFolder(10)  # olFolderContacts
            
            $contactList = @()
            foreach ($contact in $contacts.Items) {
                if ($contact.Class -eq "IPM.Contact") {
                    $contactInfo = @{
                        EntryID = $contact.EntryID
                        FirstName = if ($contact.FirstName) { $contact.FirstName } else { "" }
                        LastName = if ($contact.LastName) { $contact.LastName } else { "" }
                        FullName = if ($contact.FullName) { $contact.FullName } else { "" }
                        EmailAddresses = @()
                        PhoneNumbers = @()
                        Company = if ($contact.CompanyName) { $contact.CompanyName } else { $null }
                        JobTitle = if ($contact.JobTitle) { $contact.JobTitle } else { $null }
                    }
                    
                    if ($contact.Email1Address) { $contactInfo.EmailAddresses += $contact.Email1Address }
                    if ($contact.Email2Address) { $contactInfo.EmailAddresses += $contact.Email2Address }
                    if ($contact.Email3Address) { $contactInfo.EmailAddresses += $contact.Email3Address }
                    
                    if ($contact.BusinessTelephoneNumber) { $contactInfo.PhoneNumbers += $contact.BusinessTelephoneNumber }
                    if ($contact.HomeTelephoneNumber) { $contactInfo.PhoneNumbers += $contact.HomeTelephoneNumber }
                    if ($contact.MobileTelephoneNumber) { $contactInfo.PhoneNumbers += $contact.MobileTelephoneNumber }
                    
                    $contactList += $contactInfo
                }
            }
            
            $contactList | ConvertTo-Json -Depth 3
        "#;

        let output = Self::execute_powershell_script(script).await?;

        let contacts_json: Vec<serde_json::Value> = serde_json::from_str(&output)
            .map_err(|e| OutlookError::ComOperationFailed(format!("JSON parse error: {e}")))?;

        let mut contacts = Vec::new();
        for contact_json in contacts_json {
            contacts.push(crate::OutlookContact {
                entry_id: contact_json["EntryID"].as_str().unwrap_or("").to_string(),
                first_name: contact_json["FirstName"].as_str().unwrap_or("").to_string(),
                last_name: contact_json["LastName"].as_str().unwrap_or("").to_string(),
                full_name: contact_json["FullName"].as_str().unwrap_or("").to_string(),
                email_addresses: contact_json["EmailAddresses"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default(),
                phone_numbers: contact_json["PhoneNumbers"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default(),
                company: contact_json["Company"].as_str().map(|s| s.to_string()),
                job_title: contact_json["JobTitle"].as_str().map(|s| s.to_string()),
            });
        }

        Ok(contacts)
    }

    pub async fn get_appointments(
        &self,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<Vec<crate::OutlookAppointment>, crate::OutlookError> {
        let _start_filter = start_date.unwrap_or("");
        let _end_filter = end_date.unwrap_or("");

        let script = format!(
            r#"
            $outlook = New-Object -ComObject Outlook.Application
            $namespace = $outlook.GetNamespace("MAPI")
            $calendar = $namespace.GetDefaultFolder(9)  # olFolderCalendar
            
            $items = $calendar.Items
            $items.Sort("[Start]", $true)
            
            $appointments = @()
            foreach ($item in $items) {{
                if ($item.Class -eq "IPM.Appointment") {{
                    $appt = @{{
                        EntryID = $item.EntryID
                        Subject = $item.Subject
                        StartTime = $item.Start.ToString("o")
                        EndTime = $item.End.ToString("o")
                        Location = if ($item.Location) {{ $item.Location }} else {{ $null }}
                        Body = if ($item.Body) {{ $item.Body }} else {{ $null }}
                        Organizer = if ($item.Organizer) {{ $item.Organizer }} else {{ $null }}
                        RequiredAttendees = if ($item.RequiredAttendees) {{ ($item.RequiredAttendees -split ";") }} else {{ @() }}
                        OptionalAttendees = if ($item.OptionalAttendees) {{ ($item.OptionalAttendees -split ";") }} else {{ @() }}
                        IsAllDay = $item.AllDayEvent
                        ReminderMinutes = if ($item.ReminderMinutesBeforeStart) {{ $item.ReminderMinutesBeforeStart }} else {{ $null }}
                    }}
                    $appointments += $appt
                }}
            }}
            
            $appointments | ConvertTo-Json -Depth 3
        "#
        );

        let output = Self::execute_powershell_script(&script).await?;

        let appointments_json: Vec<serde_json::Value> = serde_json::from_str(&output)
            .map_err(|e| OutlookError::ComOperationFailed(format!("JSON parse error: {e}")))?;

        let mut appointments = Vec::new();
        for appt_json in appointments_json {
            appointments.push(crate::OutlookAppointment {
                entry_id: appt_json["EntryID"].as_str().unwrap_or("").to_string(),
                subject: appt_json["Subject"].as_str().unwrap_or("").to_string(),
                start_time: appt_json["StartTime"].as_str().unwrap_or("").to_string(),
                end_time: appt_json["EndTime"].as_str().unwrap_or("").to_string(),
                location: appt_json["Location"].as_str().map(|s| s.to_string()),
                body: appt_json["Body"].as_str().map(|s| s.to_string()),
                organizer: appt_json["Organizer"].as_str().map(|s| s.to_string()),
                required_attendees: appt_json["RequiredAttendees"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default(),
                optional_attendees: appt_json["OptionalAttendees"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default(),
                is_all_day: appt_json["IsAllDay"].as_bool().unwrap_or(false),
                reminder_minutes: appt_json["ReminderMinutes"].as_u64().map(|n| n as u32),
            });
        }

        Ok(appointments)
    }

    pub async fn create_appointment(
        &self,
        subject: &str,
        start_time: &str,
        end_time: &str,
        location: Option<&str>,
        body: Option<&str>,
        required_attendees: Option<Vec<&str>>,
        optional_attendees: Option<Vec<&str>>,
        reminder_minutes: Option<u32>,
    ) -> Result<String, crate::OutlookError> {
        let location_str = location.unwrap_or("");
        let body_str = body.unwrap_or("");
        let required_str = required_attendees.map(|v| v.join(";")).unwrap_or_default();
        let optional_str = optional_attendees.map(|v| v.join(";")).unwrap_or_default();
        let reminder_str = reminder_minutes
            .map(|m| format!("$appt.ReminderMinutesBeforeStart = {}", m))
            .unwrap_or_default();

        let script = format!(
            r#"
            $outlook = New-Object -ComObject Outlook.Application
            $appt = $outlook.CreateItem(1)  # olAppointmentItem
            
            $appt.Subject = '{}'
            $appt.Start = [DateTime]::Parse('{}')
            $appt.End = [DateTime]::Parse('{}')
            {}
            {}
            {}
            {}
            {}
            
            $appt.Save()
            $appt.EntryID
        "#,
            subject.replace("'", "''"),
            start_time,
            end_time,
            if !location_str.is_empty() {
                format!("$appt.Location = '{}'", location_str.replace("'", "''"))
            } else {
                String::new()
            },
            if !body_str.is_empty() {
                format!("$appt.Body = '{}'", body_str.replace("'", "''"))
            } else {
                String::new()
            },
            if !required_str.is_empty() {
                format!(
                    "$appt.RequiredAttendees = '{}'",
                    required_str.replace("'", "''")
                )
            } else {
                String::new()
            },
            if !optional_str.is_empty() {
                format!(
                    "$appt.OptionalAttendees = '{}'",
                    optional_str.replace("'", "''")
                )
            } else {
                String::new()
            },
            reminder_str
        );

        let entry_id = Self::execute_powershell_script(&script).await?;
        Ok(entry_id.trim().to_string())
    }
}

// Note: Full COM interop implementation requires:
// 1. Proper COM type definitions for Outlook objects
// 2. IDispatch method invocation helpers
// 3. Variant type handling
// 4. Error handling for COM HRESULT codes
//
// For a production implementation, consider:
// - Using the `windows` crate's code generation for Outlook type libraries
// - Or using a COM interop library like `com-rs` or `intercom`
// - Or creating a C++/CLI bridge DLL
