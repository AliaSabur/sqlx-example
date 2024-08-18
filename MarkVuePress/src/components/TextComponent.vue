<script lang="ts" setup>
import { computed, ref } from 'vue';
import { Marked } from "marked";
import { markedHighlight } from "marked-highlight";
import hljs from 'highlight.js';
import KatexPlugin from './KatexPlugin';

import 'katex/contrib/mathtex-script-type';
import 'katex/contrib/mhchem';
import 'katex/dist/katex.min.css'; // 导入 KaTeX 样式

const marked = new Marked(
  markedHighlight({
    langPrefix: 'hljs language-',
    highlight(code, language, _info) {
      // const language = hljs.getLanguage(lang) ? lang : 'plaintext';
      // return hljs.highlight(code, { language }).value;
      const validLang = !!(language && hljs.getLanguage(language));
      if (validLang) {
        const lang = language ?? '';
        return hljs.highlight(code, { language: lang }).value;
      }
      return hljs.highlightAuto(code).value;
    }
  })
);

const options = {
  throwOnError: false,
  displayMode: true,
  nonStandard: true
};

marked.use(KatexPlugin(options));

interface Props {
  text?: string;
}

const props = defineProps<Props>();
const textRef = ref<HTMLElement>();


const wrapClass = computed(() => {
  return [
    'message-reply',
  ];
});

const text = computed(() => {
  const value = props.text ?? '';
  console.log("Rendering text:", value);
  const html = marked.parse(value);
  return html;
});

</script>

<template>
    <div :class="wrapClass" class="text-black">
      <div ref="textRef" class="leading-relaxed break-words">
        <div class="whitespace-pre-wrap markdown-body" v-html="text"></div>
      </div>
    </div>
</template>


<style lang="less">
@import 'styles';

// [aria-hidden="true"] {
//   display: none;
// }

.markdown-body .highlight pre,
.markdown-body pre {
  padding: 5px;
  overflow: auto;
  font-size: 85%;
  line-height: 1.45;
  background-color: var(--color-canvas-subtle);
  border-radius: 6px;
}

.markdown-body {
  font-size: 30px;
  background-color: #ededed;
}

// .katex {
//   font-size: 40px;
//   /* 单独设置 KaTeX 公式的字体大小 */
// }

</style>