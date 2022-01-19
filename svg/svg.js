var svgCaptcha = require('svg-captcha');

const express = require('express')
const app = express()
const port = 3000

app.get('/captcha', function (req, res) {
    var captcha = svgCaptcha.createMathExpr({mathMin: 1, mathMax: 1256});
    
    res.type('svg');
    res.status(200).send(captcha.data);
});

app.listen(port, () => {
  console.log(`Example app listening at http://localhost:${port}`)
})
