// 인터섹션 타입과 Mapped Types 조합을 통해 객체를 유연하게 생성 
interface PersonIMT {
    name: string;
    age: number;
}

interface EmployeeIMT {
    employeeId: number;
    department: string;
}

// Person과 Employee를 결합한 타입 정의
// 두 인터페이스의 모든 속성을 가지게 됨
type PersonEmployee = PersonIMT & EmployeeIMT;

// PersonEmployee 타입을 가진 객체 생성
const employeeIMT: PersonEmployee = {
    name: "홍길동",
    age: 30,
    employeeId: 101,
    department: "개발팀"
};

console.log(`직원 정보: ${JSON.stringify(employeeIMT)}`);

// Mapped Types 사용하면 객체의 모든 속성을 읽기 전용으로 만들 수 있음
// 기존 타입을 변환해 새로운 타입을 생성함
// 아래서는 객체의 모든 속성을 Readonly로 변환함

// 모든 타입 T의 속성을 읽기 전용으로 만드는 유틸리티 타입 정의
// 이 유틸리티 타입은 타입 T의 각 속성 K에 대해 읽기 전용으로 만듦
type ReadonlyIMT<T> = {
    readonly [K in keyof T]: T[K];
};

// PersonEmployee 타입을 읽기 전용으로 변환한 ReadonlyPersonEmployee 타입 정의
// 객체의 속성을 변경하지 못하도록 보호할 수 있음
type ReadonlyPersonEmployee = ReadonlyIMT<PersonEmployee>;

// ReadonlyPersonEmployee 타입의 객체 생성
const readonlyEmployee: ReadonlyPersonEmployee = {
    name: "이순신",
    age: 45,
    employeeId: 102,
    department: "경영팀"
};

console.log(`읽기 전용 직원 정보: ${JSON.stringify(readonlyEmployee)}`);
// readonlyEmployee.name = "이름변경"; // TS에서는 오류 

// Mapped Types를 이용해 모든 속성을 선택적으로 만드는 타입 정의
type PartialIMT<T> = {
    [K in keyof T]?: T[K];
};

// 객체를 생성할 때 필요한 속성만 선택적으로 지정할 수 있음
type PartialPersonEmployee = PartialIMT<PersonEmployee>;

// 객체 생성
const partialEmployee: PartialPersonEmployee = {
    name: "김유신"
};

console.log(`선택적 지원 정보: ${JSON.stringify(partialEmployee)}`);

