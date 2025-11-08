<template>
  <div class="syntax-highlighter">
    <h3>Подсветка синтаксиса Dr Script</h3>
    <div class="code-display" v-html="highlightedCode"></div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'

const code = ref(`// Пример кода Dr Script
ver x = 10;
var y = 5;

if x > y {
    print("x больше y");
} else {
    print("y больше или равно x");
}

fun add(a, b) {
    return a + b;
}

var result = add(x, y);
print(result);`)

const highlightedCode = computed(() => {
  let highlighted = code.value
  
  // Экранируем HTML
  highlighted = highlighted
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
  
  // Комментарии
  highlighted = highlighted.replace(
    /\/\/.*$/gm,
    '<span class="comment">$&</span>'
  )
  
  // Строки
  highlighted = highlighted.replace(
    /"[^"]*"/g,
    '<span class="string">$&</span>'
  )
  
  // Числа
  highlighted = highlighted.replace(
    /\b\d+(\.\d+)?\b/g,
    '<span class="number">$&</span>'
  )
  
  // Ключевые слова
  const keywords = ['ver', 'var', 'if', 'else', 'while', 'for', 'fun', 'return', 'print']
  keywords.forEach(keyword => {
    const regex = new RegExp(`\\b${keyword}\\b`, 'g')
    highlighted = highlighted.replace(regex, `<span class="keyword">${keyword}</span>`)
  })
  
  // Операторы
  highlighted = highlighted.replace(
    /(\+\+|--|&lt;=|&gt;=|==|!=|[+\-*/%=!]|&lt;|&gt;)/g,
    '<span class="operator">$1</span>'
  )
  
  // Скобки
  highlighted = highlighted.replace(
    /([{}()\[\];,])/g,
    '<span class="punctuation">$1</span>'
  )
  
  return highlighted
})
</script>

<style scoped>
.syntax-highlighter {
  padding: 20px;
  background: #0D1117;
  color: #E6EDF3;
  font-family: 'JetBrains Mono', Consolas, Monaco, monospace;
}

h3 {
  color: #58A6FF;
  margin-bottom: 16px;
}

.code-display {
  background: #161B22;
  border: 1px solid #30363D;
  border-radius: 8px;
  padding: 16px;
  font-size: 14px;
  line-height: 1.5;
  white-space: pre;
  overflow-x: auto;
}

:deep(.keyword) {
  color: #FF7B72;
  font-weight: 600;
}

:deep(.string) {
  color: #A5D6FF;
}

:deep(.number) {
  color: #79C0FF;
}

:deep(.comment) {
  color: #8B949E;
  font-style: italic;
}

:deep(.operator) {
  color: #FF7B72;
}

:deep(.punctuation) {
  color: #C9D1D9;
}
</style>