import React, { useState, useEffect } from 'react';
import { MemoryService, Vault, CortexLayer, MemoryItem, CortexMemoryItem, VectorMemoryResult } from '../services/memoryService';

interface MemoryBrowserProps {
  memoryService: MemoryService;
}

export const MemoryBrowser: React.FC<MemoryBrowserProps> = ({ memoryService }) => {
  const [query, setQuery] = useState('');
  const [vaultResults, setVaultResults] = useState<MemoryItem[]>([]);
  const [cortexResults, setCortexResults] = useState<CortexMemoryItem[]>([]);
  const [vectorResults, setVectorResults] = useState<VectorMemoryResult[]>([]);
  const [vault, setVault] = useState<Vault>('soul');
  const [tab, setTab] = useState<'vault' | 'cortex' | 'vector'>('vault');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Subscribe to memory results
  useEffect(() => {
    const unsubVault = memoryService.onResult('memory_search', (response: any) => {
      if (response.items) {
        setVaultResults(response.items.map((item: any) => ({
          key: item.key,
          value: item.value
        })));
        setLoading(false);
        setError(null);
      }
    });

    const unsubCortex = memoryService.onResult('memory_cortex_search', (response: any) => {
      if (response.items) {
        setCortexResults(response.items.map((item: any) => ({
          key: item.key,
          layer: item.layer,
          value: item.value
        })));
        setLoading(false);
        setError(null);
      }
    });

    const unsubVector = memoryService.onResult('memory_vector_search', (response: any) => {
      if (response.results) {
        setVectorResults(response.results);
        setLoading(false);
        setError(null);
      }
    });

    const unsubError = memoryService.onResult('error', (response: any) => {
      setError(response.message || 'Unknown error');
      setLoading(false);
    });

    return () => {
      unsubVault();
      unsubCortex();
      unsubVector();
      unsubError();
    };
  }, [memoryService]);

  const handleSearch = () => {
    if (!query.trim()) {
      setError('Please enter a search query');
      return;
    }

    if (!memoryService.isConnected()) {
      setError('WebSocket not connected. Please wait for connection.');
      return;
    }

    setLoading(true);
    setError(null);

    if (tab === 'vault') {
      memoryService.searchVault(query, 10, vault);
    } else if (tab === 'vector') {
      memoryService.searchVector(query, 5);
    } else {
      memoryService.searchCortex(query, 10);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSearch();
    }
  };

  return (
    <div className="p-6 space-y-4 bg-panel-dark border border-border-dark rounded-xl">
      <div className="flex items-center justify-between">
        <h2 className="text-lg font-bold text-white uppercase tracking-wider">Memory Browser</h2>
        <div className="flex items-center gap-2">
          <span className={`size-2 rounded-full ${memoryService.isConnected() ? 'bg-green-500' : 'bg-red-500'}`}></span>
          <span className="text-[10px] font-mono text-slate-400 uppercase">
            {memoryService.isConnected() ? 'Connected' : 'Disconnected'}
          </span>
        </div>
      </div>

      {/* Tabs */}
      <div className="flex gap-2 border-b border-border-dark">
        <button
          onClick={() => setTab('vault')}
          className={`px-4 py-2 text-sm font-bold uppercase tracking-wider transition-colors ${
            tab === 'vault'
              ? 'text-primary border-b-2 border-primary'
              : 'text-slate-400 hover:text-slate-300'
          }`}
        >
          Vaults
        </button>
        <button
          onClick={() => setTab('cortex')}
          className={`px-4 py-2 text-sm font-bold uppercase tracking-wider transition-colors ${
            tab === 'cortex'
              ? 'text-primary border-b-2 border-primary'
              : 'text-slate-400 hover:text-slate-300'
          }`}
        >
          Cortex Layers
        </button>
        <button
          onClick={() => setTab('vector')}
          className={`px-4 py-2 text-sm font-bold uppercase tracking-wider transition-colors ${
            tab === 'vector'
              ? 'text-primary border-b-2 border-primary'
              : 'text-slate-400 hover:text-slate-300'
          }`}
        >
          Semantic (Vector)
        </button>
      </div>

      {/* Vault Selection (only for vault tab) */}
      {tab === 'vault' && (
        <div className="flex items-center gap-4">
          <label className="text-sm font-bold text-slate-400 uppercase tracking-wider">Vault:</label>
          <select
            value={vault}
            onChange={(e) => setVault(e.target.value as Vault)}
            className="px-3 py-1.5 bg-background-dark border border-border-dark rounded-lg text-slate-200 text-sm focus:outline-none focus:ring-2 focus:ring-primary"
          >
            <option value="soul">Soul (encrypted)</option>
            <option value="mind">Mind (plaintext)</option>
            <option value="body">Body (plaintext)</option>
          </select>
        </div>
      )}

      {/* Search Input */}
      <div className="flex gap-2">
        <input
          type="text"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          onKeyPress={handleKeyPress}
          placeholder={
            tab === 'vault'
              ? 'Search vault memories...'
              : tab === 'cortex'
              ? 'Search cortex layers (prefix)...'
              : 'Semantic search...'
          }
          className="flex-1 px-4 py-2 bg-background-dark border border-border-dark rounded-lg text-slate-200 placeholder:text-slate-600 focus:outline-none focus:ring-2 focus:ring-primary"
        />
        <button
          onClick={handleSearch}
          disabled={loading || !memoryService.isConnected()}
          className="px-6 py-2 bg-primary hover:bg-primary/90 text-white rounded-lg font-bold uppercase tracking-wider text-sm transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {loading ? 'Searching...' : 'Search'}
        </button>
      </div>

      {/* Error Message */}
      {error && (
        <div className="p-3 bg-red-500/10 border border-red-500/20 rounded-lg">
          <p className="text-sm text-red-400">{error}</p>
        </div>
      )}

      {/* Results */}
      <div className="space-y-2 max-h-96 overflow-y-auto">
        {tab === 'vault' && vaultResults.length > 0 && (
          <div className="space-y-2">
            <div className="text-xs font-mono text-slate-500 uppercase tracking-widest">
              Found {vaultResults.length} results in {vault} vault
            </div>
            {vaultResults.map((item, i) => (
              <div key={i} className="p-4 bg-background-dark border border-border-dark rounded-lg">
                <div className="font-mono text-xs text-slate-400 mb-2 break-all">{item.key}</div>
                <div className="text-sm text-slate-200 whitespace-pre-wrap">{item.value}</div>
              </div>
            ))}
          </div>
        )}

        {tab === 'cortex' && cortexResults.length > 0 && (
          <div className="space-y-2">
            <div className="text-xs font-mono text-slate-500 uppercase tracking-widest">
              Found {cortexResults.length} results in cortex
            </div>
            {cortexResults.map((item, i) => (
              <div key={i} className="p-4 bg-background-dark border border-border-dark rounded-lg">
                <div className="flex items-center justify-between mb-2">
                  <div className="font-mono text-xs text-slate-400 break-all">{item.key}</div>
                  <span className="px-2 py-0.5 bg-primary/20 text-primary text-[10px] font-bold uppercase rounded">
                    {item.layer}
                  </span>
                </div>
                <div className="text-sm text-slate-200 whitespace-pre-wrap">{item.value}</div>
              </div>
            ))}
          </div>
        )}

        {tab === 'vector' && vectorResults.length > 0 && (
          <div className="space-y-2">
            <div className="text-xs font-mono text-slate-500 uppercase tracking-widest">
              Found {vectorResults.length} semantic matches
            </div>
            {vectorResults.map((result, i) => (
              <div key={i} className="p-4 bg-background-dark border border-border-dark rounded-lg">
                <div className="flex items-center justify-between mb-2">
                  <div className="text-xs font-mono text-slate-400">ID: {result.id}</div>
                  <div className="px-2 py-0.5 bg-green-500/20 text-green-400 text-[10px] font-bold rounded">
                    {(result.score * 100).toFixed(1)}% match
                  </div>
                </div>
                <div className="text-sm text-slate-200 mb-2">{result.text}</div>
                {result.metadata && Object.keys(result.metadata).length > 0 && (
                  <div className="text-xs text-slate-500 font-mono">
                    {JSON.stringify(result.metadata, null, 2)}
                  </div>
                )}
              </div>
            ))}
          </div>
        )}

        {!loading && query && vaultResults.length === 0 && cortexResults.length === 0 && vectorResults.length === 0 && (
          <div className="text-center py-8 text-slate-500">
            <p>No results found</p>
            <p className="text-xs mt-2">Try a different search query</p>
          </div>
        )}
      </div>
    </div>
  );
};
