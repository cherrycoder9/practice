function add(a: number, b: number): number {
    return a + b;
}

console.log(`add 함수 결과: ${add(5, 3)}`);

// 선택적 매개변수와 기본값 
function greet(name: string, greeting: string = "안녕하세요"): string {
    // greeting 매개변수는 선택적으로 입력할 수 있음, 미입력시 기본값 사용
    return `${greeting}, ${name}!`;
}

console.log(`${greet("제로디")}`);
console.log(`${greet("제로디", "반갑습니다")}`);

// 화살표 함수와 함수 타입 정의
const multiply = (x: number, y: number): number => x * y;
console.log(multiply(4, 5));

// 함수 타입을 변수로 저장해 함수 정의에 사용 
let calculate: (x: number, y: number, operation: string) => number;

calculate = (x, y, operation) => {
    switch (operation) {
        case "add":
            return x + y;
        case "subtract":
            return x - y;
        case "multiply":
            return x * y;
        case "divide":
            return y !== 0 ? x / y : 0; // 0으로 나누기 방지
        default:
            throw new Error("지원하지 않는 연산입니다.");
    }
};

console.log(calculate(10, 5, "add"));
console.log(calculate(10, 0, "divide"));

// 제네릭을 사용하는 함수 타입 정의
function getArrayElement<T>(arr: T[], index: number): T {
    // 제네릭 T를 사용해 여러 타입의 배열을 처리할 수 있는 함수 
    return arr[index];
}

console.log(getArrayElement(["a", "b", "c"], 1));
console.log(getArrayElement([1, 2, 3], 2));

// 콜백 함수 타입
// 콜백 함수는 함수의 매개변수로 전달되는 함수
function processString(input: string, callback: (str: string) => string): string {
    // 매개변수로 전달받는 callback은 문자열을 받아 문자열을 반환하는 함수 타입 
    return callback(input);
}

const toUpperCase = (str: string): string => str.toUpperCase(); // 콜백으로 사용할 함수
console.log(processString("typescript", toUpperCase));

// 콜백과 제네릭 사용 함수
function transformArray<T, U>(arr: T[], transformer: (value: T) => U): U[] {
    // 배열의 각 요소를 변환하는 함수, 제네릭 T와 U를 사용해 입력과 출력 타입을 다르게 지정 가능함
    return arr.map(transformer);
}

const numbers2 = [1, 2, 3, 4];
const doubledNumbers = transformArray(numbers2, (num) => num * 2);
console.log(doubledNumbers);

