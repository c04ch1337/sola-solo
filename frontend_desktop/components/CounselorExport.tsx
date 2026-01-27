import React, { useMemo, useState } from 'react';
import { getPhoenixApiBase } from '../env';

export default function CounselorExport() {
  const PHOENIX_API_BASE = useMemo(() => getPhoenixApiBase(), []);

  const [downloading, setDownloading] = useState(false);

  const download = async () => {
    try {
      setDownloading(true);
      const res = await fetch(`${PHOENIX_API_BASE}/api/counselor/export?days=7`);
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const text = await res.text();

      const d = new Date();
      const yyyy = d.getFullYear();
      const mm = String(d.getMonth() + 1).padStart(2, '0');
      const dd = String(d.getDate()).padStart(2, '0');
      const filename = `Counselor_Report_${yyyy}-${mm}-${dd}.md`;

      const blob = new Blob([text], { type: 'text/markdown;charset=utf-8' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = filename;
      document.body.appendChild(a);
      a.click();
      a.remove();
      URL.revokeObjectURL(url);
    } finally {
      setDownloading(false);
    }
  };

  return (
    <button
      onClick={download}
      disabled={downloading}
      className="px-3 py-1.5 rounded-full border text-[10px] font-bold uppercase tracking-widest transition-colors bg-primary/15 border-primary/30 text-primary hover:bg-primary/20 disabled:opacity-50"
      title="Download Counselor report (.md)"
    >
      {downloading ? 'Exporting…' : 'Download Report (.md)'}
    </button>
  );
}

