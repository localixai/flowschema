import type { DocsConfig } from '@/shared/config';

// The docs' table of contents. Every markdown file under content/docs is a page listed here, in
// a group, or is named under `unlisted` with why: a file in neither fails the build.
const docs: DocsConfig = {
  index: {
    title: 'flowschema docs',
    description: 'How to check tool calls and structured answers against JSON Schema with flowschema: the install, an agent\'s two loops, strict modes and streaming calls.',
  },
  groups: [
    {
      title: 'Start',
      blurb: 'From the install to a first check, and the two places an agent checks JSON.',
      pages: ['getting-started', 'tool-calls-and-answers'],
    },
    {
      title: 'With a model',
      blurb: 'What a provider holds a model to, and what a model writes that is not yet a value.',
      pages: ['provider-strict-modes', 'cut-off-and-streaming'],
    },
    {
      title: 'Reference',
      blurb: 'Every item of the API, what is checked and how it is held to it, and how fast.',
      pages: ['api', 'what-is-checked', 'benchmarks'],
    },
  ],
  unlisted: {},
};

export default docs;
