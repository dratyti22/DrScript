// lib/languages/dr_script.dart (Обновлено)

import 'package:highlight/highlight.dart';

final drScript = Mode(
  refs: {
    'subst': Mode(
      className: 'subst',
      variants: [
        Mode(begin: r'\$\{', end: r'\}'),
        Mode(begin: r'\$\[', end: r'\]'),
      ],
    ),
  },
  keywords: {
    'keyword': 'fun ver var if else while for return',
    'literal': 'true false',
    'built_in': 'print len input',
  },
  contains: [
    // 1. ПУНКТУАЦИЯ (Выделение структурных символов)
    // Делаем это с высоким приоритетом, чтобы захватить символы до ключевых слов.
    Mode(
      className: 'punctuation',
      // Захватываем ; {} () [], и запятые/двоеточия
      begin: r'[;{}\(\)\[\],:]',
      relevance: 1, // Немного увеличиваем приоритет
    ),

    // 2. Операторы (Например: +, -, *, /, <, >, =)
    // Присваивание (=) и сравнение (==, >=, <=) должны быть здесь.
    Mode(
      className: 'operator',
      // Захватываем все, что выглядит как оператор (кроме тех, что уже в пунктуации)
      begin: r'[-+*/%^]=?|==|!=|<=|>=|[<>]',
      relevance: 1,
    ),

    // 3. Строки
    Mode(
      className: 'string',
      begin: "'",
      end: "'",
      illegal: r'\n',
      contains: [Mode(ref: 'subst')],
    ),
    Mode(
      className: 'string',
      begin: '"',
      end: '"',
      illegal: r'\n',
      contains: [Mode(ref: 'subst')],
    ),

    // 4. Комментарии
    Mode(
      className: 'comment',
      begin: '//',
      end: '\$',
      contains: [
        Mode(
          className: 'doctag',
          begin: r'[ ]*(?:TODO|FIXME|NOTE|BUG|XXX):',
          relevance: 0,
        ),
      ],
    ),

    // 5. Числа
    Mode(
      className: 'number',
      begin: r'\b\d+(\.\d*)?([eE][+-]?\d+)?\b',
      relevance: 0,
    ),
  ],
);