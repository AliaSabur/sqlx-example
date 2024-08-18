import katex from 'katex';
import { KatexOptions } from 'katex';
import { MarkedExtension } from 'marked';

const inlineRule = /^(\${1,2})(?!\$)((?:\\.|[^\\\n])*?(?:\\.|[^\\\n\$]))\1/;
const blockRule = /^(\${1,2})\n((?:\\[^]|[^\\])+?)\n\1(?:\n|$)/;

// const inlineParenRule = /^\\\(((?:\\.|[^\\\n])*?(?:\\.|[^\\\n\)]))\\\)/;
const inlineParenRule = /^\\\(([\s\S]*?)\\\)/;
// const blockParenRule = /^\\\[\s*([\s\S]*?)\s*\\\](?:\n|$)/;
const blockParenRule = /^\\\[([\s\S]*?)\\\]/;

export default function markedKatex(options?: KatexOptions): MarkedExtension {
  return {
    extensions: [
      inlineKatex(options, createRenderer(options, false)),
      blockKatex(options, createRenderer(options, true)),
      inlineParenKatex(options, createRenderer(options, false)),
      blockParenKatex(options, createRenderer(options, true)),
    ],
  };
}

function createRenderer(options: any, newlineAfter: any) {
  return (token: any) => katex.renderToString(token.text, { ...options, displayMode: token.displayMode }) + (newlineAfter ? '\n' : '');
}

function inlineKatex(_options: any, renderer: any) {
  const ruleReg = inlineRule;
  return {
    name: 'inlineKatex',
    level: 'inline',
    start(src: any) {
      let index;
      let indexSrc = src;

      while (indexSrc) {
        index = indexSrc.indexOf('$');
        if (index === -1) {
          return;
        }
        if (index > -1) {
          const possibleKatex = indexSrc.substring(index);

          if (possibleKatex.match(ruleReg)) {
            return index;
          }
        }

        indexSrc = indexSrc.substring(index + 1).replace(/^\$+/, '');
      }
    },
    tokenizer(src: any, _tokens: any) {
      const match = src.match(ruleReg);
      if (match) {
        return {
          type: 'inlineKatex',
          raw: match[0],
          text: match[2].trim(),
          displayMode: match[1].length === 2,
        };
      }
    },
    renderer,
  };
}

function blockKatex(_options: any, renderer: any) {
  return {
    name: 'blockKatex',
    level: 'block',
    tokenizer(src: any, _tokens: any) {
      const match = src.match(blockRule);
      if (match) {
        return {
          type: 'blockKatex',
          raw: match[0],
          text: match[2].trim(),
          displayMode: match[1].length === 2,
        };
      }
    },
    renderer,
  };
}

function inlineParenKatex(_options: any, renderer: any) {
  return {
    name: 'inlineParenKatex',
    level: 'inline',
    start(src: any) {
      let index;
      let indexSrc = src;

      while (indexSrc) {
        index = indexSrc.indexOf('\\(');
        if (index === -1) {
          return;
        }

        if (index > -1) {
          const possibleKatex = indexSrc.substring(index);

          if (possibleKatex.match(inlineParenRule)) {
            return index;
          }
        }

        indexSrc = indexSrc.substring(index + 1).replace(/^\\\(+/, '');
      }
    },
    tokenizer(src: any, _tokens: any) {
      const match = src.match(inlineParenRule);
      if (match) {
        return {
          type: 'inlineParenKatex',
          raw: match[0],
          text: match[1].trim(),
          displayMode: false
        };
      }
    },
    renderer
  };
}


// function blockParenKatex(_options: any, renderer: any) {
//   return {
//     name: 'blockParenKatex',
//     level: 'block',
//     tokenizer(src: any, _tokens: any) {
//       const match = src.match(blockParenRule);
//       if (match) {
//         return {
//           type: 'blockParenKatex',
//           raw: match[0],
//           text: match[1].trim(),
//           displayMode: true
//         };
//       }
//     },
//     renderer
//   };
// }

function blockParenKatex(_options: any, renderer: any) {
  return {
    name: 'blockParenKatex',
    level: 'block',
    start(src: any) {
      let index;
      let indexSrc = src;

      while (indexSrc) {
        index = indexSrc.indexOf('\\[');
        if (index === -1) {
          return;
        }

        if (index > -1) {
          const possibleKatex = indexSrc.substring(index);

          if (possibleKatex.match(blockParenRule)) {
            return index;
          }
        }

        indexSrc = indexSrc.substring(index + 1).replace(/^\\\[+/, '');
      }
    },
    tokenizer(src: any, _tokens: any) {
      const match = src.match(blockParenRule);
      if (match) {
        return {
          type: 'blockParenKatex',
          raw: match[0],
          text: match[1].trim(),
          displayMode: true
        };
      }
    },
    renderer
  };
}