import type { Block } from '@/shared/ui';

/** A link by its words. `href` is a route of this site (`/docs`) or an address. */
export type NavLink = { label: string; href: string };

/** What the whole site knows about itself: content/site.ts. */
export type SiteConfig = {
  /** The product's name: the titles' suffix, what a screen reader is told a link goes to. */
  name: string;
  /** The landing's <title> and its link preview's. */
  title: string;
  /** One or two sentences: the meta description, the link preview. */
  description: string;
  /** Where the site is published, with no trailing slash: canonical URLs, the sitemap. */
  url: string;
  /** The product's repository. */
  github: string;
  /** The repository this site's sources are in, when it is not the product's: where "Edit this
   *  page" goes, and where a docs link to a repository file points. */
  source?: string;
  /** The folder of that repository the site is in, when it is not its root (`site`): what a
   *  path under the site's root is given in a link to the repository. */
  sourceDir?: string;
  /** The install command, one tab per way in (a tab row shows only with several). */
  install: (readonly [label: string, command: string])[];
  /** The card every link preview shows, under public/: a PNG or a JPEG of 1200×630, or of the
   *  size given here when it is drawn larger (twice that, for a dense screen). */
  image: { path: string; alt: string; width?: number; height?: number };
  /** The bar over every page: its links, on the right, and the one action after them, a filled
   *  pill. On a phone only the first link and the action show. */
  bar: { links: NavLink[]; action?: NavLink };
  /** Under every page: one line, and links. */
  footer: { note: string; links: NavLink[] };
  /** Who makes the product, and the page of everything they make: signed first in the footer,
   *  before its line, and the way there from every page. */
  maker?: NavLink;
  /** schema.org data for the landing. */
  jsonLd?: Record<string, unknown>;
  /** The opening of llms.txt, as markdown; the docs' list follows it. */
  llms: string;
};

/** The docs' one table of contents: content/docs/nav.ts. A page is its file's name under
 *  content/docs without `.md`, reached at /docs/<name>; its title is its first heading and its
 *  description its first paragraph. */
export type DocsConfig = {
  /** The index at /docs. */
  index: { title: string; description: string };
  groups: { title: string; blurb: string; pages: string[] }[];
  /** Markdown under content/docs that is no page, and why: any other file there must be in a
   *  group, so a new doc is never left off the site unnoticed. */
  unlisted: Record<string, string>;
};

/** A screenshot or a diagram: its file under public/, its words, and its size in pixels, so its
 *  room is held before it loads. `dark` is the same picture in a dark theme, shown instead when
 *  the reader's system is dark (it has the same size). */
export type Shot = { src: string; dark?: string; alt: string; width: number; height: number };

/** What a card shows beside its words. A picture or a video names its size in pixels, so its
 *  room is held before it loads. Files are under public/.
 *  - `terminal`: what a program printed, drawn from blocks (shared/ui's Panel);
 *  - `image`: a screenshot or a diagram, itself the link to the picture whole;
 *  - `video`: a short silent clip, played while it is in view;
 *  - `film`: a terminal session recorded with asciinema, played in a real terminal. */
export type Media =
  | { kind: 'terminal'; blocks: Block[]; label?: string }
  | ({ kind: 'image' } & Shot)
  | { kind: 'video'; src: string; label: string; width: number; height: number; poster?: string }
  | { kind: 'film'; src: string; label: string };

/** A card of the landing: a claim, and beside it what shows it. `tag` is a small label over the
 *  title; `text` marks code with backticks; `links` say where to read more. With no `media` the
 *  words take the card's whole width. */
export type Card = { tag?: string; title: string; text: string; media?: Media; links?: NavLink[] };

/** The landing: content/landing.ts. `code` between backticks in a text is drawn as code. A
 *  section left out is not drawn. */
export type LandingContent = {
  hero: {
    /** The headline: an optional first word on a chip, what follows it unbroken (a comma), and
     *  the rest. */
    title: { chip?: string; after?: string; rest: string };
    lede: string;
    /** The small line under the install command; it takes [links](/docs). */
    under: string;
    /** A recorded terminal session (asciinema v3) under public/, and what it shows, in words. */
    film?: { src: string; label: string };
    /** The product itself in one picture, beside the words; shown instead of the recording when
     *  both are given. */
    shot?: Shot;
  };
  /** The cards, one sliding over another under their heading. `aside` is the short line at the
   *  heading's right. */
  loop: { title: string; aside?: string; cards: Card[] };
  /** The docs as columns, a column a group of content/docs/nav.ts, a row a page. */
  docs?: { title: string };
  /** The latest releases, read from CHANGELOG.md; `more` is the link at the heading's right. */
  releases?: { title: string; more?: NavLink };
  /** The page's end: how to start, in numbered steps, and where to go; the first link is the
   *  filled action. Its address on the page is `#start`. */
  closing: { title: string; aside?: string; steps: { title: string; text: string }[]; links: NavLink[] };
};
