import { useEffect, useRef } from 'react';
import { Film } from '@/features/film';
import type { Media } from '@/shared/config';
import { Panel, Picture } from '@/shared/ui';

/** What a card shows beside its words: a drawn terminal, a picture, a clip or a recording.
 *  `title` is the card's, for a terminal that names nothing itself. */
export function CardMedia({ media, title }: { media: Media; title: string }) {
  switch (media.kind) {
    case 'terminal':
      return <Panel blocks={media.blocks} label={media.label ?? `${title}: what the terminal shows`} />;
    case 'image':
      return (
        // The picture itself is the link to it whole: a screenshot is read at its own size. One
        // taller than wide (a popup, a phone) is drawn narrower, so the card stays a screen high.
        <a
          className={media.height > media.width ? 'loop-shot loop-shot-tall' : 'loop-shot'}
          href={media.src}
          aria-label={`${media.alt} (open the picture)`}
        >
          <Picture src={media.src} dark={media.dark} alt={media.alt} width={media.width} height={media.height} />
        </a>
      );
    case 'video':
      return <Clip media={media} />;
    case 'film':
      return <Film src={media.src} label={media.label} />;
  }
}

/** A silent clip that plays only while it is seen, and not at all with reduced motion (its
 *  poster, or its first frame, stands instead; the controls are there to play it by hand). */
function Clip({ media }: { media: Extract<Media, { kind: 'video' }> }) {
  const video = useRef<HTMLVideoElement>(null);
  useEffect(() => {
    const clip = video.current;
    if (!clip || window.matchMedia('(prefers-reduced-motion: reduce)').matches) return;
    const seen = new IntersectionObserver(
      ([entry]) => {
        if (entry?.isIntersecting) void clip.play().catch(() => {});
        else clip.pause();
      },
      { threshold: 0.4 },
    );
    seen.observe(clip);
    return () => seen.disconnect();
  }, []);
  return (
    <video
      ref={video}
      className="loop-shot"
      src={media.src}
      poster={media.poster}
      width={media.width}
      height={media.height}
      aria-label={media.label}
      muted
      loop
      playsInline
      controls
      preload="metadata"
    />
  );
}
