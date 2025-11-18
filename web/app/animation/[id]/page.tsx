"use client";

import { useState } from "react";
import { useParams } from "next/navigation";
import Link from "next/link";
import Image from "next/image";
import Button from "@/components/Button";
import AnimationPlayer from "@/components/AnimationPlayer";
import AnimationCard from "@/components/AnimationCard";

export default function AnimationDetailPage() {
  const params = useParams();
  const animationId = params.id as string;
  const [isLiked, setIsLiked] = useState(false);

  // Sample animation data
  const animation = {
    id: animationId,
    title: "Dancing Robot in Cyberpunk City",
    description:
      "A futuristic robot performing an energetic dance routine in a neon-lit cyberpunk cityscape. Features smooth movements, dynamic lighting, and vibrant colors that capture the essence of a high-tech urban environment.",
    videoUrl: "https://www.w3schools.com/html/mov_bbb.mp4",
    thumbnail: "https://picsum.photos/seed/anime1/1920/1080",
    creator: {
      username: "techartist",
      avatar: "https://picsum.photos/seed/user1/100/100",
      bio: "Digital artist specializing in cyberpunk and sci-fi animations",
    },
    views: 12500,
    likes: 856,
    createdAt: "2024-11-15",
    tags: ["Cyberpunk", "Robot", "Dance", "Neon", "Sci-Fi"],
    duration: "5 seconds",
    resolution: "1920x1080",
    fps: 30,
  };

  // Related animations
  const relatedAnimations = [
    {
      id: "2",
      title: "Neon City Skyline Timelapse",
      thumbnail: "https://picsum.photos/seed/anime7/800/450",
      creator: { username: "cityscape", avatar: "https://picsum.photos/seed/user7/100/100" },
      views: 8900,
      likes: 654,
    },
    {
      id: "3",
      title: "Mecha Warrior Transformation",
      thumbnail: "https://picsum.photos/seed/anime5/800/450",
      creator: { username: "mechamaster", avatar: "https://picsum.photos/seed/user5/100/100" },
      views: 15200,
      likes: 1203,
    },
    {
      id: "4",
      title: "Futuristic Hovercar Chase",
      thumbnail: "https://picsum.photos/seed/anime8/800/450",
      creator: { username: "speedfan", avatar: "https://picsum.photos/seed/user8/100/100" },
      views: 11300,
      likes: 892,
    },
  ];

  const handleLike = () => {
    setIsLiked(!isLiked);
  };

  const handleDownload = () => {
    alert("Download started! (This is a demo)");
  };

  return (
    <div className="py-12">
      <div className="brutal-container">
        <div className="grid lg:grid-cols-3 gap-8">
          {/* Main Content */}
          <div className="lg:col-span-2">
            {/* Video Player */}
            <div className="mb-6">
              <AnimationPlayer
                videoUrl={animation.videoUrl}
                poster={animation.thumbnail}
              />
            </div>

            {/* Title and Actions */}
            <div className="mb-6">
              <h1 className="text-4xl mb-4">{animation.title}</h1>

              <div className="flex flex-wrap items-center gap-4 mb-4">
                <div className="flex items-center gap-2 text-gray-600">
                  <span className="font-bold">👁 {animation.views.toLocaleString()}</span>
                  <span>•</span>
                  <span className="font-bold">
                    {new Date(animation.createdAt).toLocaleDateString()}
                  </span>
                </div>
              </div>

              <div className="flex flex-wrap gap-3">
                <Button
                  variant={isLiked ? "primary" : "outline"}
                  size="md"
                  onClick={handleLike}
                >
                  {isLiked ? "❤️" : "🤍"} {animation.likes + (isLiked ? 1 : 0)}
                </Button>
                <Button variant="primary" size="md" onClick={handleDownload}>
                  ⬇ DOWNLOAD
                </Button>
                <Button variant="outline" size="md">
                  🔗 SHARE
                </Button>
              </div>
            </div>

            {/* Description */}
            <div className="brutal-card p-6 mb-6">
              <h3 className="text-2xl mb-4">DESCRIPTION</h3>
              <p className="text-lg leading-relaxed mb-4">
                {animation.description}
              </p>

              <div className="flex flex-wrap gap-2">
                {animation.tags.map((tag) => (
                  <span
                    key={tag}
                    className="px-4 py-2 bg-accent border-brutal border-dark font-bold text-sm"
                  >
                    #{tag}
                  </span>
                ))}
              </div>
            </div>

            {/* Technical Details */}
            <div className="brutal-card p-6">
              <h3 className="text-2xl mb-4">TECHNICAL DETAILS</h3>
              <div className="grid md:grid-cols-3 gap-4">
                <div>
                  <div className="text-sm text-gray-600 font-bold uppercase">
                    Duration
                  </div>
                  <div className="text-lg font-bold">{animation.duration}</div>
                </div>
                <div>
                  <div className="text-sm text-gray-600 font-bold uppercase">
                    Resolution
                  </div>
                  <div className="text-lg font-bold">{animation.resolution}</div>
                </div>
                <div>
                  <div className="text-sm text-gray-600 font-bold uppercase">
                    Frame Rate
                  </div>
                  <div className="text-lg font-bold">{animation.fps} FPS</div>
                </div>
              </div>
            </div>
          </div>

          {/* Sidebar */}
          <div className="lg:col-span-1">
            {/* Creator Info */}
            <div className="brutal-card p-6 mb-6">
              <h3 className="text-xl mb-4">CREATOR</h3>

              <div className="flex items-center gap-3 mb-4">
                {animation.creator.avatar ? (
                  <div className="relative w-16 h-16 rounded-full overflow-hidden border-brutal border-dark">
                    <Image
                      src={animation.creator.avatar}
                      alt={animation.creator.username}
                      fill
                      className="object-cover"
                    />
                  </div>
                ) : (
                  <div className="w-16 h-16 rounded-full bg-accent border-brutal border-dark flex items-center justify-center">
                    <span className="text-2xl font-bold">
                      {animation.creator.username[0].toUpperCase()}
                    </span>
                  </div>
                )}
                <div>
                  <Link
                    href={`/profile/${animation.creator.username}`}
                    className="font-bold text-xl hover:text-primary transition-colors"
                  >
                    {animation.creator.username}
                  </Link>
                </div>
              </div>

              <p className="text-gray-600 mb-4">{animation.creator.bio}</p>

              <Link href={`/profile/${animation.creator.username}`}>
                <Button variant="primary" size="md" className="w-full">
                  VIEW PROFILE
                </Button>
              </Link>
            </div>

            {/* Related Animations */}
            <div>
              <h3 className="text-2xl mb-4">RELATED ANIMATIONS</h3>
              <div className="space-y-4">
                {relatedAnimations.map((related) => (
                  <AnimationCard key={related.id} {...related} />
                ))}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
