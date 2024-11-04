interface User2 {
    name: string;
    age: number;
    greet(): void; // 인사 메서드 정의, 반환값 없음 
}

// 인터페이스 확장해 새로운 인터페이스 정의
interface Employee2 extends User2 {
    employeeId: string; // 직원 아이디를 나타내는 문자열 타입 속성
    department: string; // 부서를 나타내는 문자열 타입 속성
}

// 함수 타입을 가진 인터페이스 정의
interface StringManipulation {
    // 두 문자열을 입력받아 결합하는 함수 정의
    (a: string, b: string): string;
}

// Employee 인터페이스를 구현하는 클래스 정의
class Developer implements Employee2 {
    name: string;
    age: number;
    employeeId: string;
    department: string;

    // 생성자를 통해 필수 프로퍼티들 초기화
    constructor(name: string, age: number, employeeId: string, department: string) {
        this.name = name;
        this.age = age;
        this.employeeId = employeeId;
        this.department = department;
    }

    // greet 메서드 구현 
    greet(): void {
        console.log(`이름은 ${this.name}이고, 나이는 ${this.age}입니다.`);
        console.log(`직원 아이디는 ${this.employeeId}, 부서는 ${this.department}입니다.`);
    }
}

// 타입 앨리어스를 통한 객체 타입 정의
type Product2 = {
    productId: string;
    productName: string;
    price: number;
};

// 인터페이스는 확장(상속) 가능하고, 클래스에서 구현될 수 있음
// 타입 앨리어스는 기존 타입을 재사용하거나 결합하는데 유용
// 유연성 측면에서는 인터페이스가 더 강력하지만, 특정 상황에서는 타입 앨리어스가 더 간결

// 문자열 조작 함수 타입을 사용하는 변수 정의
const concatenateStrings: StringManipulation = (a: string, b: string): string => {
    return `${a} ${b}`;
};

// Developer 클래스의 인스턴스 생성 및 사용
const dev = new Developer('홍길동', 30, 'DEV123', '개발팀');
dev.greet(); // 홍길동 개발자의 정보 출력

// 문자열 조작 함수 호출
console.log(concatenateStrings('타입스크립트', '인터페이스'));

// 타입 앨리어스를 통한 제품 객체 생성
const product: Product2 = {
    productId: 'P12345',
    productName: '노트북',
    price: 1500000,
};

// 제품 정보 출력
console.log(`제품명: ${product.productName}, 가격: ${product.price}원`);

