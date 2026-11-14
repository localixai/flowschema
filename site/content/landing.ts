import type { Block, Card, LandingContent, Line, Media } from '@/shared/config';

// The landing's words. The design is the template's; this file is the site's. Every message a
// card shows is what the library prints (the repository's doc tests hold them), and every figure
// is the latest run's, in the repository's bench/results: change one there first, then here.
const DOCS = '/docs';
const GITHUB = 'https://github.com/localixai/flowschema';

const terminal = (blocks: Block[], label: string): Media => ({ kind: 'terminal', blocks, label });
const ui = (...lines: Line[]): Block => ({ ui: lines });
const json = (text: string): Block => ({ lang: 'json', text });
// A line with nothing on it: a space, since a row with no text has no height.
const GAP: Line = [['', ' ']];

const CARDS: Card[] = [
  {
    tag: 'Tool calls',
    title: 'A call that does not fit is not run',
    text: 'Compile a tool’s parameters, check every call the model writes, and send what is wrong back as the call’s result. Each message says where the value is, the rule, and what the data has.',
    links: [{ href: `${DOCS}/tool-calls-and-answers`, label: 'Tool calls and answers' }],
    media: terminal(
      [
        ui([['dim', 'the call the model wrote']]),
        json('{"path": 7, "limit": 0, "force": true}'),
        ui(
          GAP,
          [['dim', 'what goes back to it']],
          [['hi', 'data has "force", which the schema does not allow']],
          [['hi', 'data.limit must be >= 1 (is 0)']],
          [['hi', 'data.path must be string (is integer)']],
        ),
      ],
      'A tool call with a number for its path, a limit of zero and a field the schema does not allow, and the three messages flowschema writes for the model',
    ),
  },
  {
    tag: 'Per request',
    title: 'A schema that comes with the request',
    text: 'Where every request brings its own tools, compile the schema, check, and drop it. `Schema::new` reads the document where it is and copies nothing, so there is no cache to key, to bound or to clear.',
    links: [{ href: `${DOCS}/benchmarks`, label: 'Benchmarks' }],
    media: terminal(
      [
        {
          grid: [
            [['dim', 'compiled, one answer checked'], ['dim', 'flowschema'], ['dim', 'jsonschema'], ['dim', '']],
            ['a tool call', ['hi', '516 ns'], '3.98 µs', ['ok', '7.7×']],
            ['a structured answer', ['hi', '1.26 µs'], '6.50 µs', ['ok', '5.2×']],
            ['nested models', ['hi', '2.67 µs'], '10.47 µs', ['ok', '3.9×']],
          ],
          align: 'lrrr',
        },
        ui(GAP, [['dim', 'Both built for speed, on an Apple M5; the jsonschema crate 0.60.']]),
      ],
      'A schema compiled and one answer checked: a tool call in 516 nanoseconds where the jsonschema crate takes 3.98 microseconds, a structured answer in 1.26 against 6.50 microseconds, nested models in 2.67 against 10.47',
    ),
  },
  {
    tag: 'Providers',
    title: 'What a strict mode takes, and what is still yours',
    text: 'OpenAI, Anthropic and Gemini each hold a model to their own part of JSON Schema. `fit` gives the schema a provider takes and says every change; the answer is still checked against the schema as it was written.',
    links: [{ href: `${DOCS}/provider-strict-modes`, label: 'Provider strict modes' }],
    media: terminal(
      [
        ui(
          [['dim', 'fit(&schema, Provider::Anthropic).changes']],
          [['hi', 'schema.properties.limit: `minimum` is not held: taken out']],
          [['hi', 'schema: `additionalProperties` set to false, as the provider asks']],
          GAP,
          [['dim', 'the answer, against the schema as written']],
          [['hi', 'data.limit must be >= 1 (is 0)']],
        ),
      ],
      'The changes fit makes for Anthropic: minimum is taken out and additionalProperties is set to false; the answer still fails the schema as written, limit must be at least 1',
    ),
  },
  {
    tag: 'Streams',
    title: 'Cut off, still arriving, or written as a string',
    text: '`whole` gives what a reply the output limit stopped wrote whole. `may_match` says whether a call still streaming can still end in a match. `mend` reads `"30"` as the integer the schema asks for, and says so.',
    links: [{ href: `${DOCS}/cut-off-and-streaming`, label: 'Cut-off and streaming calls' }],
    media: terminal(
      [
        ui(
          [['dim', 'whole: '], ['', '{"path": "src/lib.rs", "lines": [1, 2']],
          [['hi', '{"lines":[1],"path":"src/lib.rs"}']],
          GAP,
          [['dim', 'may_match: '], ['', '{"mode": "x']],
          [['hi', 'false'], ['dim', '   no option begins so']],
          GAP,
          [['dim', 'mend: '], ['', '{"limit": "30"}']],
          [['hi', 'data.limit: "30" read as 30']],
        ),
      ],
      'A call cut off inside a list keeps the path and the first line; a call whose mode begins with x can no longer match; a limit written as the string 30 is read as the integer 30',
    ),
  },
  {
    tag: 'Whole',
    title: 'All of JSON Schema, and what is not checked is said',
    text: 'Drafts 4 to 2020-12, every keyword and applicator, `$ref` and formats: every required test of the official suite passes. A schema that asks for what is not checked is never taken as stricter than it is: `unchecked` names it.',
    links: [{ href: `${DOCS}/what-is-checked`, label: 'What is checked' }],
    media: terminal(
      [
        {
          grid: [
            ['official test suite', ['hi', '7,675 of 7,675']],
            ['real LLM answers judged as labelled', ['hi', '99.98%']],
            ['schemas of MaskBench compiled', ['hi', '11,306 of 11,306']],
            ['panics on any of them', ['hi', '0']],
          ],
          align: 'lr',
        },
      ],
      'The official test suite: 7,675 of 7,675 required tests pass; 99.98% of real LLM answers are judged as labelled; all 11,306 schemas of MaskBench compile, with no panic',
    ),
  },
  {
    tag: 'Small',
    title: 'Two dependencies, no allocation on a check',
    text: 'It adds 309 KB to a release binary and two crates to a build: `serde_json`, which you have, and `regex-lite`. No `unsafe`, no I/O, nothing fetched, and every pattern runs in time linear in its text.',
    links: [{ href: `${DOCS}/benchmarks#weight`, label: 'What it weighs' }],
    media: terminal(
      [
        {
          grid: [
            [['dim', ''], ['dim', 'flowschema'], ['dim', 'jsonschema']],
            ['added to a binary', ['hi', '309 KB'], '1,884 KB'],
            ['crates added to a build', ['hi', '2'], '57'],
            ['allocations on a check', ['hi', '0'], 'up to 7'],
          ],
          align: 'lrr',
        },
      ],
      'flowschema adds 309 kilobytes and 2 crates where the jsonschema crate adds 1,884 kilobytes and 57, and allocates nothing on a check',
    ),
  },
];

const landing: LandingContent = {
  hero: {
    title: { chip: 'JSON Schema', rest: 'for agents' },
    lede: 'Tool calls and structured output, checked as models write them. flowschema compiles a schema in about a microsecond, checks a call against it with no allocation, and says what is wrong in words a model can fix its answer from.',
    under: 'A Rust crate · MIT · two dependencies · [Get started](/docs/getting-started)',
    film: {
      src: '/demo.cast',
      label: 'A terminal session: flowschema is added to a crate from its repository, the program checks a tool call a model wrote, and its run prints the three messages that say what to fix',
    },
  },
  loop: {
    title: 'What an agent does with JSON',
    aside: 'Six things it is made for',
    cards: CARDS,
  },
  docs: { title: 'Documentation' },
  closing: {
    title: 'Start checking',
    aside: 'Free · MIT license',
    steps: [
      { title: 'Add it', text: '`cargo add flowschema --git https://github.com/localixai/flowschema`, beside `serde_json`. Rust 1.89 or later.' },
      { title: 'Compile the schema', text: '`Schema::new(&schema)`: a tool’s parameters or an answer’s shape, once for many checks or from the request that brings it.' },
      { title: 'Check, and send back', text: '`matches` says whether the data fits; `mismatches` writes what to send the model so it fixes its answer.' },
    ],
    links: [
      { label: 'Getting started', href: `${DOCS}/getting-started` },
      { label: 'Source on GitHub', href: GITHUB },
    ],
  },
};

export default landing;
