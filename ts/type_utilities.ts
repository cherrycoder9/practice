// Partial 유틸리티 타입
// Partial<T>는 타입 T의 모든 프로퍼티를 선택적으로 만들어주는 유틸리티 타입
interface UserProfile {
    name: string;
    age: number;
    email: string;
}

// user의 일부 프로퍼티만 채우고 싶을 때 Partial을 사용해 모든 프로퍼티가 선택적으로 됨
const updateUser = (user: Partial<UserProfile>): void => {
    console.log(`업데이트된 사용자 정보: ${JSON.stringify(user)}`);
};

updateUser({ name: "홍길동" }); // 선택적으로 name만 업데이트 
updateUser({ email: "hong@example.com" }); // 선택적으로 email만 업데이트

// Readonly 유틸리티 타입
// Readonly<T>는 타입 T의 모든 프로퍼티를 읽기 전용으로 만들어줌
const originalUser: Readonly<UserProfile> = {
    name: "이순신",
    age: 45,
    email: "lee@example.com"
};

// originalUser.age = 50; // 오류, Readonly로 선언된 프로퍼티를 수정 불가
console.log(`사용자 정보 (읽기 전용): ${JSON.stringify(originalUser)}`);


// Pick 유틸리티 타입
// Pick<T, K>는 타입 T에서 K에 해당하는 프로퍼티만 선택해 새로운 타입을 만듦
interface Product3 {
    id: number;
    name: string;
    price: number;
    description: string;
}

// Product3 타입에서 id와 name만 선택해 새로운 타입 정의
const getProductSummary = (product: Pick<Product, "id" | "name">): void => {
    console.log(`상품 요약: ID(${product.id}), 이름(${product.name})`);
};

getProductSummary({ id: 1, name: "노트북" });

// Record 유틸리티 타입
// Record<K, T>는 키 K와 값 T로 구성된 객체 타입을 만들어줌
// 사용자의 역할을 정의하는 객체 생성
type Role = "admin" | "user" | "guest";
interface RoleInfo {
    permissions: string[];
}

const rolePermissions: Record<Role, RoleInfo> = {
    admin: { permissions: ["read", "write", "delete"] },
    user: { permissions: ["read", "write"] },
    guest: { permissions: ["read"] }
};

console.log(`사용자 권한 정보: ${JSON.stringify(rolePermissions)}`);

// Exclude 유틸리티 타입
// Exclude<T, U>는 타입 T에서 U에 해당하는 타입을 제외해 새로운 타입을 만듦

type AvailableColors = "red" | "green" | "blue" | "yellow";
type PrimaryColors = Exclude<AvailableColors, "yellow">; // yellow를 제외한 나머지 색상만 포함

const color: PrimaryColors = "red";
console.log(`기본 색상: ${color}`);

// Omit 유틸리티 타입
// Omit<T, K>는 타입 T에서 프로퍼티 K를 제거해 새로운 타입을 만듦 
interface Car {
    brand: string;
    model: string;
    year: number;
    owner: string;
}

// Car 타입에서 owner 프로퍼티를 제외한 타입 정의
const createCarWithoutOwner = (car: Omit<Car, "owner">): void => {
    console.log(`차량 정보: 브랜드(${car.brand}), 모델(${car.model}), 연식(${car.year})`);
};

createCarWithoutOwner({ brand: "현대", model: "소나타", year: 2022 });

// Mapped Types
// 타입의 모든 프로퍼티를 변경할때 사용, 여기서는 모든 프로퍼티를 선택적으로 만들어줌
interface Address2 {
    street: string;
    city: string;
    postalCode: string;
}

type OptionalAddress = {
    [P in keyof Address2]?: Address2[P];
};

const myAddress2: OptionalAddress = {
    street: "서울특별시",
};

console.log(`선택적 주소 정보: ${JSON.stringify(myAddress2)}`);

// NonNullable 유틸리티 타입
// NonNullable<T>는 타입 T에서 null과 undefined를 제외한 타입을 만들어줌
interface UserContact {
    phoneNumber: string | null;
    email: string | undefined;
}

type ValidContact = NonNullable<UserContact["phoneNumber"]>; // null을 제외한 문자열 타입

const contactInfo: ValidContact = "010-1234-5678";
console.log(`유효한 연락처 정보: ${contactInfo}`);
