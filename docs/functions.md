# Встроенные функции

### print()

Выводит значение в консоль:

```dr
print("Hello, World!");
print(42);
```

### len()

Возвращает длину строки в байтах:

```dr
var text = "Hello";
print(len(text)); // вывод: 5
ver text2 = "Привет";
print(len(text2)); // вывод: 12
```

### input()

Запрашивает ввод от пользователя (возвращает строку). Можно передать сообщение-подсказку:

```dr
ver name = input("Введите имя: ");
print("Привет, " + name);

ver age = input(); // Без сообщения
print(age);
```

### inputint()

Запрашивает ввод числа от пользователя. Можно передать сообщение-подсказку:

```dr
ver age = inputint("Введите возраст: ");
print("Вам " + str(age) + " лет");

ver num = inputint(); // Без сообщения
print(num + 10);
```

### random()

Генерирует случайное целое число в диапазоне [min, max):

```dr
ver dice = random(1, 7); // от 1 до 6
print("Выпало: " + str(dice));

ver coin = random(0, 2); // 0 или 1
```

### chars()

Возвращает количество символов (не байтов) в строке:

```dr
var text = "Hello";
print(chars(text)); // вывод: 5

ver text2 = "Привет";
print(chars(text2)); // вывод: 6
```

### int()

Преобразует строку в целое число:

```dr
var text = "123";
ver num = int(text);
print(num + 10); // вывод: 133
```

### str()

Преобразует число в строку:

```dr
var num = 42;
ver text = str(num);
print("Число: " + text); // вывод: Число: 42
```

### time()

Возвращает текущее время в миллисекундах (Unix timestamp):

```dr
ver start = time();
// ... какой-то код ...
ver end = time();
print("Прошло: " + str(end - start) + " мс");
```

### type()

Возвращает тип значения в виде строки:

```dr
print(type(42));        // вывод: "int"
print(type("hello"));   // вывод: "string"
```
