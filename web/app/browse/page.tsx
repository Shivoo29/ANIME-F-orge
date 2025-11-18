"use client";

import { useState } from "react";
import AnimationCard from "@/components/AnimationCard";
import Button from "@/components/Button";

const CATEGORIES = [
  "All",
  "Anime",
  "Fantasy",
  "Sci-Fi",
  "Action",
  "Nature",
  "Abstract",
];

export default function BrowsePage() {
  const [selectedCategory, setSelectedCategory] = useState("All");
  const [searchQuery, setSearchQuery] = useState("");

  // Sample animations data
  const animations = [
    {
      id: "1",
      title: "Dancing Robot in Cyberpunk City",
      thumbnail: "https://picsum.photos/seed/anime1/800/450",
      creator: { username: "techartist", avatar: "https://picsum.photos/seed/user1/100/100" },
      views: 12500,
      likes: 856,
      category: "Sci-Fi",
    },
    {
      id: "2",
      title: "Magical Girl Transformation Sequence",
      thumbnail: "https://picsum.photos/seed/anime2/800/450",
      creator: { username: "magicmaker", avatar: "https://picsum.photos/seed/user2/100/100" },
      views: 8900,
      likes: 654,
      category: "Anime",
    },
    {
      id: "3",
      title: "Epic Dragon Battle in the Clouds",
      thumbnail: "https://picsum.photos/seed/anime3/800/450",
      creator: { username: "dragonlord", avatar: "https://picsum.photos/seed/user3/100/100" },
      views: 15200,
      likes: 1203,
      category: "Fantasy",
    },
    {
      id: "4",
      title: "Sakura Petals Falling in Ancient Temple",
      thumbnail: "https://picsum.photos/seed/anime4/800/450",
      creator: { username: "zenmaster", avatar: "https://picsum.photos/seed/user4/100/100" },
      views: 9800,
      likes: 721,
      category: "Nature",
    },
    {
      id: "5",
      title: "Mecha Warrior vs Alien Invasion",
      thumbnail: "https://picsum.photos/seed/anime5/800/450",
      creator: { username: "mechaenthusiast", avatar: "https://picsum.photos/seed/user5/100/100" },
      views: 18500,
      likes: 1456,
      category: "Action",
    },
    {
      id: "6",
      title: "Neon Geometric Patterns Morphing",
      thumbnail: "https://picsum.photos/seed/anime6/800/450",
      creator: { username: "abstractart", avatar: "https://picsum.photos/seed/user6/100/100" },
      views: 7200,
      likes: 512,
      category: "Abstract",
    },
    {
      id: "7",
      title: "Space Station Orbiting Blue Planet",
      thumbnail: "https://picsum.photos/seed/anime7/800/450",
      creator: { username: "spacefan", avatar: "https://picsum.photos/seed/user7/100/100" },
      views: 11300,
      likes: 892,
      category: "Sci-Fi",
    },
    {
      id: "8",
      title: "Ninja Assassin Stealth Mission",
      thumbnail: "https://picsum.photos/seed/anime8/800/450",
      creator: { username: "ninjamaster", avatar: "https://picsum.photos/seed/user8/100/100" },
      views: 14600,
      likes: 1134,
      category: "Action",
    },
    {
      id: "9",
      title: "Underwater Coral Reef Paradise",
      thumbnail: "https://picsum.photos/seed/anime9/800/450",
      creator: { username: "oceanexplorer", avatar: "https://picsum.photos/seed/user9/100/100" },
      views: 8500,
      likes: 678,
      category: "Nature",
    },
  ];

  const filteredAnimations = animations.filter((animation) => {
    const matchesCategory =
      selectedCategory === "All" || animation.category === selectedCategory;
    const matchesSearch = animation.title
      .toLowerCase()
      .includes(searchQuery.toLowerCase());
    return matchesCategory && matchesSearch;
  });

  return (
    <div className="py-12">
      <div className="brutal-container">
        {/* Header */}
        <div className="mb-8">
          <h1 className="mb-4">BROWSE ANIMATIONS</h1>
          <p className="text-xl">
            Explore thousands of animations created by our community
          </p>
        </div>

        {/* Search Bar */}
        <div className="mb-8">
          <input
            type="text"
            placeholder="SEARCH ANIMATIONS..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="brutal-input w-full text-lg uppercase placeholder:text-gray-500"
          />
        </div>

        {/* Category Filter */}
        <div className="mb-8 flex flex-wrap gap-3">
          {CATEGORIES.map((category) => (
            <Button
              key={category}
              variant={selectedCategory === category ? "primary" : "outline"}
              size="sm"
              onClick={() => setSelectedCategory(category)}
            >
              {category}
            </Button>
          ))}
        </div>

        {/* Results Count */}
        <div className="mb-6">
          <p className="text-lg font-bold">
            {filteredAnimations.length} ANIMATION
            {filteredAnimations.length !== 1 ? "S" : ""} FOUND
          </p>
        </div>

        {/* Animations Grid */}
        {filteredAnimations.length > 0 ? (
          <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
            {filteredAnimations.map((animation) => (
              <AnimationCard
                key={animation.id}
                id={animation.id}
                title={animation.title}
                thumbnail={animation.thumbnail}
                creator={animation.creator}
                views={animation.views}
                likes={animation.likes}
              />
            ))}
          </div>
        ) : (
          <div className="text-center py-20">
            <div className="text-6xl mb-4">🔍</div>
            <h3 className="text-2xl mb-2">NO ANIMATIONS FOUND</h3>
            <p className="text-lg text-gray-600">
              Try a different search or category
            </p>
          </div>
        )}

        {/* Load More Button */}
        {filteredAnimations.length > 0 && (
          <div className="text-center mt-12">
            <Button variant="primary" size="lg">
              Load More Animations
            </Button>
          </div>
        )}
      </div>
    </div>
  );
}
