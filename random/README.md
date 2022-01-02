Функции для рандомизации чего-либо

## Использование

Перемешивание массива

```
|DV|[array] = ["a", "b", "c", 1]

|DV|[shffled] = (|DLL|dllName:random;funcName:shuffle;params:|DV|[array];|DLL|)
```
Рандомное значение из массива

```
|DV|[random_value] = (|DLL|dllName:random;funcName:choice;params:|DV|[array];|DLL|)
```
Рандомное целое число от и до

```
|DV|[random_int_value] = (|DLL|dllName:random;funcName:range;params:1|PDEL|10;|DLL|)\
```
Рандомное дробное число от и до с возможностью округления

```
|DV|[random_float_value] = (|DLL|dllName:random;funcName:rangef;params:1.6|PDEL|10.3|PDEL|3;|DLL|)
```
UUID v4

```
|DV|[uuid] = (|DLL|dllName:random;funcName:uuidv4;params:;|DLL|)
```
Заполняем строку другой строкой/символом до определенной длинны

25 - Исходная строка

0 - чем заполняем

5 - финальная длинна

0 - по какому индексу заполняем (0 - начало строки)


```
|DV|[filled] = (|DLL|dllName:random;funcName:fill_with;params:25|PDEL|0|PDEL|5|PDEL|0;|DLL|)
```

Рандомная строка по шаблону (регулярка)

Особенность: чтобы генерить и по русским буквам нужно включить это возможность таким  образом, где `u` - это флаг `(?u:[а-я])`

В остальном - все так же

Подробнее про синтаксис здесь: https://docs.rs/regex/latest/regex/

```
|DV|[random_str] = (|DLL|dllName:random;funcName:rand_regex;params:(?u:[а-я0-9]){32};|DLL|)
```