Функции для рандомизации чего-либо

## Использование

Перемешивание массива
```
#beginScript
|DV|[array] = ["a", "b", "c", 1]

|DV|[shffled] = (|DLL|dllName:random;funcName:shuffle;params:|DV|[array];|DLL|)
|DV|[random_value] = (|DLL|dllName:random;funcName:choice;params:|DV|[array];|DLL|)

|DV|[random_int_value] = (|DLL|dllName:random;funcName:range;params:1|PDEL|10;|DLL|)

Последний аргумент - количество знаков после запятой
|DV|[random_float_value] = (|DLL|dllName:random;funcName:rangef;params:1.6|PDEL|10.3|PDEL|3;|DLL|)

|DV|[uuid] = (|DLL|dllName:random;funcName:uuidv4;params:;|DLL|)

Заполняем строку другой строкой/символом до определенной длинны (число 5), 0 - откуда начинать заполнение
Здесь мы заполняем строку-число нулями слева до длинны 5
Полезно когда нужно перебирать код
|DV|[filled] = (|DLL|dllName:random;funcName:fill_with;params:25|PDEL|0|PDEL|5|PDEL|0;|DLL|)
```