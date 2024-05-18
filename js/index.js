const express = require('express');
const app = express();

app.get('/', function (req, res) {
    res.send("hello");
});

app.get('/user/:id', function (req, res) {
    const q = req.params;
    console.log(q);
    res.send(q);
});

app.listen(3000);