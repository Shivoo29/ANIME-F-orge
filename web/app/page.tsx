import Link from "next/link";
import Button from "@/components/Button";
import AnimationCard from "@/components/AnimationCard";

export default function Home() {
  // Sample animations for showcase
  const sampleAnimations = [
    {
      id: "1",
      title: "Dancing Robot in Cyberpunk City",
      thumbnail: "https://picsum.photos/seed/anime1/800/450",
      creator: { username: "techartist", avatar: "https://picsum.photos/seed/user1/100/100" },
      views: 12500,
      likes: 856,
    },
    {
      id: "2",
      title: "Magical Girl Transformation Sequence",
      thumbnail: "https://picsum.photos/seed/anime2/800/450",
      creator: { username: "magicmaker", avatar: "https://picsum.photos/seed/user2/100/100" },
      views: 8900,
      likes: 654,
    },
    {
      id: "3",
      title: "Epic Dragon Battle in the Clouds",
      thumbnail: "https://picsum.photos/seed/anime3/800/450",
      creator: { username: "dragonlord", avatar: "https://picsum.photos/seed/user3/100/100" },
      views: 15200,
      likes: 1203,
    },
  ];

  return (
    <div>
      {/* Hero Section */}
      <section className="bg-accent py-20 border-b-brutal border-dark">
        <div className="brutal-container">
          <div className="max-w-4xl mx-auto text-center">
            <h1 className="mb-6">
              TRANSFORM WORDS INTO STUNNING ANIMATIONS
            </h1>
            <p className="text-xl md:text-2xl mb-8 font-semibold">
              No code required. Just describe what you want, and watch AI bring
              your vision to life.
            </p>
            <div className="flex flex-col sm:flex-row gap-4 justify-center">
              <Link href="/register">
                <Button variant="primary" size="lg">
                  Start Creating Free
                </Button>
              </Link>
              <Link href="/browse">
                <Button variant="outline" size="lg">
                  Browse Gallery
                </Button>
              </Link>
            </div>
            <p className="mt-6 text-sm font-bold">
              Join 10,000+ creators making amazing animations
            </p>
          </div>
        </div>
      </section>

      {/* How It Works Section */}
      <section id="how-it-works" className="py-20 bg-white">
        <div className="brutal-container">
          <h2 className="text-center mb-12">HOW IT WORKS</h2>
          <div className="grid md:grid-cols-3 gap-8">
            <div className="brutal-card p-8 text-center">
              <div className="w-20 h-20 bg-primary border-brutal border-dark shadow-brutal-sm mx-auto mb-4 flex items-center justify-center">
                <span className="text-4xl font-bold text-white">1</span>
              </div>
              <h3 className="text-2xl mb-4">DESCRIBE YOUR VISION</h3>
              <p className="text-lg">
                Simply type what you want to see. "A robot dancing in a neon
                city" or "A magical forest at sunset"
              </p>
            </div>

            <div className="brutal-card p-8 text-center">
              <div className="w-20 h-20 bg-secondary border-brutal border-dark shadow-brutal-sm mx-auto mb-4 flex items-center justify-center">
                <span className="text-4xl font-bold text-white">2</span>
              </div>
              <h3 className="text-2xl mb-4">AI CREATES MAGIC</h3>
              <p className="text-lg">
                Our advanced AI engine transforms your text into stunning
                animation frames in seconds
              </p>
            </div>

            <div className="brutal-card p-8 text-center">
              <div className="w-20 h-20 bg-accent border-brutal border-dark shadow-brutal-sm mx-auto mb-4 flex items-center justify-center">
                <span className="text-4xl font-bold text-dark">3</span>
              </div>
              <h3 className="text-2xl mb-4">DOWNLOAD & SHARE</h3>
              <p className="text-lg">
                Get your high-quality animation instantly. Share it anywhere or
                use it in your projects
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section id="features" className="py-20 bg-secondary text-white">
        <div className="brutal-container">
          <h2 className="text-center mb-12">POWERFUL FEATURES</h2>
          <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
            <div className="bg-white text-dark brutal-card p-6">
              <div className="text-4xl mb-3">⚡</div>
              <h4 className="text-xl mb-2">LIGHTNING FAST</h4>
              <p>Generate animations in seconds, not hours</p>
            </div>

            <div className="bg-white text-dark brutal-card p-6">
              <div className="text-4xl mb-3">🎨</div>
              <h4 className="text-xl mb-2">ENDLESS STYLES</h4>
              <p>Anime, cartoon, realistic, and more</p>
            </div>

            <div className="bg-white text-dark brutal-card p-6">
              <div className="text-4xl mb-3">📱</div>
              <h4 className="text-xl mb-2">ANY RESOLUTION</h4>
              <p>Export in HD, 4K, or custom sizes</p>
            </div>

            <div className="bg-white text-dark brutal-card p-6">
              <div className="text-4xl mb-3">💰</div>
              <h4 className="text-xl mb-2">COMMERCIAL USE</h4>
              <p>Full rights to use in your projects</p>
            </div>
          </div>
        </div>
      </section>

      {/* Sample Animations Showcase */}
      <section className="py-20 bg-white">
        <div className="brutal-container">
          <div className="text-center mb-12">
            <h2 className="mb-4">FEATURED ANIMATIONS</h2>
            <p className="text-xl">
              See what our community is creating
            </p>
          </div>

          <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8">
            {sampleAnimations.map((animation) => (
              <AnimationCard key={animation.id} {...animation} />
            ))}
          </div>

          <div className="text-center">
            <Link href="/browse">
              <Button variant="primary" size="lg">
                View All Animations
              </Button>
            </Link>
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="py-20 bg-primary text-white border-y-brutal border-dark">
        <div className="brutal-container text-center">
          <h2 className="mb-6">READY TO CREATE YOUR FIRST ANIMATION?</h2>
          <p className="text-xl md:text-2xl mb-8 font-semibold max-w-3xl mx-auto">
            Join thousands of creators and start making stunning animations
            today. No credit card required.
          </p>
          <Link href="/register">
            <Button variant="accent" size="lg">
              Get Started Free
            </Button>
          </Link>
        </div>
      </section>
    </div>
  );
}
