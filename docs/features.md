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

Запрашивает ввод от пользователя. Можно передать сообщение-подсказку:

```dr
ver name = input("Введите имя: ");
print("Привет, " + name);

ver age = input(); // Без сообщения
print(age);
```