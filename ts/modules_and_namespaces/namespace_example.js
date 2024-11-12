"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const utils_1 = require("./utils");
// 이메일 주소 유효성 검사
const email = 'test@example.com';
if (utils_1.Utils.isValidEmail(email)) {
    console.log(`이메일 ${email}은 유효한 형식입니다.`);
}
else {
    console.log(`이메일 ${email}은 유효하지 않은 형식입니다.`);
}
// 유저 이름 유효성 검사
const userName = '이순신';
if (utils_1.Utils.isValidUserName(userName)) {
    console.log(`유저 이름 ${userName}은 유효합니다.`);
}
else {
    console.log(`유저 이름 ${userName}은 유효하지 않습니다.`);
}
