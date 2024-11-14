// 데코레이터는 클래스, 메서드, 접근자, 프로퍼티, 매개변수 등에 추가 기능을 부여하기 위해 사용됨
// TS의 데코레이터는 함수로 구현됨
// 클래스 생성시 로그를 출력하고, 클래스를 수정할 수 있도록 하는 데코레이터 작성

function LogClass(target: Function) {
    // 클래스가 생성될 때마다 호출됨
    console.log(`클래스 ${target.name}가 생성되었습니다.`);
}

// @LogClass
// 클래스 데코레이터를 사용하기 위해 @ 데코레이터 이름을 클래스 정의 위에 붙임
// 이 데코레이터는 클래스 생성시 추가적인 동작 부여 
@LogClass
class Person {
    name: string;
    age: number;

    constructor(name: string, age: number) {
        this.name = name;
        this.age = age;
    }

    greet() {
        console.log(`이름은 ${this.name}, 나이는 ${this.age}`);
    }
}

const personCD = new Person('민수', 30);
personCD.greet();


// 메서드 데코레이터
// 메서드 호출 시간을 측정해 성능을 추적하는 기능을 추가하는 데코레이터
function MeasureExecutionTime(
    target: Object,
    propertyKey: string,
    descriptor: PropertyDescriptor
): PropertyDescriptor {
    if (!descriptor || typeof descriptor.value !== 'function') {
        throw new TypeError(`Descriptor가 유효한 메서드를 참조하지 않았습니다.`);
    }

    const originalMethod = descriptor.value;

    descriptor.value = function (...args: any[]) {
        const start = performance.now();
        const result = originalMethod.apply(this, args);
        const end = performance.now();
        console.log(`${propertyKey} 메서드 실행 시간: ${(end - start).toFixed(2)}ms`);
        return result;
    };

    return descriptor;
}

class Calculator {
    // tsconfig.json 설정에서
    // "experimentalDecorators": true
    // "emitDecoratorMetadata": true
    // 적용해야 함 
    @MeasureExecutionTime
    fibonacci(n: number): number {
        if (n <= 1) return n;
        return this.fibonacci(n - 1) + this.fibonacci(n - 2);
    }
}

const calc = new Calculator();
calc.fibonacci(10);


// 프로퍼티 데코레이터
// 특정 프로퍼티에 대한 접근 권한을 제어하는 기능을 추가하는 데코레이터
// 클래스의 프로퍼티에 대한 메타데이터를 설정하거나 제어하기 위해 사용
// 특정 프로퍼티에 접글할 때 콘솔에 로그를 출력하는 기능을 추가하는 예제
function LogProperty(target: Object, propertyKey: string): void {
    // 인스턴스별 값을 저장
    const valueMap = new WeakMap<any, any>();

    // 프로퍼티 디스크립터 정의
    Object.defineProperty(target, propertyKey, {
        get: function (this: any): any {
            const value = valueMap.get(this);
            console.log(`프로퍼티 ${propertyKey}에 접근했습니다. 값: ${value}`);
            return value;
        },

        set: function (this: any, newValue: any): void {
            const currentValue = valueMap.get(this);
            if (currentValue === undefined) {
                console.log(`프로퍼티 ${propertyKey} 초기값이 ${newValue}로 설정되었습니다.`);

            } else {
                console.log(`프로퍼티 ${propertyKey} 값이 ${currentValue}에서 ${newValue}로 변경되었습니다.`);
            }
            valueMap.set(this, newValue);
        },
        enumerable: true,
        configurable: true,
    });
}

class UserCD {
    @LogProperty
    public username!: string; // 초기화 체크 비활성화

    constructor(username: string) {
        this.username = username;
    }
}

const userCD = new UserCD("영희");
console.log(`현재 username: ${userCD.username}`);
userCD.username = "철수";
console.log(`변경된 username: ${userCD.username}`);

