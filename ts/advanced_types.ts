// 조건부 타입
// 타입이 특정 조건에 따라 달라지게 하여 유연성을 극대화하는 타입 시스템 기능

// 'T'가 배열 타입인지 확인해 배열인 경우 배열의 요소 타입을 반환하고
// 배열이 아닌 경우 그대로 'T' 타입을 반환함
// 배열인지 확인하기 위해 조건부 타입을 사용함
// 조건부 타입 안에서 infer 키워드를 사용해 타입을 추론할 수 있음
type ElementType<T> = T extends (infer U)[] ? U : T;

// number[]를 입력으로 받으면 number 타입 반환
type NumberElement = ElementType<number[]>; // 결과적으로 number 타입이 됨

// string을 입력으로 받으면 string 타입을 그대로 반환
type StringElement = ElementType<string>; // 결과적으로 string 타입이 됨

// 조건부 타입을 이용해 객체 타입을 특정 조건에 따라 변경하는 예제
interface UserAT {
    name: string;
    age: number;
    isAdmin?: boolean; // 선택적 속성
}

// T가 isAdmin 속성을 포함하면 UserAT를 반환하고, 포함하지 않으면 UserAT & {isAdmin:boolean} 반환
type EnsureAdmin<T> = T extends { isAdmin: boolean; } ? T : T & { isAdmin: boolean; };

// 기본 UserAT 인터페이스는 isAdmin을 포함하지 않을 수 있으므로 EnsureAdmin 타입을 사용해 이를 보장함
type AdminUser = EnsureAdmin<UserAT>; // 결과적으로 User & {isAdmin:boolean} 타입이 됨

// 함수의 반환 타입을 추론하는 타입 정의
// T가 함수인지 확인하고, 함수라면 반환 타입을 추론함
// 그렇지 않다면 never 타입을 반환함
type ReturnTypeInfer<T> = T extends (...args: any[]) => infer R ? R : never;

const addART = (a: number, b: number) => {
    return a + b;
};

// addART 함수의 반환 타입 추론
type AddReturnType = ReturnTypeInfer<typeof addART>; // 결과적으로 number 타입이 됨

// Promise 내부 타입을 추론해 반환하는 타입 정의
type UnpackPromise<T> = T extends Promise<infer U> ? U : T;

const fetchData = async (): Promise<string> => {
    return "데이터를 가져왔습니다.";
};

// fetchData 함수의 반환 타입인 Promise 내부 타입 추론
type FetchDataType = UnpackPromise<ReturnType<typeof fetchData>>; // string 타입이 됨

// 객체의 모든 속성을 선택적으로 만드는 유틸리티 타입 정의
type MakeOptional<T> = {
    [K in keyof T]?: T[K];
};

// 모든 속성을 필수적으로 만드는 유틸리티 타입 정의
type MakeRequired<T> = {
    // -? 기호는 선택적 속성 또는 읽기 전용 속성의 플래그를 제거하는 역할 
    [K in keyof T]-?: T[K];
};

// UserAT 타입을 선택적으로 만듦
type OptionalUser = MakeOptional<UserAT>;

// UserAT 타입을 필수적으로 만듦 
type RequiredUser = MakeRequired<UserAT>;

// 객체의 특정 속성만 수정 가능하게 해 불변성을 유지하도록 설계
type Mutable<T, K extends keyof T> = {
    -readonly [P in K]: T[P]; // 지정된 키는 수정 가능하게 만듦
} & {
    readonly [P in Exclude<keyof T, K>]: T[P]; // 나머지 속성은 읽기 전용으로 유지 
};

// 예시 객체 정의
interface ReadonlyUser {
    readonly name: string;
    readonly age: number;
    readonly isAdmin: boolean;
}

// 이름만 수정 가능하고 나머지 속성은 수정 불가능한 타입 정의
type MutableNameUser = Mutable<ReadonlyUser, "name">;

const userMNU: MutableNameUser = {
    name: "홍길동",
    age: 30,
    isAdmin: true,
};

userMNU.name = "이몽룡"; // 가능함
// userMNU.age = 35; // 오류 발생, age는 읽기 전용 

console.log(userMNU);
