Постарался собрать все известные мне шифровки/хэши/kdf в одном плагине

## Возможности
    - AES с разными паддингами (пока только CBC и ECB)
    - Экспорт делителей (modulus) публичного RSA ключа в pem формат 
    - RSA pkcs1v15 и OAEP, а также подпись (pkcs1 или pss)
    - Хэши очень многих видов, а также hmac
    - bcrypt, соль нужно передавать самому
    - scrypt, поддержка всех параметров
    - pbkdf2
    - функция для генерации рандомных байтов (для соли/ключей шифрования)
    - blowfish

## Особенности
 - Данные для шифрования/хэширования, а так же ключ передавать в base64
 - Если какой-либо параметр для вызова функции нужно пропустить - не убирайте |PDEL|, просто ничего не ставьте на это место
   
   Пример: `(|DLL|dllName:crypto;funcName:some_func;params:первый|PDEL||PDEL|третий;|DLL|)`. Тут второй параметр не обязателен, но пропускать его нельзя
 - Ошибки при вызове функии возвращаются в формате `ERR|описание ошибки`

## Использование
<details><summary>AES</summary>

```
******************
Используем CBC или ECB (для него iv не нужен)
Паддинги: - pkcs7
          - zero
          - iso7816
          - ansi_x923
Параметры: - шифруемый текст или зашифрованный если aes_decrypt
           - ключ
           - iv (для ECB не нужен)
           - Mode
           - паддинг
******************
|DV|[data] = (|BASE64|test data|BASE64|)
|DV|[aes_key] = Ju1DB7Dfa5Wjtbp3CTSQmS2PbgjMmarq2BZWCWFwPuY=
|DV|[aes_iv] = wsr+7AkmUy9j30UjIQV6Xw==
|DV|[encrypted] = (|DLL|dllName:crypto;funcName:aes_encrypt;params:|DV|[data]|PDEL||DV|[aes_key]|PDEL||DV|[aes_iv]|PDEL|cbc|PDEL|pkcs7;|DLL|)
|DV|[decrypted_base64] = (|DLL|dllName:crypto;funcName:aes_decrypt;params:|DV|[encrypted]|PDEL||DV|[aes_key]|PDEL||DV|[aes_iv]|PDEL|cbc|PDEL|pkcs7;|DLL|)
|DV|[decrypted] = (|DLL|dllName:encoding;funcName:b64_decode;params:|DV|[decrypted_base64];|DLL|)
```

</details>

<details><summary>Экспорт modulus в pem</summary>

```
******************
Все параметры в base64
Параметры: - N (длинный)
           - E (короткий, дефолтный - 010001)
******************
|DV|[N] = (|DLL|dllName:encoding;funcName:hex_to_b64;params:cd9e82d72fe848af8e6fceb9696be6fc359e61b65b9a4921a649723c37786c2815fc25054e6f8160919299c8be6f981a956b0adb70c81f6db4896613e545d64d43e035b797fdef3374632db00f994774bf332c7afafe9aefc48a3d07b63c640cfa61dc3f04e1cade68a63a52749f0bc4c2c59121defc779fd0ffead520fcf649;|DLL|) // N
|DV|[E] = (|DLL|dllName:encoding;funcName:hex_to_b64;params:010001;|DLL|) // exp
|DV|[pem] = (|DLL|dllName:crypto;funcName:rsa_pem_from_modulus;params:|DV|[N]|PDEL||DV|[E];|DLL|)
```

</details>

<details><summary>RSA (шифровка, дешифровка и подпись)</summary>

```
******************
Сообщение в base64
Параметры: - шифруемый текст или зашифрованный если rsa_decrypt
           - ключ в pem формате
           - хэш (только если хотите использовать OAEP) НЕ ПРОПУСКАЙТЕ ЭТОТ ПАРАМЕТР
******************
#beginScript
|DV|[pub_pem] = -----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDNnoLXL+hIr45vzrlpa+b8NZ5h
tluaSSGmSXI8N3hsKBX8JQVOb4FgkZKZyL5vmBqVawrbcMgfbbSJZhPlRdZNQ+A1
t5f97zN0Yy2wD5lHdL8zLHr6/prvxIo9B7Y8ZAz6Ydw/BOHK3mimOlJ0nwvEwsWR
Id78d5/Q/+rVIPz2SQIDAQAB
-----END PUBLIC KEY-----
#endScript
|DV|[data] = (|BASE64|test data|BASE64|)
|DV|[encrypted] = (|DLL|dllName:crypto;funcName:rsa_encrypt;params:|DV|[data]|PDEL||DV|[pub_pem]|PDEL|sha384;|DLL|)

#beginScript
|DV|[priv_pem] = -----BEGIN RSA PRIVATE KEY-----
MIICXQIBAAKBgQDNnoLXL+hIr45vzrlpa+b8NZ5htluaSSGmSXI8N3hsKBX8JQVO
b4FgkZKZyL5vmBqVawrbcMgfbbSJZhPlRdZNQ+A1t5f97zN0Yy2wD5lHdL8zLHr6
/prvxIo9B7Y8ZAz6Ydw/BOHK3mimOlJ0nwvEwsWRId78d5/Q/+rVIPz2SQIDAQAB
AoGAFhKxbQmhipv9/cFYD28b6XCVbgaLfYaNltzvkcif+XcG1SiMPJ1PwDQgZA3e
vlAPxSvWizQSPP15PKlj8rWKiajFRvdNFzyH6D7bw2YNcS0LOvcS1zBgA/VxmsxI
x5cUTO3lNsvcIqdcoJQ8ERY1+FWfxH0IUBdMzgivCFnJyZkCQQD395DFwp8HbGSN
qW3fngUAQ5d3TBrY0NwDa8d8pSCbNLrv3vG2ygB7YaPGd8+1Tuqpc/BFBs1T3UQc
cRb7LFCjAkEA1Ee/Suicx0D7KYYFIq52Asru50J9vDJgaBss6tScxau7sLFGYGYB
bGSgE8RTqUTG2jgB3hncC0yOlL3+KQlQIwJAQ+Ss5vjawhWTkpYJV2jUxbW8CxXz
Y9oL44PnIuGzg8t0Q6kvVXUJnL6nMPgtDt+EsNDlwICUR5oVxBPSzwLbMQJBAIcn
fxW0cE00hDT1zUM9jIlOpzi6Ts+Jy3O9CaYh5Aa+xmtDEynBCFr43ip3r0RwM6Mw
UZAVKtJO1eDB7pY2Bb0CQQCU74lVEuqWZbG8dPYW6KVDvG3aqD1hslZGr+0YFQNb
xPOWMpx0oEFvdZQfKcvH31v8hAqgtyr/EwKu/wtiiDnW
-----END RSA PRIVATE KEY-----
#endScript
|DV|[decrypted_base64] = (|DLL|dllName:crypto;funcName:rsa_decrypt;params:|DV|[encrypted]|PDEL||DV|[priv_pem]|PDEL|sha384;|DLL|)
|DV|[decrypted] = (|DLL|dllName:encoding;funcName:b64_decode;params:|DV|[decrypted_base64];|DLL|)

******************
RSA Sign
Сообщение в base64
Параметры: - подписываемый текст
           - приватный ключ в pem формате
           - хэш, нужен всегда
           - алгоритм подписи pkcs1 или pss
******************
|DV|[singed] = (|DLL|dllName:crypto;funcName:rsa_sign;params:|DV|[data]|PDEL||DV|[priv_pem]|PDEL|sha384|PDEL|pkcs1;|DLL|)
```

</details>

<details><summary>Хэширование и hmac</summary>

```
******************
Hash
Сообщение в base64
Параметры: - алгоритм хеширования
           - сообщение
******************
|DV|[data] = (|BASE64|test data|BASE64|)
|DV|[hashed] = (|DLL|dllName:crypto;funcName:hash;params:keccak256|PDEL||DV|[data];|DLL|)

******************
Hmac
Сообщение и ключ в base64
Параметры: - алгоритм хеширования
           - сообщение
           - ключ
******************
|DV|[data] = (|BASE64|test data|BASE64|)
|DV|[secretkey] = (|BASE64|secretkey|BASE64|)
|DV|[hashed] = (|DLL|dllName:crypto;funcName:hmac;params:keccak256|PDEL||DV|[data]|PDEL||DV|[secretkey];|DLL|)
```

</details>

<details><summary>Генерация рандомных байтов</summary>

```
******************
Параметры: - нужное колличество рандомных байтов
******************
|DV|[random_bytes] = (|DLL|dllName:crypto;funcName:random_bytes;params:16;|DLL|)
```

</details>

<details><summary>Bcrypt</summary>

```
******************
Параметры: - сообщение
           - количество раундом (повторений) 
           - соль
******************
|DV|[data] = (|BASE64|test data|BASE64|)
|DV|[random_salt] = (|DLL|dllName:crypto;funcName:random_bytes;params:16;|DLL|)
|DV|[hashed] = (|DLL|dllName:crypto;funcName:bcrypt;params:|DV|[data]|PDEL|11|PDEL||DV|[random_salt];|DLL|)
```

</details>

<details><summary>Scrypt</summary>

```
******************
Параметры: - сообщение
           - N - параметр, задающий сложность, степень двойки количества повторений (log2)
           - r - (параметр, задающий размер блока, оптимально 8)
           - p - (степень параллельностиб оптимально 1)
           - размер хэшированного сообщения на выходе
           - соль
******************
|DV|[data] = (|BASE64|test data|BASE64|)
|DV|[random_salt] = (|DLL|dllName:crypto;funcName:random_bytes;params:16;|DLL|)
|DV|[hashed] = (|DLL|dllName:crypto;funcName:scrypt;params:|DV|[data]|PDEL|11|PDEL|8|PDEL|1|PDEL|64|PDEL||DV|[random_salt];|DLL|)
```

</details>

<details><summary>Pbkdf2</summary>

```
******************
Параметры: - сообщение
           - соль
           - параметр, задающий сложность, количество повторений
           - размер хэшированного сообщения на выходе
           - тип хэша
******************
|DV|[data] = (|BASE64|test data|BASE64|)
|DV|[random_salt] = (|DLL|dllName:crypto;funcName:random_bytes;params:16;|DLL|)
|DV|[hashed]= (|DLL|dllName:crypto;funcName:pbkdf2;params:|DV|[data]|PDEL||DV|[random_salt]|PDEL|10000|PDEL|32|PDEL|md5;|DLL|)
```

</details>

<details><summary>Blowfish</summary>

```
******************
Используем CBC или ECB (для него iv не нужен)
Паддинги: - pkcs7
          - zero
          - iso7816
          - ansi_x923
Параметры: - шифруемый текст или зашифрованный если aes_decrypt
           - ключ - 8 байтов
           - iv - 8 байтов (для ECB не нужен)
           - Mode
           - паддинг
******************
|DV|[data] = (|BASE64|test data|BASE64|)
|DV|[blowfish_key] = Ld0Ydw/qj0k=
|DV|[blowfish_iv] = djFUivAKUUs=
|DV|[encrypted] = (|DLL|dllName:crypto;funcName:blowfish_encrypt;params:|DV|[data]|PDEL||DV|[blowfish_key]|PDEL||DV|[blowfish_iv]|PDEL|cbc|PDEL|pkcs7;|DLL|)
|DV|[decrypted_base64] = (|DLL|dllName:crypto;funcName:blowfish_decrypt;params:|DV|[encrypted]|PDEL||DV|[blowfish_key]|PDEL||DV|[blowfish_iv]|PDEL|cbc|PDEL|pkcs7;|DLL|)
|DV|[decrypted] = (|DLL|dllName:encoding;funcName:b64_decode;params:|DV|[decrypted_base64];|DLL|)
```

</details>

## Скачать здесь
https://github.com/Numenorean/pkdll-rs/raw/main/crypto/target/i686-pc-windows-msvc/release/crypto.dll
