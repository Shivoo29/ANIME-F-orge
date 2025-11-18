"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import Button from "@/components/Button";
import AnimationCard from "@/components/AnimationCard";
import { useAuthStore } from "@/store/authStore";

export default function DashboardPage() {
  const router = useRouter();
  const { user } = useAuthStore();
  const [showUploadModal, setShowUploadModal] = useState(false);
  const [prompt, setPrompt] = useState("");
  const [isGenerating, setIsGenerating] = useState(false);

  // Redirect if not logged in
  if (!user) {
    router.push("/login");
    return null;
  }

  // Sample user animations
  const userAnimations = [
    {
      id: "1",
      title: "My First Animation",
      thumbnail: "https://picsum.photos/seed/myanim1/800/450",
      creator: { username: user.username, avatar: user.avatar },
      views: 234,
      likes: 45,
    },
    {
      id: "2",
      title: "Epic Battle Scene",
      thumbnail: "https://picsum.photos/seed/myanim2/800/450",
      creator: { username: user.username, avatar: user.avatar },
      views: 1256,
      likes: 189,
    },
  ];

  const handleGenerate = async () => {
    if (!prompt.trim()) {
      alert("Please enter a description for your animation");
      return;
    }

    setIsGenerating(true);

    // Simulate animation generation
    setTimeout(() => {
      setIsGenerating(false);
      setShowUploadModal(false);
      setPrompt("");
      alert("Animation generated successfully! (This is a demo)");
    }, 3000);
  };

  return (
    <div className="py-12">
      <div className="brutal-container">
        {/* Welcome Header */}
        <div className="mb-12">
          <h1 className="mb-2">WELCOME BACK, {user.username.toUpperCase()}!</h1>
          <p className="text-xl">
            Manage your animations and create new ones
          </p>
        </div>

        {/* Stats Overview */}
        <div className="grid md:grid-cols-4 gap-6 mb-12">
          <div className="brutal-card p-6 bg-primary text-white">
            <div className="text-4xl font-bold mb-2">{userAnimations.length}</div>
            <div className="text-lg uppercase font-bold">Animations</div>
          </div>

          <div className="brutal-card p-6 bg-secondary text-white">
            <div className="text-4xl font-bold mb-2">
              {userAnimations.reduce((acc, a) => acc + a.views, 0).toLocaleString()}
            </div>
            <div className="text-lg uppercase font-bold">Total Views</div>
          </div>

          <div className="brutal-card p-6 bg-accent">
            <div className="text-4xl font-bold mb-2">
              {userAnimations.reduce((acc, a) => acc + a.likes, 0)}
            </div>
            <div className="text-lg uppercase font-bold">Total Likes</div>
          </div>

          <div className="brutal-card p-6 bg-white">
            <div className="text-4xl font-bold mb-2">✨</div>
            <div className="text-lg uppercase font-bold">Pro Member</div>
          </div>
        </div>

        {/* Quick Actions */}
        <div className="mb-12">
          <h2 className="text-3xl mb-6">QUICK ACTIONS</h2>
          <div className="grid md:grid-cols-2 gap-6">
            <button
              onClick={() => setShowUploadModal(true)}
              className="brutal-card-hover p-8 text-left"
            >
              <div className="text-5xl mb-4">🎬</div>
              <h3 className="text-2xl mb-2">CREATE NEW ANIMATION</h3>
              <p className="text-gray-600">
                Use AI to generate stunning animations from text
              </p>
            </button>

            <button
              onClick={() => router.push("/browse")}
              className="brutal-card-hover p-8 text-left"
            >
              <div className="text-5xl mb-4">🔍</div>
              <h3 className="text-2xl mb-2">BROWSE GALLERY</h3>
              <p className="text-gray-600">
                Explore animations from the community
              </p>
            </button>
          </div>
        </div>

        {/* User's Animations */}
        <div>
          <div className="flex items-center justify-between mb-6">
            <h2 className="text-3xl">YOUR ANIMATIONS</h2>
            <Button
              variant="primary"
              size="md"
              onClick={() => setShowUploadModal(true)}
            >
              + New Animation
            </Button>
          </div>

          {userAnimations.length > 0 ? (
            <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
              {userAnimations.map((animation) => (
                <AnimationCard key={animation.id} {...animation} />
              ))}
            </div>
          ) : (
            <div className="brutal-card p-12 text-center">
              <div className="text-6xl mb-4">🎨</div>
              <h3 className="text-2xl mb-2">NO ANIMATIONS YET</h3>
              <p className="text-lg text-gray-600 mb-6">
                Create your first animation to get started!
              </p>
              <Button
                variant="primary"
                size="lg"
                onClick={() => setShowUploadModal(true)}
              >
                Create Animation
              </Button>
            </div>
          )}
        </div>
      </div>

      {/* Upload Modal */}
      {showUploadModal && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
          <div className="brutal-card bg-white max-w-2xl w-full p-8">
            <div className="flex items-center justify-between mb-6">
              <h2 className="text-3xl">CREATE NEW ANIMATION</h2>
              <button
                onClick={() => setShowUploadModal(false)}
                className="text-4xl font-bold hover:text-primary transition-colors"
              >
                ×
              </button>
            </div>

            <div className="space-y-6">
              <div>
                <label className="block font-bold mb-2 uppercase">
                  Describe Your Animation
                </label>
                <textarea
                  value={prompt}
                  onChange={(e) => setPrompt(e.target.value)}
                  className="brutal-input w-full h-40 resize-none"
                  placeholder="A magical girl transforming with sparkles and ribbons in an anime style..."
                  disabled={isGenerating}
                />
                <p className="text-sm text-gray-600 mt-2">
                  Be as detailed as possible. Include style, colors, mood, and
                  actions.
                </p>
              </div>

              <div className="grid md:grid-cols-2 gap-4">
                <div>
                  <label className="block font-bold mb-2 uppercase text-sm">
                    Style
                  </label>
                  <select className="brutal-input w-full" disabled={isGenerating}>
                    <option>Anime</option>
                    <option>Realistic</option>
                    <option>Cartoon</option>
                    <option>3D</option>
                  </select>
                </div>

                <div>
                  <label className="block font-bold mb-2 uppercase text-sm">
                    Duration
                  </label>
                  <select className="brutal-input w-full" disabled={isGenerating}>
                    <option>3 seconds</option>
                    <option>5 seconds</option>
                    <option>10 seconds</option>
                    <option>15 seconds</option>
                  </select>
                </div>
              </div>

              <div className="flex gap-4">
                <Button
                  variant="primary"
                  size="lg"
                  className="flex-1"
                  onClick={handleGenerate}
                  disabled={isGenerating}
                >
                  {isGenerating ? "GENERATING..." : "GENERATE ANIMATION"}
                </Button>
                <Button
                  variant="outline"
                  size="lg"
                  onClick={() => setShowUploadModal(false)}
                  disabled={isGenerating}
                >
                  CANCEL
                </Button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
