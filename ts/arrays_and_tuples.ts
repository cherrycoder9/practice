// 타입스크립트에선 배열의 타입을 정의할 때 요소 타입 뒤에 대괄호를 붙여 정의
let numbers: number[] = [1, 2, 3, 4, 5]; // 숫자형 배열 정의
let strings: string[] = ["하나", "둘", "셋"]; // 문자열 배열 정의

// 배열의 타입 제한과 다차원 배열
// 요소의 타입이 일관되도록 배열을 제한함으로써, 타입 안정성을 높임
let booleanMatrix: boolean[][] = [
    [true, false, true],
    [false, true, false],
];
// 다차원 배열에서 각 요소는 동일한 타입이어야 함

// 튜플의 고정 길이와 타입 순서
// 튜플은 배열과 유사하지만, 고정된 길이와 각 위치에 따른 타입을 정의할 수 있음
let person: [string, number] = ["홍길동", 30];
// 첫 번째 요소는 문자열, 두 번째 요소는 숫자로 고정된 타입

// 튜플의 요소 순서를 바꾸면 타입 오류 발생
// let incorrectPerson: [string, number] = [30, "홍길동"];

// 튜플의 특정 요소는 유연하게 선택적으로 정의할 수 있음 
let optionalTuple: [string, number?] = ["선택적 요소"];
// 두 번째 요소는 있어도 되고 없어도 됨

// 배열은 동일한 타입의 여러 요소를 가질 수 있어 반복 작업에 유리함 
let fruits: string[] = ["사과", "바나나", "오렌지"];
fruits.push("포도"); // 배열은 길이가 동적으로 변할 수 있음

// 튜플은 특정한 구조와 순서를 보장해야 하는 경우 적합
let myAddress: [string, number, string] = ["서울특별시", 12345, "대한민국"];

// 특정 데이터 구조를 나타낼 때 배열과 튜플을 혼합해 사용할 수 있음 
let employees: [string, number][] = [
    ["김철수", 35],
    ["이영희", 28],
];
// 각 직원의 이름과 나이를 튜플로 표현하고, 배열로 관리함

// 직원 목록에서 나이를 기준으로 특정 나이 이상의 직원 이름을 반환하는 함수
function getEmployeesAboveAge(employees: [string, number][], age: number): string[] {
    return employees.filter((employee) => employee[1] >= age).map((employee) => employee[0]);
}

console.log(getEmployeesAboveAge(employees, 30)); // ['김철수']
