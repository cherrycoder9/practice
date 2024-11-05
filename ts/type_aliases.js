"use strict";
// 타입 앨리어스를 이용해 주어진 정보를 포함한 사용자 목록을 출력하는 함수
function printUserList(users) {
    console.group(`사용자 목록 출력`);
    users.forEach((user, index) => {
        console.log(`사용자 ${index + 1}, 이름: ${user.name}, 이메일: ${user.email}, 나이:${user.age}`);
        if (user.phoneNumber) {
            console.log(`전화번호: ${user.phoneNumber}`);
        }
    });
    console.groupEnd();
}
// 평균 계산 함수 정의
const calculateAverage = (numbers) => {
    const sum = numbers.reduce((acc, val) => acc + val, 0); // 배열의 모든 요소의 합을 계산
    return numbers.length > 0 ? sum / numbers.length : 0; // 평균 값을 반환 
};
// 제네릭 타입을 사용한 API 응답 생성 함수 
function createApiResponse(data, status) {
    return {
        status,
        data,
        error: status === 'suspended' ? '사용자 접근 제한' : undefined,
    };
}
// 실행
const users = [
    { name: "홍길동", email: "hong@x.com", age: 25 },
    { name: "이몽룡", email: "lee@x.com", age: 35, phoneNumber: "010-3333-3333" },
];
printUserList(users);
const averageAge = calculateAverage(users.map((user) => user.age));
console.log(`사용자들 평균 나이: ${averageAge}`);
const response = createApiResponse(users, 'active');
// console.log(`API 응답: ${response}`);
// 객체의 내부 구조를 직렬화해서 보기 좋게 출력해야 함.
// null: JSON 변환 과정에서 모든 속성을 변환 대상으로 사용한다는 의미
// 2: JSON 문자열의 들여쓰기 수준
console.log(`API 응답: ${JSON.stringify(response, null, 2)}`);
// ReadonlyUserInfo 타입 사용
const readonlyUser = {
    name: "성춘향",
    email: "sung@x.com",
    age: 24,
    phoneNumber: "010-1234-5678",
};
// readonlyUser.age = 25; // 읽기 전용 속성이므로 수정 불가 
// console.log(`읽기 전용 사용자 정보: ${readonlyUser}`);
console.log(`읽기 전용 사용자 정보: ${JSON.stringify(readonlyUser, null, 2)}`);
