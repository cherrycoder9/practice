"use strict";
// 유저 데이터와 관련된 함수들을 제공하는 모듈
// 유저 정보를 저장하고 검색하는 함수 정의
Object.defineProperty(exports, "__esModule", { value: true });
exports.addUser = addUser;
exports.getUserById = getUserById;
exports.getAllUsers = getAllUsers;
// 유저 데이터를 저장할 배열 (실제 환경에선 DB)
let users = [];
// 새로운 유저를 추가하는 함수
function addUser(user) {
    users.push(user);
    console.log(`${user.name} 유저가 추가되었습니다.`);
}
// 유저 ID를 통해 유저 정보를 검색하는 함수
function getUserById(id) {
    return users.find(user => user.id === id);
}
// 모든 유저 리스트를 반환하는 함수
function getAllUsers() {
    return users;
}
