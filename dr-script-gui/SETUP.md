# Настройка Dr Script GUI

## Установка зависимостей

Для запуска обновленного GUI необходимо установить новые зависимости:

```bash
cd dr-script-gui
npm install
```

## Запуск в режиме разработки

```bash
npm run tauri dev
```

## Сборка для продакшена

```bash
npm run tauri build
```

## Новые возможности

### 🎨 Улучшенный интерфейс
- Темная тема в стиле GitHub
- Кастомная подсветка синтаксиса для Dr Script
- Анимации и плавные переходы
- Улучшенная типографика

### 📝 Редактор кода
- Monaco Editor с полной поддержкой Dr Script
- Подсветка синтаксиса для ключевых слов, операторов, строк и чисел
- Нумерация строк
- Автоматическое форматирование
- Поддержка горячих клавиш

### 📚 Панель примеров
- 10 готовых примеров кода
- От простых до сложных программ
- Быстрая загрузка примеров
- Случайный выбор примера

### 🖥️ Панель вывода
- Цветной вывод (зеленый для успеха, красный для ошибок)
- Автоматическое открытие при запуске
- Возможность скрытия/показа
- Прокрутка длинного вывода

### 📊 Статус бар
- Информация о количестве строк и символов
- Индикатор состояния (готов/выполняется)
- Название текущего примера
- Версия языка

## Горячие клавиши

- `Ctrl+Enter` - Запуск кода
- `Ctrl+N` - Новый файл
- `Ctrl+Shift+E` - Показать/скрыть примеры
- `Ctrl+Shift+O` - Показать/скрыть вывод

## Структура файлов

```
src/
├── App.vue          # Основной компонент
├── main.js          # Точка входа
├── examples.js      # Примеры кода
└── assets/
    └── styles.css   # Дополнительные стили
```

## Кастомизация

### Добавление новых примеров

Отредактируйте файл `src/examples.js`:

```javascript
export const examples = [
  {
    title: "Название примера",
    description: "Описание примера",
    code: `// Код примера
print("Hello, World!");`
  },
  // ... другие примеры
];
```

### Изменение темы

Отредактируйте цвета в `src/App.vue` в функции `registerDrScriptLanguage()`:

```javascript
monaco.editor.defineTheme('drscript-dark', {
  base: 'vs-dark',
  inherit: true,
  rules: [
    { token: 'keyword', foreground: 'C586C0', fontStyle: 'bold' },
    // ... другие правила
  ],
  colors: {
    'editor.background': '#0D1117',
    // ... другие цвета
  }
})
```

## Возможные проблемы

### Monaco Editor не загружается
Убедитесь, что установлена правильная версия:
```bash
npm install monaco-editor@^0.45.0
```

### Ошибки сборки
Очистите кеш и переустановите зависимости:
```bash
rm -rf node_modules package-lock.json
npm install
```

### Проблемы с Tauri
Убедитесь, что установлены все системные зависимости Tauri:
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

# Arch Linux
sudo pacman -S webkit2gtk base-devel curl wget openssl gtk3 libayatana-appindicator librsvg
```