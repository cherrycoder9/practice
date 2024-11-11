"use strict";
// 열거형(Enum)
// 여러 관련된 상수를 하나의 그룹으로 묶어 사용
// 열거형 멤버에 숫자 또는 문자열을 할당할 수 있음
// 기본 숫자형 열거형
// 열거형은 기본적으로 숫자형으로 구성되며, 첫번째 값은 0부터 시작하고 이후 값들은 1씩 증가
var Direction;
(function (Direction) {
    Direction[Direction["Up"] = 0] = "Up";
    Direction[Direction["Down"] = 1] = "Down";
    Direction[Direction["Left"] = 2] = "Left";
    Direction[Direction["Right"] = 3] = "Right"; // 3으로 초기화됨 
})(Direction || (Direction = {}));
console.log("기본 숫자형 열거형");
console.log(Direction.Up); // 0
console.log(Direction[2]); // 'Left'
// 문자열 열거형은 명시적으로 각 멤버에 문자열 값을할당해야 함
var Colors;
(function (Colors) {
    Colors["Red"] = "RED";
    Colors["Green"] = "GREEN";
    Colors["Blue"] = "BLUE";
})(Colors || (Colors = {}));
console.log("문자열 열거형");
console.log(Colors.Red); // 'RED'
// Status.Active는 컴파일 후 숫자 리터럴 0으로 대체됨
const currentStatus = 0 /* Status2.Active */;
console.log("상수 열거형");
console.log(currentStatus); // 0
console.log(currentStatus === 0 /* Status2.Active */);
// 계산된 멤버를 가진 열거형
var ComputedEnum;
(function (ComputedEnum) {
    ComputedEnum[ComputedEnum["First"] = 10] = "First";
    ComputedEnum[ComputedEnum["Second"] = 20] = "Second";
    ComputedEnum[ComputedEnum["Third"] = 25] = "Third"; // 25
})(ComputedEnum || (ComputedEnum = {}));
console.log("계산된 멤버를 가진 열거형");
console.log(ComputedEnum.Second); // 20
// 이종 열거형 (혼합 열거형)
// 숫자형과 문자열형 값을 혼합해 사용 가능
var MixedEnum;
(function (MixedEnum) {
    MixedEnum[MixedEnum["No"] = 0] = "No";
    MixedEnum["Yes"] = "YES";
})(MixedEnum || (MixedEnum = {}));
console.log("이종 열거형");
console.log(MixedEnum.Yes); // 'YES'
// 상수 열거형을 활용해 사용자 상태 지정
const user1 = {
    name: "Alice",
    status: 1 /* Status2.Inactive */
};
console.log("사용자 상태");
console.log(user1); // { name: 'Alice', status: 1 }
// 열거형 비트 연산
var Permissions2;
(function (Permissions2) {
    Permissions2[Permissions2["None"] = 0] = "None";
    Permissions2[Permissions2["Read"] = 1] = "Read";
    Permissions2[Permissions2["Write"] = 2] = "Write";
    Permissions2[Permissions2["Execute"] = 4] = "Execute";
    Permissions2[Permissions2["All"] = 7] = "All"; // 0111
})(Permissions2 || (Permissions2 = {}));
let permission = Permissions2.Read | Permissions2.Write;
console.log("비트 연산을 사용한 권한 설정");
console.log(permission); // 3 (0011)
// 권한 검사 함수
function hasPermission(current, required) {
    return (current & required) === required;
}
console.log("권한 검사");
console.log(hasPermission(permission, Permissions2.Read)); // true
console.log(hasPermission(permission, Permissions2.Execute)); // false
// 열거형 타입을 사용해 함수의 매개변수를 제한해 잘못된 값이 전달되지 않게 함
function move(direction) {
    console.log(`캐릭터가 ${Direction[direction]} 방향으로 이동합니다.`);
}
console.log("열거형 타입 사용");
move(Direction.Left);
// move("위쪽"); // 오류, '위쪽'은 Direction 타입이 아님
// TS에서 열거형은 기본적으로 런타임에 실제 객체로 존재함
// 다른 객체와 마찬가지로 일부 런타임 오버헤드가 발생할 수 있음
// 상수 열거형(const enum)은 런타임 오버헤드를 줄이기 위해 특정 상황에서 사용 
