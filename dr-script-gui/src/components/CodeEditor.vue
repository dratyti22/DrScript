<script setup>
import { ref, computed } from 'vue'

const code = ref(`fn main() {
    let mut x = 5;
    if x < 10 {
        println!("x is small");
    }
}`)

const lines = computed(() => code.value.split('\n'))

function syncScroll(e) {
  const numbers = e.target.previousElementSibling
  numbers.scrollTop = e.target.scrollTop
}
</script>

<template>
  <div class="code-editor-container">
    <!-- Нумерация строк -->
    <div class="line-numbers custom-scrollbar">
      <div v-for="(_, i) in lines" :key="i" class="line-number">{{ i + 1 }}</div>
    </div>

    <!-- Поле ввода кода -->
    <textarea
      v-model="code"
      class="code-input custom-scrollbar"
      spellcheck="false"
      @scroll="syncScroll"
    ></textarea>
  </div>
</template>

<style scoped>
.code-editor-container {
  position: relative;
  display: flex;
  height: 100%;
  font-family: 'JetBrains Mono', monospace;
  font-size: 14px;
  line-height: 1.5;
  background: #0d1117;
  color: #e6edf3;
  border: 1px solid #30363d;
  border-radius: 8px;
  overflow: hidden;
}

/* Номера строк */
.line-numbers {
  width: 48px;
  background: #0d1117;
  border-right: 1px solid #30363d;
  color: #6e7681;
  font-size: 12px;
  text-align: right;
  padding: 8px 4px;
  overflow: hidden;
  user-select: none;
}

.line-number {
  height: 21px;
  line-height: 21px;
}

/* Поле кода */
.code-input {
  flex: 1;
  padding: 8px 12px;
  background: #0d1117;
  color: #e6edf3;
  border: none;
  outline: none;
  resize: none;
  white-space: pre;
  overflow: auto;
  font-family: inherit;
  font-size: inherit;
  line-height: inherit;
}

/* Прокрутка */
.code-input::-webkit-scrollbar,
.line-numbers::-webkit-scrollbar {
  width: 6px;
  height: 6px;
}

.code-input::-webkit-scrollbar-thumb {
  background: #30363d;
  border-radius: 3px;
}

.code-input::-webkit-scrollbar-thumb:hover {
  background: #484f58;
}
</style>
