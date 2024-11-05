"use strict";
// 리터럴 타입은 고정된 특정 값만 가질 수 있는 타입
// 문자열 리터럴 타입
let switchState = 'on';
console.log(`현재 스위치 상태: ${switchState}`); // 현재 스위치 상태: on
// 숫자 리터럴 타입
// 특정 숫자 값들만 허용되는 타입을 정의
let diceRoll;
diceRoll = 3;
console.log(`주사위 값: ${diceRoll}`);
// 더 복잡한 타입
let userResponse;
userResponse = 'yes';
userResponse = true;
console.log(`사용자 응답: ${userResponse}`);
// 리터럴 타입을 사용하는 함수 
// 매개변수로 리터럴 타입을 지정해 함수 호출 시 잘못된 인자가 전달되지 않도록 방지할 수 있음
function setAlignment(alignment) {
    console.log(`텍스트 정렬: ${alignment}`);
}
setAlignment('left');
setAlignment('center');
// 올바른 리터럴 타입을 가진 객체 생성
let myDevice = {
    type: 'laptop',
    brand: 'Apple'
};
console.log(`기기타입: ${myDevice.type}, 브랜드: ${myDevice.brand}`);
// 잘못된 타입을 지정하면 오류 발생
// myDevice.type = 'desktop';
// 조건문을 사용해 리터럴 타입을 좁힐 수 있음
function printLightStatus(status) {
    if (status === 'red') {
        console.log(`정지`);
    }
    else if (status === 'green') {
        console.log(`출발`);
    }
    else {
        console.log(`주의`);
    }
}
printLightStatus('red');
printLightStatus('green');
