Небольшая либа для декодирования base64, кодирования и декодирования hex, а так же перевод из hex в base64 и наоборот

## Использование
```
|PARS|[1] = (|DLL|dllName:encoding;funcName:hex_encode;params:тест;|DLL|)
|PARS|[2] = (|DLL|dllName:encoding;funcName:hex_decode;params:|PARS|[1];|DLL|)

|PARS|[3] = (|DLL|dllName:encoding;funcName:b64_encode;params:тест;|DLL|)
|PARS|[4] = (|DLL|dllName:encoding;funcName:b64_decode;params:|PARS|[3];|DLL|)

|PARS|[5] = (|DLL|dllName:encoding;funcName:hex_to_b64;params:|PARS|[1];|DLL|)
|PARS|[6] = (|DLL|dllName:encoding;funcName:b64_to_hex;params:|PARS|[3];|DLL|)
```

## Баги
Киппер по какой-то причине не может закодировать кириллицу в base64, так что в таких случаях используйте b64_encode
