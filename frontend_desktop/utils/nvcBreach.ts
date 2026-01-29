export interface NvcBreach {
  kind: string;
  needle: string;
  message: string;
}

/**
 * Minimal, deterministic "breach" scan for Non-Violent Communication (NVC) violations.
 *
 * Note: The existing resonance analyzer already flags some of these.
 * This returns structured items so the UI can highlight.
 */
export function detectNvcBreaches(script: string): NvcBreach[] {
  const raw = script.trim();
  const t = raw.toLowerCase();
  const out: NvcBreach[] = [];

  const push = (kind: string, needle: string, msg: string) => {
    if (t.includes(needle)) {
      out.push({
        kind,
        needle,
        message: msg,
      });
    }
  };

  // Absolutes / globalized judgments
  for (const w of ['always', 'never']) {
    push(
      'absolute',
      w,
      'Absolutes can be heard as character judgments. Swap for a specific recent instance.',
    );
  }

  // Directives
  for (const w of ['you should', 'you need to', 'you have to']) {
    push(
      'directive',
      w,
      "Directive language often triggers defensiveness. Try an invitational request (e.g., 'Would you be willing to…').",
    );
  }

  // Blame pattern
  for (const w of ['you make me feel', 'because you', 'your fault']) {
    push(
      'blame',
      w,
      "This reads as blame. Try: 'When I notice…, I feel…, because I need… Would you be willing to…'",
    );
  }

  // "You" statements (very rough heuristic)
  push(
    'you_statement',
    'you are',
    "'You are…' often lands as evaluation. Try describing an observable behavior instead.",
  );

  return out;
}
