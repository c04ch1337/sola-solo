import React, { useState, useEffect } from 'react';

interface PhotoFile {
  name: string;
  path: string;
  lastModified: Date;
  size: number;
  isExplicit: boolean;
}

interface PhotoCategory {
  name: string;
  count: number;
  lastUpdated: Date | null;
  icon: string;
}

interface PhotoVaultProps {
  compact?: boolean;
}

export const PhotoVault: React.FC<PhotoVaultProps> = ({ compact = false }) => {
  const [photos, setPhotos] = useState<PhotoFile[]>([]);
  const [categories, setCategories] = useState<PhotoCategory[]>([]);
  const [selectedCategory, setSelectedCategory] = useState<string | null>(null);
  const [selectedPhoto, setSelectedPhoto] = useState<PhotoFile | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState('');
  const [sortOrder, setSortOrder] = useState<'newest' | 'oldest' | 'name'>('newest');

  // Get all photos from the local storage
  useEffect(() => {
    const fetchPhotos = async () => {
      try {
        setLoading(true);
        
        // @ts-ignore - solaApp is defined in globals.d.ts
        if (window.solaApp?.listPhotoLibrary) {
          // @ts-ignore - solaApp is defined in globals.d.ts
          const result = await window.solaApp.listPhotoLibrary();
          
          // Transform the data
          const photoList = result.map((item: any) => ({
            name: item.name,
            path: item.path,
            lastModified: new Date(item.lastModified),
            size: item.size,
            isExplicit: item.tags?.includes('explicit') || false
          }));
          
          setPhotos(photoList);
          
          // Generate categories
          generateCategories(photoList);
        } else {
          // Demo data for development
          const demoPhotos: PhotoFile[] = Array(18).fill(0).map((_, i) => ({
            name: `profile_${i + 1}.jpg`,
            path: `/photos/profile_${i + 1}.jpg`,
            lastModified: new Date(Date.now() - Math.random() * 30 * 24 * 60 * 60 * 1000),
            size: Math.floor(Math.random() * 5000000) + 500000,
            isExplicit: Math.random() > 0.4
          }));
          
          setPhotos(demoPhotos);
          generateCategories(demoPhotos);
        }
      } catch (err) {
        console.error('Error fetching photos:', err);
        setError('Failed to load photo library');
      } finally {
        setLoading(false);
      }
    };
    
    fetchPhotos();
  }, []);

  const generateCategories = (photoList: PhotoFile[]) => {
    // Create predefined categories
    const baseCategories: PhotoCategory[] = [
      { name: 'All Photos', count: photoList.length, lastUpdated: null, icon: '📸' },
      { name: 'Professional', count: 0, lastUpdated: null, icon: '👔' },
      { name: 'Research', count: 0, lastUpdated: null, icon: '🔍' },
      { name: 'Zodiac', count: 0, lastUpdated: null, icon: '✨' }
    ];
    
    // Count explicit vs professional
    const professional = photoList.filter(p => !p.isExplicit);
    const research = photoList.filter(p => p.isExplicit);
    
    // Find the most recent date for each category
    const professionalLastUpdate = professional.length > 0 
      ? new Date(Math.max(...professional.map(p => p.lastModified.getTime()))) 
      : null;
      
    const researchLastUpdate = research.length > 0
      ? new Date(Math.max(...research.map(p => p.lastModified.getTime())))
      : null;
      
    // Update counts and dates
    baseCategories[1].count = professional.length;
    baseCategories[1].lastUpdated = professionalLastUpdate;
    
    baseCategories[2].count = research.length;
    baseCategories[2].lastUpdated = researchLastUpdate;
    
    // Count zodiac photos (containing zodiac name in filename)
    const zodiacNames = ['aries', 'taurus', 'gemini', 'cancer', 'leo', 'virgo', 
      'libra', 'scorpio', 'sagittarius', 'capricorn', 'aquarius', 'pisces'];
      
    const zodiacPhotos = photoList.filter(p => 
      zodiacNames.some(z => p.name.toLowerCase().includes(z))
    );
    
    const zodiacLastUpdate = zodiacPhotos.length > 0
      ? new Date(Math.max(...zodiacPhotos.map(p => p.lastModified.getTime())))
      : null;
      
    baseCategories[3].count = zodiacPhotos.length;
    baseCategories[3].lastUpdated = zodiacLastUpdate;
    
    setCategories(baseCategories);
    
    // Select the "All Photos" category by default
    setSelectedCategory('All Photos');
  };

  const handleCategorySelect = (category: string) => {
    setSelectedCategory(category);
    setSelectedPhoto(null);
  };

  const handlePhotoSelect = (photo: PhotoFile) => {
    setSelectedPhoto(photo);
  };

  const handlePhotoDelete = async (photo: PhotoFile) => {
    if (!window.confirm(`Delete "${photo.name}"? This cannot be undone.`)) {
      return;
    }
    
    try {
      // @ts-ignore - solaApp is defined in globals.d.ts
      if (window.solaApp?.deletePhoto) {
        // @ts-ignore - solaApp is defined in globals.d.ts
        await window.solaApp.deletePhoto(photo.path);
      }
      
      // Update state after deletion
      setPhotos(photos.filter(p => p.path !== photo.path));
      
      // Regenerate categories
      generateCategories(photos.filter(p => p.path !== photo.path));
      
      // Clear selection if needed
      if (selectedPhoto?.path === photo.path) {
        setSelectedPhoto(null);
      }
    } catch (err) {
      console.error('Error deleting photo:', err);
      setError('Failed to delete photo');
    }
  };

  const handleExport = async (photo: PhotoFile) => {
    try {
      // @ts-ignore - solaApp is defined in globals.d.ts
      if (window.solaApp?.exportPhoto) {
        // @ts-ignore - solaApp is defined in globals.d.ts
        await window.solaApp.exportPhoto(photo.path);
      }
    } catch (err) {
      console.error('Error exporting photo:', err);
      setError('Failed to export photo');
    }
  };

  const filteredPhotos = () => {
    let filtered = [...photos];
    
    // Filter by category
    if (selectedCategory === 'Professional') {
      filtered = filtered.filter(p => !p.isExplicit);
    } else if (selectedCategory === 'Research') {
      filtered = filtered.filter(p => p.isExplicit);
    } else if (selectedCategory === 'Zodiac') {
      const zodiacNames = ['aries', 'taurus', 'gemini', 'cancer', 'leo', 'virgo', 
        'libra', 'scorpio', 'sagittarius', 'capricorn', 'aquarius', 'pisces'];
        
      filtered = filtered.filter(p => 
        zodiacNames.some(z => p.name.toLowerCase().includes(z))
      );
    }
    
    // Filter by search query
    if (searchQuery) {
      const query = searchQuery.toLowerCase();
      filtered = filtered.filter(p => 
        p.name.toLowerCase().includes(query)
      );
    }
    
    // Sort photos
    if (sortOrder === 'newest') {
      filtered.sort((a, b) => b.lastModified.getTime() - a.lastModified.getTime());
    } else if (sortOrder === 'oldest') {
      filtered.sort((a, b) => a.lastModified.getTime() - b.lastModified.getTime());
    } else if (sortOrder === 'name') {
      filtered.sort((a, b) => a.name.localeCompare(b.name));
    }
    
    return filtered;
  };

  const formatFileSize = (bytes: number) => {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  };

  const renderCompactView = () => (
    <div className="flex flex-col border border-border-dark rounded-lg bg-panel-dark">
      <div className="p-3 border-b border-border-dark flex justify-between items-center">
        <h3 className="font-semibold text-sm flex items-center gap-1">
          <span>📁</span>
          <span>Photo Vault</span>
        </h3>
        <span className="text-xs text-gray-400">{photos.length} files</span>
      </div>
      <div className="p-3 text-xs text-gray-400 flex flex-wrap gap-2">
        {categories.map(cat => (
          <div key={cat.name} className="flex items-center gap-1">
            <span>{cat.icon}</span>
            <span>{cat.name}: {cat.count}</span>
          </div>
        ))}
      </div>
    </div>
  );

  // Main render
  if (compact) return renderCompactView();

  return (
    <div className="bg-panel-dark border border-border-dark rounded-lg overflow-hidden flex flex-col h-[600px]">
      {/* Header */}
      <div className="p-3 border-b border-border-dark flex justify-between items-center">
        <div className="font-semibold flex items-center gap-2">
          <span className="text-lg">📁</span>
          <span>Photo Vault</span>
        </div>
        <div className="text-xs text-gray-400">
          {photos.length} files • {formatFileSize(photos.reduce((acc, p) => acc + p.size, 0))}
        </div>
      </div>
      
      {/* Search & Sort Controls */}
      <div className="p-3 border-b border-border-dark flex flex-col md:flex-row gap-2">
        <div className="relative flex-grow">
          <input
            type="text"
            className="w-full bg-input-bg border border-border-dark rounded px-3 py-1 pl-8 text-sm focus:outline-none focus:border-blue-500"
            placeholder="Search photos..."
            value={searchQuery}
            onChange={e => setSearchQuery(e.target.value)}
          />
          <span className="absolute left-3 top-2 text-gray-400">🔍</span>
        </div>
        <select
          className="bg-input-bg border border-border-dark rounded px-2 py-1 text-sm focus:outline-none"
          value={sortOrder}
          onChange={e => setSortOrder(e.target.value as 'newest' | 'oldest' | 'name')}
        >
          <option value="newest">Newest First</option>
          <option value="oldest">Oldest First</option>
          <option value="name">By Name</option>
        </select>
      </div>
      
      {/* Main content */}
      <div className="flex flex-grow overflow-hidden">
        {/* Categories sidebar */}
        <div className="w-40 border-r border-border-dark overflow-y-auto bg-panel-darker">
          {categories.map(category => (
            <button
              key={category.name}
              className={`w-full text-left p-3 text-sm hover:bg-blue-500/10 flex items-center gap-2
                ${selectedCategory === category.name ? 'bg-blue-500/10 border-l-2 border-blue-500' : ''}
              `}
              onClick={() => handleCategorySelect(category.name)}
            >
              <span>{category.icon}</span>
              <div>
                <div>{category.name}</div>
                <div className="text-xs text-gray-400">{category.count}</div>
              </div>
            </button>
          ))}
        </div>
        
        {/* Photos grid */}
        <div className="flex-grow overflow-y-auto p-3 h-full">
          {loading ? (
            <div className="flex h-full items-center justify-center">
              <span>Loading photos...</span>
            </div>
          ) : error ? (
            <div className="flex h-full items-center justify-center text-red-400">
              {error}
            </div>
          ) : filteredPhotos().length === 0 ? (
            <div className="flex h-full items-center justify-center text-gray-400">
              No photos found
            </div>
          ) : (
            <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3">
              {filteredPhotos().map((photo) => (
                <div
                  key={photo.path}
                  className={`relative border border-border-dark rounded overflow-hidden cursor-pointer
                    ${selectedPhoto?.path === photo.path ? 'ring-2 ring-blue-500' : ''}
                    ${photo.isExplicit ? 'border-red-500/30' : ''}
                  `}
                  onClick={() => handlePhotoSelect(photo)}
                >
                  {/* Photo placeholder - in a real app, this would be an actual image */}
                  <div className="aspect-square bg-gray-700 flex items-center justify-center">
                    <span className="text-2xl">{photo.isExplicit ? '🔍' : '👔'}</span>
                  </div>
                  <div className="p-2">
                    <div className="text-xs truncate">{photo.name}</div>
                    <div className="flex justify-between text-xs text-gray-400">
                      <div>{formatFileSize(photo.size)}</div>
                      {photo.isExplicit && <span className="text-red-400">Research</span>}
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
        
        {/* Photo details sidebar */}
        {selectedPhoto && (
          <div className="w-60 border-l border-border-dark p-3 overflow-y-auto bg-panel-darker">
            <div className="mb-3 font-semibold">{selectedPhoto.name}</div>
            
            {/* Preview */}
            <div className="aspect-square bg-gray-700 mb-3 flex items-center justify-center">
              <span className="text-5xl">{selectedPhoto.isExplicit ? '🔍' : '👔'}</span>
            </div>
            
            {/* Details */}
            <div className="mb-4 space-y-2 text-sm">
              <div className="flex justify-between">
                <span className="text-gray-400">Type:</span>
                <span>{selectedPhoto.isExplicit ? 'Research' : 'Professional'}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-400">Size:</span>
                <span>{formatFileSize(selectedPhoto.size)}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-400">Created:</span>
                <span>{selectedPhoto.lastModified.toLocaleDateString()}</span>
              </div>
            </div>
            
            {/* Actions */}
            <div className="space-y-2">
              <button
                className="w-full py-1 bg-blue-500 hover:bg-blue-600 rounded text-sm"
                onClick={() => handleExport(selectedPhoto)}
              >
                Export
              </button>
              <button
                className="w-full py-1 bg-red-500/20 hover:bg-red-500/30 text-red-400 rounded text-sm"
                onClick={() => handlePhotoDelete(selectedPhoto)}
              >
                Delete
              </button>
            </div>
          </div>
        )}
      </div>
      
      {/* Status bar */}
      <div className="p-2 border-t border-border-dark flex justify-between items-center text-xs text-gray-400">
        <div>
          {filteredPhotos().length} photos in {selectedCategory}
        </div>
        <div>
          Last updated: {
            categories.find(c => c.name === selectedCategory)?.lastUpdated?.toLocaleDateString() || 'Never'
          }
        </div>
      </div>
    </div>
  );
};

export default PhotoVault;