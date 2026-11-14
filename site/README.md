# The site

The landing page and the docs of flowschema, served at <https://flowschema.localix.ai>, as a
folder of the library's repository. It was made from the
[llx-sitekit](https://github.com/rsaryev/llx-sitekit) template (its commit `643651f`, which
`sitekit.synced` in `package.json` names) and keeps the template's layout, so
[AGENTS.md](AGENTS.md) here holds for every change in this folder.

```sh
cd site
npm install
npm run dev          # http://localhost:5173
npm run check        # the layers, the types, the build, the docs' standard
npm run sync         # the template's changes since sitekit.synced, as a patch
```

## What is whose

| Path | Owner | |
| --- | --- | --- |
| `content/` | the site | `site.ts` (name, address, the install command), `landing.ts` (the landing's words), `docs/` (the pages and `nav.ts`), `theme.css`, `demo/` (the program the hero's recording runs) |
| `public/` | the site | the icons, the link preview (`og.png`, drawn from `og-card.html`), the hero's recording (`demo.cast`), `llms.txt`, `robots.txt` |
| `src/`, `scripts/`, the configs | the template | change one only where the site cannot say it in `content/` |

## What the site says is the repository's

- **A figure** on the landing or in `content/docs/benchmarks.md` is the latest run's in
  `../bench/results/` (`micro.md`, `corpus.md`, `answers.md`, `size.md`). After a run that moves
  one, change it here in the same commit; never type one that no result holds.
- **A message** a card or a page shows is what the library prints: the examples of
  `content/docs/` are those of `../docs/`, which the crate's doc tests run.
- **The hero's recording** (`public/demo.cast`) is a real session: `content/demo/main.rs` in a
  new crate that took flowschema by the site's own install command, played by
  `content/demo/session.sh` under `asciinema rec --headless --output-format asciicast-v3
  --window-size 92x30 -c 'sh session.sh'`, its header cut to the terminal's size and a title,
  and a marker put before each command. Record it again when a message it shows changes.

## What differs from a site cloned from the template

- **It has no repository of its own.** `content/site.ts` says so with `sourceDir: 'site'`, which
  `sourceUrl` in `src/shared/config/index.ts` puts into a page's "Edit on GitHub".
- **The link preview is drawn larger**, 2400×1260; `image.width` and `image.height` say so in the
  page's head.
- **Its workflow is the repository's**, `../.github/workflows/site.yml`: it checks the site on a
  change to it on `main` and publishes `dist/` to Cloudflare Pages, project `flowschema`.
