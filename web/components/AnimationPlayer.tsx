"use client";

import { useRef, useState } from "react";

interface AnimationPlayerProps {
  videoUrl: string;
  poster?: string;
}

export default function AnimationPlayer({
  videoUrl,
  poster,
}: AnimationPlayerProps) {
  const videoRef = useRef<HTMLVideoElement>(null);
  const [isPlaying, setIsPlaying] = useState(false);

  const togglePlay = () => {
    if (videoRef.current) {
      if (isPlaying) {
        videoRef.current.pause();
      } else {
        videoRef.current.play();
      }
      setIsPlaying(!isPlaying);
    }
  };

  return (
    <div className="brutal-card overflow-hidden">
      <div className="relative aspect-video bg-black group">
        <video
          ref={videoRef}
          src={videoUrl}
          poster={poster}
          className="w-full h-full object-contain"
          controls
          onPlay={() => setIsPlaying(true)}
          onPause={() => setIsPlaying(false)}
        />

        {!isPlaying && (
          <button
            onClick={togglePlay}
            className="absolute inset-0 flex items-center justify-center bg-black/30 group-hover:bg-black/40 transition-all"
          >
            <div className="w-20 h-20 bg-primary border-brutal border-dark shadow-brutal flex items-center justify-center">
              <svg
                className="w-10 h-10 text-white ml-1"
                fill="currentColor"
                viewBox="0 0 24 24"
              >
                <path d="M8 5v14l11-7z" />
              </svg>
            </div>
          </button>
        )}
      </div>
    </div>
  );
}
