import Link from "next/link";
import Image from "next/image";

interface AnimationCardProps {
  id: string;
  title: string;
  thumbnail: string;
  creator: {
    username: string;
    avatar?: string;
  };
  views?: number;
  likes?: number;
}

export default function AnimationCard({
  id,
  title,
  thumbnail,
  creator,
  views = 0,
  likes = 0,
}: AnimationCardProps) {
  return (
    <Link href={`/animation/${id}`}>
      <div className="brutal-card-hover overflow-hidden">
        <div className="relative aspect-video bg-gray-100">
          <Image
            src={thumbnail}
            alt={title}
            fill
            className="object-cover"
            sizes="(max-width: 768px) 100vw, (max-width: 1200px) 50vw, 33vw"
          />
        </div>

        <div className="p-4 border-t-brutal border-dark">
          <h3 className="font-bold text-lg mb-2 line-clamp-2">{title}</h3>

          <div className="flex items-center justify-between text-sm">
            <div className="flex items-center gap-2">
              {creator.avatar ? (
                <div className="relative w-6 h-6 rounded-full overflow-hidden border-2 border-dark">
                  <Image
                    src={creator.avatar}
                    alt={creator.username}
                    fill
                    className="object-cover"
                  />
                </div>
              ) : (
                <div className="w-6 h-6 rounded-full bg-accent border-2 border-dark flex items-center justify-center">
                  <span className="text-xs font-bold">
                    {creator.username[0].toUpperCase()}
                  </span>
                </div>
              )}
              <span className="font-semibold">{creator.username}</span>
            </div>

            <div className="flex items-center gap-3 text-xs font-bold">
              <span>👁 {views.toLocaleString()}</span>
              <span>❤️ {likes.toLocaleString()}</span>
            </div>
          </div>
        </div>
      </div>
    </Link>
  );
}
