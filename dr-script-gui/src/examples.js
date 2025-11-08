// Примеры кода для Dr Script (только числа)

export const examples = [
  {
    title: "Привет, мир!",
    description: "Простой пример вывода чисел",
    code: `// Простой пример
print(42);
print(123);

ver x = 10;
print(x);`
  },
  
  {
    title: "Переменные и операторы",
    description: "Работа с переменными и арифметическими операторами",
    code: `// Переменные
ver x = 10;        // мутабельная переменная
var y = 5;         // немутабельная переменная

print(x);
print(y);

// Арифметические операции
print(x + y);
print(x - y);
print(x * y);
print(x / y);

// Префиксные и постфиксные операторы
print(++x);  // x становится 11
print(x++);  // выводит 11, x становится 12
print(x);    // x теперь 12`
  },
  
  {
    title: "Условные операторы",
    description: "Использование if-else конструкций",
    code: `ver age = 18;

if age >= 18 {
    print(1);  // совершеннолетний
    
    if age >= 65 {
        print(2);  // пенсионер
    } else {
        print(3);  // трудоспособный
    }
} else {
    print(0);  // несовершеннолетний
}
`
  },
  
  {
    title: "Циклы while",
    description: "Использование цикла while",
    code: `// Цикл while - счетчик
ver i = 1;

while i <= 5 {
    print(i);
    i = i + 1;
}

// Цикл while - сумма чисел
ver sum = 0;
ver n = 1;

while n <= 10 {
    sum = sum + n;
    n = n + 1;
}

print(sum);  // сумма от 1 до 10`
  },
  
  {
    title: "Циклы for",
    description: "Использование цикла for",
    code: `// Простой цикл for
for (i = 1; i <= 10; i++) {
    print(3 * i);  // таблица умножения на 3
}

// Вложенные циклы
for (i = 1; i <= 3; i++) {
    for (j = 1; j <= 3; j++) {
        print(i * j);
    }
}`
  },
  
  {
    title: "Функции",
    description: "Определение и использование функций",
    code: `// Функция с вычислениями
fun square(x) {
    return x * x;
}

fun cube(x) {
    return x * x * x;
}

ver num = 5;
print(num);
print(square(num));
print(cube(num));

// Функция с условием
fun max(a, b) {
    if a > b {
        return a;
    } else {
        return b;
    }
}

print(max(10, 7));`
  },
  
  {
    title: "Рекурсия - Факториал",
    description: "Рекурсивная функция для вычисления факториала",
    code: `// Рекурсивная функция факториала
fun factorial(n) {
    if n <= 1 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

for (i = 1; i <= 6; i++) {
    print(factorial(i));
}`
  },
  
  {
    title: "Рекурсия - Числа Фибоначчи",
    description: "Рекурсивная функция для чисел Фибоначчи",
    code: `// Числа Фибоначчи
fun fibonacci(n) {
    if n <= 1 {
        return n;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

for (i = 0; i < 10; i++) {
    print(fibonacci(i));
}`
  },
  
  {
    title: "Сложный пример",
    description: "Программа для проверки простых чисел",
    code: `// Проверка простых чисел
fun isPrime(n) {
    if n <= 1 {
        return 0;  // false
    }
    
    if n <= 3 {
        return 1;  // true
    }
    
    if n % 2 == 0 {
        return 0;  // false
    }
    
    ver i = 3;
    while i * i <= n {
        if n % i == 0 {
            return 0;  // false
        }
        i = i + 2;
    }
    
    return 1;  // true
}

for (num = 1; num <= 30; num++) {
    if isPrime(num) {
        print(num);  // простые числа
    }
}`
  },
  
  {
    title: "Игра - Угадай число",
    description: "Простая игра на угадывание числа",
    code: `// Игра "Угадай число"
ver secretNumber = 42;
ver attempts = 0;

fun checkGuess(secret, userGuess) {
    if userGuess == secret {
        return 0;  // угадал
    } else {
        if userGuess < secret {
            return -1;  // меньше
        } else {
            return 1;   // больше
        }
    }
}

// Симуляция нескольких попыток
ver guesses = 35;
for (i = 0; i < 5; i++) {
    attempts = attempts + 1;
    ver result = checkGuess(secretNumber, guesses);
    
    print(guesses);  // текущая попытка
    
    if result == 0 {
        print(attempts);  // количество попыток
        break;
    } else {
        if result == -1 {
            guesses = guesses + 5;
        } else {
            guesses = guesses - 3;
        }
    }
}`
  }
];

export function getRandomExample() {
  return examples[Math.floor(Math.random() * examples.length)];
}

export function getExampleByIndex(index) {
  return examples[index] || examples[0];
}