import type { SiteConfig } from '@/shared/config';

// What the site knows about itself: the library's name, where the site is published, the
// repository, how a program takes the crate. It is taken from the repository (no release is
// tagged yet, and it is not on crates.io), which is what the command says.
const GITHUB = 'https://github.com/localixai/flowschema';
const URL = 'https://flowschema.localix.ai';
const DESCRIPTION =
  'JSON Schema for agents: tool calls and structured output, checked as models write them. A Rust crate: drafts 4 to 2020-12, two dependencies, no allocation on a check.';

const site: SiteConfig = {
  name: 'flowschema',
  // The family's pattern, "<Product>: <its line>", as the repository's description starts.
  title: 'flowschema: JSON Schema for agents',
  description: DESCRIPTION,
  url: URL,
  github: GITHUB,
  // The site is a folder of the library's repository, not a repository of its own.
  sourceDir: 'site',
  install: [['Cargo', `cargo add flowschema --git ${GITHUB}`]],
  image: {
    path: '/og.png',
    width: 2400,
    height: 1260,
    alt: 'flowschema, JSON Schema for agents: a tool call checked against its schema, and the message that says what the model must fix',
  },
  bar: {
    links: [
      { label: 'Docs', href: '/docs' },
      { label: 'Benchmarks', href: '/docs/benchmarks' },
      { label: 'GitHub', href: GITHUB },
    ],
    action: { label: 'Install', href: '/#start' },
  },
  maker: { label: 'Localix Lab', href: 'https://localix.ai' },
  footer: {
    note: 'flowschema is MIT-licensed and open source.',
    links: [
      { label: 'Docs', href: '/docs' },
      { label: 'GitHub', href: GITHUB },
      { label: 'Changelog', href: `${GITHUB}/blob/main/CHANGELOG.md` },
    ],
  },
  // The landing's WebSite gets these: what it is, beyond a page about it.
  jsonLd: {
    mainEntity: {
      '@type': 'SoftwareSourceCode',
      '@id': `${URL}/#crate`,
      name: 'flowschema',
      description: DESCRIPTION,
      programmingLanguage: 'Rust',
      codeRepository: GITHUB,
      license: 'https://opensource.org/licenses/MIT',
      url: `${URL}/`,
      image: `${URL}/og.png`,
      sameAs: [GITHUB],
      author: { '@type': 'Organization', name: 'Localix Lab', url: 'https://localix.ai' },
    },
    inLanguage: 'en',
  },
  llms: `> ${DESCRIPTION}`,
};

export default site;
