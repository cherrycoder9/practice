// 타입 앨리어스를 사용해 복잡한 타입을 간단히 표현할 수 있음
// 특정 타입에 이름을 부여해 재사용성을 높임
// 클래스와는 다름. type 키워드는 데이터의 구조를 정의하는데만 사용
// 객체를 생성하거나 실행시 동작을 포함하지 않음
// C 구조체는 런타임에 메모리를 할당하고 사용할 수 있는 실제 데이터 타입인 반면
// 타입스크립트의 type은 컴파일 시점에만 존재함  
type UserInfo = {
    name: string;
    email: string;
    age: number;
    phoneNumber?: string;
};

// 여러 타입을 포함하는 유니언 타입의 앨리어스 정의
type Status = 'active' | 'inactive' | 'suspended';

// 기본 설정 값 타입 정의
type DefaultSetting = string | number;

// 함수 타입 앨리어스 정의
type CalculateAverage = (numbers: number[]) => number;

// 복잡한 제네릭 타입을 사용한 앨리어스 정의
type Response2<T> = {
    status: Status;
    data: T;
    error?: string;
};

// 유틸리티 타입을 활용해 읽기 전용 타입 정의
// UserInfo 타입의 모든 프로퍼티를 읽기 전용으로 만듦
type ReadonlyUserInfo = Readonly<UserInfo>;

// 타입 앨리어스를 이용해 주어진 정보를 포함한 사용자 목록을 출력하는 함수
function printUserList(users: UserInfo[]): void {
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
const calculateAverage: CalculateAverage = (numbers) => {
    const sum = numbers.reduce((acc, val) => acc + val, 0); // 배열의 모든 요소의 합을 계산
    return numbers.length > 0 ? sum / numbers.length : 0; // 평균 값을 반환 
};

// 제네릭 타입을 사용한 API 응답 생성 함수 
function createApiResponse<T>(data: T, status: Status): Response2<T> {
    return {
        status,
        data,
        error: status === 'suspended' ? '사용자 접근 제한' : undefined,
    };
}

// 실행
const users: UserInfo[] = [
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
const readonlyUser: ReadonlyUserInfo = {
    name: "성춘향",
    email: "sung@x.com",
    age: 24,
    phoneNumber: "010-1234-5678",
};

// readonlyUser.age = 25; // 읽기 전용 속성이므로 수정 불가 
// console.log(`읽기 전용 사용자 정보: ${readonlyUser}`);
console.log(`읽기 전용 사용자 정보: ${JSON.stringify(readonlyUser, null, 2)}`);


