<script setup lang="ts">
import {onMounted, ref} from "vue";
import axios from "axios";
import TextComponent from "./components/TextComponent.vue";

const text = ref('')
const url = ref('')

onMounted(() => {
  const urlParams = new URLSearchParams(window.location.search);
  const id = urlParams.get('id');

  axios.get('/?id=' + id).then(res => {
    text.value = res.data.text
  })

  text.value = testText.value
})

// 这里可以测试发送写入markdown文本的请求
const testText = ref(`
# 一级标题

## 二级标题

### 三级标题

🌍

:smile: :smile: :smile: :satellite:
:smile:

公式测试

a$$e^{i\\pi} + 1 = 0$$a

a$$\\frac{1}{2} $$a

bbc\\[\\frac{1}{2}\\]abc

\\[ \\int_0^\\pi \\sin x \\, dx \\]

a$\\frac{1}{2}$a

a\\(\\frac{1}{2}\\)a

公式测试

a$ax^2 + bx + c = 0$a

$ax^2 + bx + c = 0$

a\\(ax^2 + bx + c = 0\\)a

\\[ ax^2 + bx + c = 0 \\]

\\[
ax^2 + bx + c = 0 
\\]

\\( x = \\frac{-b \\pm \\sqrt{b^2 - 4ac}}{2a} \\)

test\\(\\left( \\frac{1}{2}abc \\right) \\)test

$
\\left( \\begin{matrix}
	1&		0&		0\\\\
	0&		1&		0\\\\
	0&		0&		1\\\\
\\end{matrix}\\right)
$

a\\(\\left( \\begin{matrix}
1&		aaa&		0\\\\
0&		1&		0\\\\
0&		0&		1\\\\
\\end{matrix}\\right)\\)bcd

a\\(\\left( \\begin{matrix}1& aaa& 0\\\\0& 1& 0\\\\0& 0& 1\\\\\\end{matrix}\\right)\\)bcd

a$\\left( \\begin{matrix}1& aaa& 0\\\\0& 1& 0\\\\0& 0& 1\\\\\\end{matrix}\\right)$bcd

这是一个普通的注释 //aa


\\[
\\left( \\begin{matrix}
	1&		0&		0\\\\
	0&		1&		0\\\\
	0&		0&		1\\\\
\\end{matrix}\\right)
\\]

$$
\\left[ \\begin{matrix}
	1&		0&		0\\\\
	0&		1&		0\\\\
	0&		0&		1\\\\
\\end{matrix} \\right]
$$

\\[\\left[ \\frac{1}{2}ABC \\right] \\]

代码高亮测试
\`\`\`java
// FileName: HelloWorld.java
public class HelloWorld {
  // Java 入口程序，程序从此入口
  public static void main(String[] args) {
    System.out.println("Hello,World!"); // 向控制台打印一条语句
  }
}
\`\`\`

\`\`\`c
#include <stdio.h>  // 引入标准输入输出库头文件

int main() {
  // 使用 printf 函数输出 "Hello, World!"
  printf("Hello, World!\\n");
  return 0;  // 程序执行成功，返回0
}
\`\`\`

| Header 1      | Header 2      | Header 3      |
|---------------|---------------|---------------|
| Row 1, Col 1  | Row 1, Col 2  | Row 1, Col 3  |
| Row 2, Col 1  | Row 2, Col 2  | Row 2, Col 3  |
| Row 3, Col 1  | Row 3, Col 2  | Row 3, Col 3  |

### 普通测试：
1. **包引入**：使用 \`tikz\` 和 \`pgfplots\` 包，\`pgfplots\` 是基于 TiKZ 的功能更丰富的绘图工具，适合用于绘制二维和三维图表。
2. **坐标轴设置**：
   - \`xmode=log\` 将 x 轴设置为对数模式，适合频率响应图。
   - \`xmin\`、\`xmax\`、\`ymin\`、\`ymax\` 设定坐标轴的范围。
   - \`xtick\` 和 \`ytick\` 定义坐标轴的刻度。
   - 网格线帮助更好地阅读图表。
3. **增益曲线**：
   - 使用 \`\\addplot\` 绘制四段不同的曲线，表示不同频率区间的增益变化。
   - 使用颜色和粗细区分不同的曲线段。
4. **图例说明**：\`\\legend\` 添加图例，描述每段曲线的特性。

**缺点**：
- 降低了样式表的可维护性，因为 \`!important\` 会打破 CSS 的自然层叠规则。
- 使得调试样式问题变得更加困难，因为它可以覆盖你可能没有注意到的样式规则。
- 过度使用 \`!important\` 可能导致样式冲突难以解决，因为每个使用 \`!important\` 的样式都会尝试优先于其他样式。

`)
function uploadTest() {
  axios.post('/upload', {
    text: testText.value,
  }).then(res => {
    url.value = res.data.url
  })
}

</script>


<template>
  <div>
    <textarea v-model="testText" style="height: 100px;width: 100%"></textarea>
    <button style="background-color: #4CAF50; color: white; padding: 5px 20px; border: none; cursor: pointer;" @click="uploadTest">点击我测试写入 Markdown 的请求</button>
    <div>请求返回的URL链接：<a :href="url" target="_blank">{{url}}</a> （点击此链接查看渲染后的Markdown文本）</div>
    <br/>
    <TextComponent ref="textRef" :text="text"/>
  </div>
</template>

<style scoped>

</style>
