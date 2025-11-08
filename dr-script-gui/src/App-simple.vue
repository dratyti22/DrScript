<template>
  <div style="background: #0D1117; color: white; padding: 20px; height: 100vh; font-family: Arial, sans-serif;">
    <h1>Dr Script IDE - Test</h1>
    <textarea 
      v-model="code" 
      style="width: 100%; height: 200px; background: #161B22; color: white; border: 1px solid #30363D; padding: 10px; font-family: monospace;"
    ></textarea>
    <br><br>
    <button 
      @click="run" 
      style="background: #238636; color: white; padding: 10px 20px; border: none; border-radius: 5px; cursor: pointer;"
    >
      Запустить
    </button>
    <br><br>
    <pre style="background: #161B22; padding: 10px; border: 1px solid #30363D; white-space: pre-wrap;">{{ output }}</pre>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const code = ref('print(42);')
const output = ref('Готов к работе...')

async function run() {
  try {
    output.value = 'Выполняется...'
    const result = await invoke('run_code', { code: code.value })
    output.value = result || 'Программа выполнена успешно'
  } catch (e) {
    output.value = `Ошибка: ${e}`
  }
}
</script>