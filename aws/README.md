## Библиотека для работы с AWS sig v4
https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html

## Пример использования

```
#beginScript]
|DV|[headers_sign] = accept: application/json
content-type: application/json; charset=utf-8
user-agent: aws-sdk-iOS/2.26.5 iOS/14.4 en_US
#endScript

// только в POST запросе, base64
|DV|[payload] =

|DV|[key_id] = ASIAUKS2XSL25YK3FDNO
|DV|[secret_key] = DGZ7TAVURXmOixK27NMxOYxVOhE2xyTSER8YqQL2

// нужно не всегда
|DV|[session_token] = IQoJb3JpZ2luX2VjEOH//////////...

// нужно не всегда
|DV|[exp] = 1640801496000

|DV|[auth_header_and_date] = (|DLL|dllName:aws;funcName:sign;params:GET|PDEL|https://access.example.com/info?type=user|PDEL|execute-api|PDEL|us-west-2|PDEL||DV|[headers_sign]|PDEL||DV|[[payload]|PDEL||DV|[key_id]|PDEL||DV|[secret]|PDEL||DV|[session_token]|PDEL||DV|[exp];|DLL|)

|DV|[date] = (|SIMPLEPARS||FROM|"x-amz-date":"|FROM||IN||PARS|[6]|IN||TO|"|TO||SIMPLEPARS|)
|DV|[auth_header] = (|SIMPLEPARS||FROM|"authorization":"|FROM||IN||PARS|[6]|IN||TO|"|TO||SIMPLEPARS|)
|DV|[payload_sign] = (|SIMPLEPARS||FROM|"x-amz-content-sha256":"|FROM||IN||PARS|[6]|IN||TO|"|TO||SIMPLEPARS|)
```

После этого сами заголовки для входа будут +- такие:

```
Accept: application/json
Content-Type: application/json; charset=utf-8
Authorization: |DV|[auth_header]
X-Amz-Date: |DV|[date]
X-Amz-Security-Token: |DV|[session_token]
X-Amz-Content-Sha256: |DV|[payload_sign]
Accept-Language: en-us
Accept-Encoding: gzip, deflate
User-Agent: aws-sdk-iOS/2.26.5 iOS/14.4 en_US
```

## Ремарки и возможные баги

- |DV|[headers_sign] - заголовки, которые нужно подписывать, все они должны быть в финальном запросе
- X-Amz-Content-Sha256 нужен всегда
- post запрос с данными еще не протестирован
