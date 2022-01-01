## Библиотека для работы со временем

## Примеры использования

- timestamp в строку

```
|DV|[timestamp] = |TIMESTAMP|

// строка форматирования (про мараметры можно почитать здесь https://docs.rs/chrono/latest/chrono/format/strftime/index.html)
|DV|[format] = %Y-%m-%dT%H:%M:%S%:z

// часовой пояс в секундах (может быть отрицательным) или local - часовой пояс машины
|DV|[timezone] = -3600

|DV|[formatted] = (|DLL|dllName:time;funcName:format;params:|DV|[timestamp]|PDEL||DV|[format]|PDEL||DV|[timezone];|DLL|)
```

- распарсить строку в timestamp

```
|DV|[formatted] = 2022-01-01T16:03:01-01:00

// строка форматирования
|DV|[format] = %Y-%m-%dT%H:%M:%S%:z

|DV|[timestamp] = (|DLL|dllName:time;funcName:parse;params:|DV|[formatted]|PDEL||DV|[format];|DLL|)
```

## Ремарки и возможные баги

- распарсить строку в timestamp можно только с заданным шаблоном
