Плагин для получения значений из json строки посредством указания пути к значению

## Использование
```
#beginScript
|DV|[json] = {
  "name": {"first": "Tom", "last": "Anderson"},
  "age":37,
  "children": [
      {"first": "Sara"},
      {"first": "John"},
      {"first": "Len"},
  ],
  "fav.movie": "Deer Hunter",
  "friends": [
    {"first": "Dale", "last": "Murphy", "age": 44, "nets": ["ig", "fb", "tw"]},
    {"first": "Roger", "last": "Craig", "age": 68, "nets": ["fb", "tw"]},
    {"first": "Jane", "last": "Murphy", "age": 47, "nets": ["ig", "tw"]}
  ]
}
#endScript
/*
Нужно получить имена только друзей
*/
|DV|[friends] = (|DLL|dllName:json;funcName:get;params:|DV|[json]|PDEL|friends.#.first;|DLL|)
```

## Подробное описание всех возможных параметров 
https://github.com/tidwall/gjson.rs