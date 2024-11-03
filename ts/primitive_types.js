"use strict";
// 문자열 타입 
const greeting = '안녕, 타입스크립트!';
console.log(greeting);
// 숫자 타입 
const year = 2024;
const pi = 3.14;
console.log(`올해는 ${year}년입니다.`);
console.log(`파이 값은 ${pi}입니다.`);
// 불리언 타입
const isCompleted = true;
console.log(`작업 완료 여부: ${isCompleted}`);
// null: 명시적으로 '값이 없음'을 나타냄
// undefined: 선언되었으나 아직 초기화되지 않은 상태 
let value = null;
let notInitialized = undefined;
console.log(`value의 값: ${value}`);
console.log(`notInitialized 값: ${notInitialized}`);
// 심볼 타입 (Symbol Type)
// 심볼은 고유하고 변경 불가능한 값을 생성하기 위한 데이터 타입
const uniqueKey1 = Symbol('key1');
const uniqueKey2 = Symbol('key2');
console.log(uniqueKey1 === uniqueKey2); // false, 심볼은 항상 고유한 값을 가짐
// 객체에서 심볼을 키로 사용하는 예제 
const user = {
    [uniqueKey1]: '사용자 1',
    [uniqueKey2]: '사용자 2'
};
console.log(user[uniqueKey1]); // '사용자 1' 출력
console.log(user[uniqueKey2]); // '사용자 2' 출력
// bigint는 매우 큰 정수를 다룰 때 사용함
// 일반 숫자 타입(number) 한계를 초과하는 정수 표현 가능 
// tsconfig.json 파일에서 "target": "ES2020" 이상으로 설정 필요 
const largeNumber = 1234567890123456789012345678901234567890n;
console.log(`큰 숫자: ${largeNumber}`);
// 타입스크립트는 컴파일 타임에 타입을 확인해 오류 예방
let width = 100;
// width = '100'; // 오류 발생
function calculateArea(height, width) {
    // 면적 계산 함수, 매개변수 타입을 명시해 타입 오류 예방
    return height * width;
}
const height = 20;
const area = calculateArea(height, width);
console.log(`면적: ${area}`);
