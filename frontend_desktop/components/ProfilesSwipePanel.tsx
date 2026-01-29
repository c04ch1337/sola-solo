import React, { useMemo, useState, useEffect } from 'react';
import { useAtom } from 'jotai';
import TrustProgressBar from './TrustProgressBar';
import { predictedTrustGainAtom, trustScoreAtom, zodiacSignAtom, computeTrustPhase } from '../stores/trustStore';

interface ProfilePhoto {
  id: string;
  url: string;
  is_explicit: boolean;
  prompt: string;
  generated_at: number;
}

interface GeneratedProfile {
  id: string;
  name: string;
  age: number;
  bio: string;
  interests: string[];
  kinks: string[];
  photos: ProfilePhoto[];
  personality_traits: string[];
  intimacy_level: string;
  created_at: number;
}

interface ProfilesSwipePanelProps {
  onClose: () => void;
  backendUrl: string;
}

const ProfilesSwipePanel: React.FC<ProfilesSwipePanelProps> = ({ onClose, backendUrl }) => {
  const [profiles, setProfiles] = useState<GeneratedProfile[]>([]);
  const [currentIndex, setCurrentIndex] = useState(0);
  const [currentPhotoIndex, setCurrentPhotoIndex] = useState(0);
  const [loading, setLoading] = useState(false);
  const [generating, setGenerating] = useState(false);
  const [showExplicit, setShowExplicit] = useState(false);
  const [matches, setMatches] = useState<GeneratedProfile[]>([]);

  // L7 trust (TODO: wire to backend); currently local store.
  const [trustScore, setTrustScore] = useAtom(trustScoreAtom);
  const [predictedGain] = useAtom(predictedTrustGainAtom);
  const [zodiacSign] = useAtom(zodiacSignAtom);

  const trustPhase = useMemo(() => computeTrustPhase(trustScore), [trustScore]);
  const intimateUnlocked = trustScore >= 0.8;

  useEffect(() => {
    loadProfiles();
  }, []);

  const loadProfiles = async () => {
    setLoading(true);
    try {
      const response = await fetch(`${backendUrl}/api/profiles/list`);
      const data = await response.json();
      setProfiles(data.profiles || []);
    } catch (error) {
      console.error('Failed to load profiles:', error);
    } finally {
      setLoading(false);
    }
  };

  const generateProfile = async () => {
    setGenerating(true);
    try {
      const response = await fetch(`${backendUrl}/api/profiles/generate`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          intimacy_level: 'explicit',
          preferred_traits: ['adventurous', 'kinky', 'open-minded'],
          kink_preferences: ['bondage', 'roleplay', 'dominance'],
          photo_count: 10,
          explicit_photo_ratio: 0.6,
        }),
      });
      const data = await response.json();
      if (data.success) {
        setProfiles([...profiles, data.profile]);
      }
    } catch (error) {
      console.error('Failed to generate profile:', error);
    } finally {
      setGenerating(false);
    }
  };

  const handleSwipe = (direction: 'left' | 'right') => {
    const currentProfile = profiles[currentIndex];
    
    if (direction === 'right' && currentProfile) {
      // Match! Add to matches and trigger intimate chat
      setMatches([...matches, currentProfile]);
      // TODO: Trigger chat with intimate/kink context
      console.log('Match!', currentProfile);

      // Demo: positive interaction increases trust.
      setTrustScore((t) => Math.min(1, t + 0.06));
    }

    // Move to next profile
    setCurrentIndex(currentIndex + 1);
    setCurrentPhotoIndex(0);
  };

  const nextPhoto = () => {
    const currentProfile = profiles[currentIndex];
    if (currentProfile && currentPhotoIndex < currentProfile.photos.length - 1) {
      setCurrentPhotoIndex(currentPhotoIndex + 1);
    }
  };

  const prevPhoto = () => {
    if (currentPhotoIndex > 0) {
      setCurrentPhotoIndex(currentPhotoIndex - 1);
    }
  };

  const currentProfile = profiles[currentIndex];
  const currentPhoto = currentProfile?.photos[currentPhotoIndex];

  const canViewExplicit = showExplicit || intimateUnlocked;

  if (loading) {
    return (
      <div className="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center">
        <div className="bg-panel-dark border border-border-dark rounded-2xl p-8 max-w-md w-full">
          <div className="flex items-center justify-center gap-3">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
            <span className="text-white font-medium">Loading profiles...</span>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4">
      <div className="bg-panel-dark border border-border-dark rounded-2xl shadow-2xl max-w-2xl w-full max-h-[90vh] overflow-hidden flex flex-col">
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b border-border-dark">
          <div className="flex items-center gap-3">
            <span className="material-symbols-outlined text-primary text-2xl">favorite</span>
            <h2 className="text-xl font-bold text-white">Profiles</h2>
            <span className="text-xs text-slate-500 font-mono">
              {currentIndex + 1} / {profiles.length}
            </span>
          </div>
          <div className="flex items-center gap-2">
            <div className="hidden md:block w-[320px]">
              <TrustProgressBar
                trustScore={trustScore}
                predictedGain={predictedGain}
                zodiacSign={zodiacSign}
                onPhaseUnlocked={(phase) => {
                  // Small, local-only effect; can be replaced with toast/notification service.
                  console.log('Phase unlocked:', phase);
                }}
              />
            </div>
            <button
              onClick={generateProfile}
              disabled={generating}
              className="px-3 py-1.5 bg-primary/20 hover:bg-primary/30 text-primary rounded-lg text-sm font-medium transition-colors disabled:opacity-50"
            >
              {generating ? 'Generating...' : '+ Generate'}
            </button>
            <button
              onClick={onClose}
              className="p-2 hover:bg-slate-700 rounded-lg transition-colors"
            >
              <span className="material-symbols-outlined text-slate-400">close</span>
            </button>
          </div>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto custom-scrollbar p-6">
          {!currentProfile ? (
            <div className="flex flex-col items-center justify-center h-full gap-4">
              <span className="material-symbols-outlined text-slate-600 text-6xl">person_off</span>
              <p className="text-slate-400 text-center">
                No more profiles. Generate new ones to continue.
              </p>
              <button
                onClick={generateProfile}
                disabled={generating}
                className="px-6 py-3 bg-primary hover:bg-primary/90 text-white rounded-xl font-medium transition-colors disabled:opacity-50"
              >
                {generating ? 'Generating...' : 'Generate Profile'}
              </button>
            </div>
          ) : (
            <div className="space-y-6">
              {/* Photo Display */}
              <div className="relative aspect-[3/4] bg-slate-800 rounded-xl overflow-hidden group">
                {currentPhoto && (
                  <>
                    <img
                      src={currentPhoto.url}
                      alt={`${currentProfile.name} - Photo ${currentPhotoIndex + 1}`}
                      className={`w-full h-full object-cover transition-all duration-300 ${
                        currentPhoto.is_explicit && !canViewExplicit ? 'blur-2xl scale-105' : ''
                      }`}
                    />

                    {/* Explicit overlay gate */}
                    {currentPhoto.is_explicit && !canViewExplicit && (
                      <div className="absolute inset-0 flex items-center justify-center bg-slate-900/60 backdrop-blur-sm">
                        <div className="text-center space-y-3 px-6">
                          <div className="text-slate-200 font-semibold">Locked</div>
                          <div className="text-xs text-slate-400">
                            Reach <span className="text-violet-200 font-semibold">Intimate</span> (T ≥ 0.8)
                            to unlock.
                          </div>
                          <div className="text-[10px] text-slate-500 font-mono">
                            Current: {trustPhase} • T={(trustScore).toFixed(2)}
                          </div>
                          <button
                            onClick={() => setShowExplicit(true)}
                            className="px-4 py-2 bg-primary/20 hover:bg-primary/30 text-primary rounded-lg text-sm font-medium transition-colors"
                            title="Temporary override in demo UI; production should rely on L7 trust gate"
                          >
                            Request Reveal
                          </button>
                        </div>
                      </div>
                    )}
                    
                    {/* Photo Navigation */}
                    <div className="absolute inset-0 flex items-center justify-between p-4 opacity-0 group-hover:opacity-100 transition-opacity">
                      <button
                        onClick={prevPhoto}
                        disabled={currentPhotoIndex === 0}
                        className="p-2 bg-black/50 hover:bg-black/70 rounded-full transition-colors disabled:opacity-30"
                      >
                        <span className="material-symbols-outlined text-white">chevron_left</span>
                      </button>
                      <button
                        onClick={nextPhoto}
                        disabled={currentPhotoIndex === currentProfile.photos.length - 1}
                        className="p-2 bg-black/50 hover:bg-black/70 rounded-full transition-colors disabled:opacity-30"
                      >
                        <span className="material-symbols-outlined text-white">chevron_right</span>
                      </button>
                    </div>

                    {/* Photo Indicators */}
                    <div className="absolute bottom-4 left-0 right-0 flex justify-center gap-1">
                      {currentProfile.photos.map((_, idx) => (
                        <div
                          key={idx}
                          className={`h-1 rounded-full transition-all ${
                            idx === currentPhotoIndex
                              ? 'w-8 bg-primary'
                              : 'w-1 bg-white/30'
                          }`}
                        />
                      ))}
                    </div>
                  </>
                )}
              </div>

              {/* Profile Info */}
              <div className="space-y-4">
                <div>
                  <h3 className="text-2xl font-bold text-white">
                    {currentProfile.name}, {currentProfile.age}
                  </h3>
                  <p className="text-sm text-slate-400 mt-1">{currentProfile.bio}</p>
                </div>

                {/* Interests */}
                {currentProfile.interests.length > 0 && (
                  <div>
                    <h4 className="text-xs font-bold text-slate-500 uppercase tracking-wider mb-2">
                      Interests
                    </h4>
                    <div className="flex flex-wrap gap-2">
                      {currentProfile.interests.map((interest, idx) => (
                        <span
                          key={idx}
                          className="px-3 py-1 bg-slate-800 text-slate-300 rounded-full text-sm"
                        >
                          {interest}
                        </span>
                      ))}
                    </div>
                  </div>
                )}

                {/* Kinks */}
                {currentProfile.kinks.length > 0 && (
                  <div>
                    <h4 className="text-xs font-bold text-primary uppercase tracking-wider mb-2">
                      Kinks & Preferences
                    </h4>
                    <div className="flex flex-wrap gap-2">
                      {currentProfile.kinks.map((kink, idx) => (
                        <span
                          key={idx}
                          className="px-3 py-1 bg-primary/10 text-primary rounded-full text-sm border border-primary/30"
                        >
                          {kink}
                        </span>
                      ))}
                    </div>
                  </div>
                )}
              </div>

              {/* Swipe Actions */}
              <div className="flex items-center justify-center gap-6 pt-4">
                <button
                  onClick={() => handleSwipe('left')}
                  className="p-4 bg-red-500/20 hover:bg-red-500/30 text-red-500 rounded-full transition-colors"
                >
                  <span className="material-symbols-outlined text-3xl">close</span>
                </button>
                <button
                  onClick={() => handleSwipe('right')}
                  className="p-4 bg-green-500/20 hover:bg-green-500/30 text-green-500 rounded-full transition-colors"
                >
                  <span className="material-symbols-outlined text-3xl">favorite</span>
                </button>
              </div>
            </div>
          )}
        </div>

        {/* Matches Footer */}
        {matches.length > 0 && (
          <div className="border-t border-border-dark p-4">
            <div className="flex items-center gap-2">
              <span className="material-symbols-outlined text-green-500">check_circle</span>
              <span className="text-sm text-slate-400">
                {matches.length} match{matches.length !== 1 ? 'es' : ''}
              </span>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default ProfilesSwipePanel;
