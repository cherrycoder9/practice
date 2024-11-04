// 객체 타입
type User = {
    name: string; // 사용자의 이름, 문자열 타입
    age: number; // 사용자의 나이, 숫자 타입
    email?: string; // 이메일 주소, 선택적 속성
};

// 중첩 객체 타입 정의
type Address = {
    city: string; // 도시 이름, 문자열 타입
    zipCode: string; // 우편번호, 문자열 타입
};

type Employee = {
    userInfo: User; // 사용자 정보, User 타입의 중첩 객체
    position: string; // 직급, 문자열 타입
    address: Address; // 주소 정보, Address 타입의 중첩 객체
};

// 객체 타입과 인터페이스 비교
// 둘다 객체 구조를 정의하는데 사용할 수 있음
// 그러나 인터페이스는 주로 클래스와의 호환성을 위해 사용되며 확장성이 더 좋음
// 인터페이스를 사용하면 상속을 통한 확장이 더 자연스러움

interface Product {
    id: number; // 제품 ID, 숫자 타입
    name: string; // 제품 이름, 문자열 타입
    description?: string; // 선택적 프로퍼티, 제품 설명 
}

interface EletronicProduct extends Product {
    power: string; // 전력 정보, 문자열 타입
}

// 선택적 프로퍼티를 가진 객체
const product1: Product = {
    id: 101,
    name: "노트북",
}; // description은 선택적이므로 생략 가능

const product2: Product = {
    id: 102,
    name: "스마트폰",
    description: "최신 모델 스마트폰"
}; // 선택적 프로퍼티 포함

// 객체 구조 분해는 객체에서 특정 속성을 추출해 변수에 할당하는 방법
// 타입스크립트에서는 구조 분해와 함께 변수의 타입도 명시할 수 있음
const employee: Employee = {
    userInfo: {
        name: "김철수",
        age: 30,
    },
    position: "개발자",
    address: {
        city: "서울",
        zipCode: "12345",
    },
};

// 객체 구조 분해를 통해 Employee 객체의 특정 속성 추출
const { userInfo, address }: { userInfo: User; address: Address; } = employee;

console.log(`사용자 이름: ${userInfo.name}`);
console.log(`사용자 나이: ${userInfo.age}`);
console.log(`도시: ${address.city}`);
console.log(`우편번호: ${address.zipCode}`);

// 함수의 매개변수에 객체 타입을 정의해 객체 구조를 명시적으로 요구할 수 있음
function printEmployeeInfo(employee: Employee): void {
    const { userInfo, position, address } = employee;
    console.log(`직원 이름: ${userInfo.name}`);
    console.log(`직원 직급: ${position}`);
    console.log(`도시: ${address.city}, 우편번호: ${address.zipCode}`);
}

printEmployeeInfo(employee);