/**
 * Notification Service
 * 
 * Handles OS-level notifications via Tauri API
 * Used for long-running tasks, important events, and proactive messages
 */

// Check if Tauri API is available
const isTauriAvailable = () => {
  return typeof window !== 'undefined' && (window as any).__TAURI__;
};

/**
 * Send a system notification
 * @param title Notification title
 * @param body Notification body text
 * @returns Promise that resolves when notification is sent
 */
export async function sendNotification(title: string, body: string): Promise<void> {
  if (!isTauriAvailable()) {
    console.warn('Tauri API not available, notification not sent:', { title, body });
    return;
  }

  try {
    const { invoke } = (window as any).__TAURI__.tauri;
    await invoke('send_notification', { title, body });
  } catch (error) {
    console.error('Failed to send notification:', error);
    throw error;
  }
}

/**
 * Notify about dream completion
 */
export async function notifyDreamComplete(dreamType: string): Promise<void> {
  await sendNotification(
    '✨ Dream Complete',
    `Your ${dreamType} dream session has finished. Check the Dreams panel for details.`
  );
}

/**
 * Notify about agent spawn
 */
export async function notifyAgentSpawned(agentType: string): Promise<void> {
  await sendNotification(
    '🤖 Agent Spawned',
    `A new ${agentType} agent has been created and is ready to assist.`
  );
}

/**
 * Notify about memory creation
 */
export async function notifyMemoryCreated(memoryType: string): Promise<void> {
  await sendNotification(
    '💭 Memory Created',
    `A new ${memoryType} memory has been stored in your vault.`
  );
}

/**
 * Notify about approval needed
 */
export async function notifyApprovalNeeded(action: string): Promise<void> {
  await sendNotification(
    '⚠️ Approval Needed',
    `Sola needs your approval to: ${action}`
  );
}

/**
 * Notify about proactive message
 */
export async function notifyProactiveMessage(preview: string): Promise<void> {
  await sendNotification(
    '💬 Message from Sola',
    preview
  );
}

/**
 * Notify about task completion
 */
export async function notifyTaskComplete(taskName: string): Promise<void> {
  await sendNotification(
    '✅ Task Complete',
    `${taskName} has finished successfully.`
  );
}

/**
 * Notify about error
 */
export async function notifyError(errorMessage: string): Promise<void> {
  await sendNotification(
    '❌ Error',
    errorMessage
  );
}

export default {
  sendNotification,
  notifyDreamComplete,
  notifyAgentSpawned,
  notifyMemoryCreated,
  notifyApprovalNeeded,
  notifyProactiveMessage,
  notifyTaskComplete,
  notifyError,
};
