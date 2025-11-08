<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { examples, getRandomExample } from './examples.js'
import SyntaxHighlighter from "./components/SyntaxHighlighter.vue";

const code = ref(`// Добро пожаловать в Dr Script IDE!
print(42);
print(10 + 5);

ver x = 10;
var y = 5;

if x > y {
    print(1);
} else {
    print(0);
}`)

const output = ref('')
const isRunning = ref(false)
const showOutput = ref(false)
const showExamples = ref(false)
const selectedExample = ref(null)

async function run() {
  isRunning.value = true
  output.value = ''
  showOutput.value = true

  try {
    const result = await invoke('run_code', { code: code.value })
    output.value = result || 'Программа выполнена успешно'
  } catch (e) {
    output.value = `❌ Ошибка выполнения:\n${e}`
  } finally {
    isRunning.value = false
  }
}

function clearOutput() {
  output.value = ''
  showOutput.value = false
}

function loadExample(example) {
  code.value = example.code
  showExamples.value = false
  selectedExample.value = example
}

function loadRandomExample() {
  const example = getRandomExample()
  loadExample(example)
}

function toggleExamples() {
  showExamples.value = !showExamples.value
}

function newFile() {
  code.value = '// Новый файл Dr Script\nprint(42);'
  selectedExample.value = null
  showExamples.value = false
}
</script>

<template>
  <div class="min-h-screen bg-[#0D1117] text-gray-100 flex flex-col">
    <!-- Заголовок -->
    <header class="bg-[#161B22] border-b border-[#30363D] px-6 py-4">
      <div class="flex items-center justify-between">
        <div class="flex items-center space-x-4">
          <h1 class="text-xl font-bold text-white flex items-center">
            <span class="text-2xl mr-2">🔬</span>
            Dr Script IDE
          </h1>
          <div class="text-sm text-gray-400">v1.0.0</div>
        </div>

        <div class="flex items-center space-x-3">
          <button
            @click="newFile"
            class="bg-[#21262D] hover:bg-[#30363D] text-gray-300 px-3 py-2 rounded-md text-sm transition border border-[#30363D] hover-lift"
          >
            📄 Новый
          </button>

          <button
            @click="toggleExamples"
            class="bg-[#21262D] hover:bg-[#30363D] text-gray-300 px-3 py-2 rounded-md text-sm transition border border-[#30363D] hover-lift"
            :class="{ 'bg-[#30363D]': showExamples }"
          >
            📚 Примеры
          </button>

          <button
            @click="loadRandomExample"
            class="bg-[#21262D] hover:bg-[#30363D] text-gray-300 px-3 py-2 rounded-md text-sm transition border border-[#30363D] hover-lift"
          >
            🎲 Случайный
          </button>

          <div class="w-px h-6 bg-[#30363D]"></div>

          <button
            @click="clearOutput"
            class="bg-[#21262D] hover:bg-[#30363D] text-gray-300 px-3 py-2 rounded-md text-sm transition border border-[#30363D] hover-lift"
          >
            🗑️ Очистить
          </button>

          <button
            @click="run"
            :disabled="isRunning"
            class="bg-[#238636] hover:bg-[#2EA043] disabled:bg-[#1B4332] text-white px-4 py-2 rounded-md font-medium transition flex items-center space-x-2 disabled:opacity-50 hover-lift"
          >
            <span v-if="isRunning" class="pulse">⏳</span>
            <span v-else>▶️</span>
            <span>{{ isRunning ? 'Выполняется...' : 'Запустить' }}</span>
          </button>
        </div>
      </div>
    </header>

    <!-- Основная область -->
    <div class="flex flex-1 overflow-hidden">
      <!-- Панель примеров -->
      <div
        v-if="showExamples"
        class="w-80 bg-[#0D1117] border-r border-[#30363D] flex flex-col fade-in"
      >
        <div class="bg-[#161B22] border-b border-[#30363D] px-4 py-2 text-sm text-gray-400 flex items-center justify-between">
          <span>📚 Примеры кода</span>
          <button
            @click="showExamples = false"
            class="text-gray-500 hover:text-gray-300 text-lg leading-none"
          >
            ×
          </button>
        </div>

        <div class="flex-1 overflow-auto custom-scrollbar">
          <div class="p-2 space-y-2">
            <div
              v-for="(example, index) in examples"
              :key="index"
              @click="loadExample(example)"
              class="p-3 bg-[#161B22] hover:bg-[#21262D] border border-[#30363D] rounded-lg cursor-pointer transition-all hover-lift"
              :class="{ 'border-blue-500 bg-[#21262D]': selectedExample?.title === example.title }"
            >
              <h3 class="font-medium text-white text-sm mb-1">
                {{ example.title }}
              </h3>
              <p class="text-xs text-gray-400 leading-relaxed">
                {{ example.description }}
              </p>
            </div>
          </div>
        </div>
      </div>

      <!-- Редактор кода -->
      <div class="flex-1 flex flex-col">
        <div class="bg-[#161B22] border-b border-[#30363D] px-4 py-2 text-sm text-gray-400 flex items-center justify-between">
          <div class="flex items-center space-x-2">
            <span>📄</span>
            <span>{{ selectedExample?.title || 'main.dr' }}</span>
            <span v-if="selectedExample" class="text-xs bg-[#30363D] px-2 py-1 rounded">Пример</span>
          </div>
          <span class="text-xs">Dr Script</span>
        </div>

        <div class="flex-1 relative">
          <div class="absolute inset-0 flex">
            <!-- Нумерация строк -->
            <div class="w-12 bg-[#0D1117] border-r border-[#30363D] text-right text-xs text-gray-500 py-4 px-2 font-mono">
              <div v-for="(line, index) in code.split('\n')" :key="index" class="leading-6">
                {{ index + 1 }}
              </div>
            </div>

            <!-- Редактор -->
            <textarea
              v-model="code"
              class="flex-1 bg-[#0D1117] text-gray-100 p-4 font-mono text-sm leading-6 resize-none outline-none border-none"
              style="font-family: 'JetBrains Mono', Consolas, Monaco, monospace;"
              spellcheck="false"
            ></textarea>
          </div>
        </div>
      </div>

      <!-- Панель вывода -->
      <div
        v-if="showOutput"
        class="w-96 bg-[#0D1117] border-l border-[#30363D] flex flex-col fade-in"
      >
        <div class="bg-[#161B22] border-b border-[#30363D] px-4 py-2 text-sm text-gray-400 flex items-center justify-between">
          <span>📊 Вывод программы</span>
          <button
            @click="showOutput = false"
            class="text-gray-500 hover:text-gray-300 text-lg leading-none hover-lift"
          >
            ×
          </button>
        </div>

        <div class="flex-1 overflow-auto custom-scrollbar">
          <pre
            class="p-4 text-sm font-mono leading-relaxed whitespace-pre-wrap"
            :class="{
              'text-green-400': !output.includes('❌') && output,
              'text-red-400': output.includes('❌'),
              'text-gray-500': !output
            }"
          >{{ output || '🚀 Нажмите "Запустить" для выполнения кода...' }}</pre>
        </div>
      </div>
    </div>

    <!-- Статус бар -->
    <footer class="bg-[#161B22] border-t border-[#30363D] px-4 py-2 text-xs text-gray-500 flex items-center justify-between">
      <div class="flex items-center space-x-4">
        <span>Строк: {{ code.split('\n').length }}</span>
        <span>Символов: {{ code.length }}</span>
        <span v-if="selectedExample">Пример: {{ selectedExample.title }}</span>
      </div>
      <div class="flex items-center space-x-4">
        <span class="text-gray-400">Dr Script v1.0</span>
        <div class="flex items-center space-x-2">
          <div
            class="w-2 h-2 rounded-full"
            :class="{
              'bg-green-500': !isRunning,
              'bg-yellow-500 animate-pulse': isRunning
            }"
          ></div>
          <span>{{ isRunning ? 'Выполняется...' : 'Готов к работе' }}</span>
        </div>
      </div>
    </footer>
  </div>
</template>

<style>
html, body, #app {
  margin: 0;
  padding: 0;
  height: 100%;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}

.fade-in {
  animation: fadeIn 0.3s ease-out;
}

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}

.hover-lift {
  transition: transform 0.2s ease;
}

.hover-lift:hover {
  transform: translateY(-1px);
}

.pulse {
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

.custom-scrollbar::-webkit-scrollbar {
  width: 6px;
  height: 6px;
}

.custom-scrollbar::-webkit-scrollbar-track {
  background: #161B22;
}

.custom-scrollbar::-webkit-scrollbar-thumb {
  background: #30363D;
  border-radius: 3px;
}

.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: #484F58;
}

textarea::selection {
  background: rgba(59, 130, 246, 0.3);
}
</style>