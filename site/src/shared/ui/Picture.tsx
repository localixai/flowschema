import './Picture.css';

/** A screenshot as the reader's system is set: its dark version under a dark theme, when there is
 *  one, the light one otherwise. The size holds its room before it loads; `eager` is for the one
 *  picture of the first screen, which should not wait for the page to settle. */
export function Picture({
  src,
  dark,
  alt,
  width,
  height,
  eager = false,
}: {
  src: string;
  dark?: string | undefined;
  alt: string;
  width: number;
  height: number;
  eager?: boolean;
}) {
  return (
    <picture className="picture">
      {dark && <source media="(prefers-color-scheme: dark)" srcSet={dark} width={width} height={height} />}
      <img src={src} alt={alt} width={width} height={height} loading={eager ? 'eager' : 'lazy'} decoding="async" />
    </picture>
  );
}
