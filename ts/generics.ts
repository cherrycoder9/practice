// 제네릭 타입을 사용하는 클래스를 정의해 다양한 타입의 Stack 자료구조를 만듦
class Stack<T> {
    private items: T[] = []; // T 타입의 배열 사용, Stack 요소들 저장

    // push 메서드, Stack에 요소 추가
    push(item: T): void {
        this.items.push(item);
        console.log(`${item} 스택에 추가됨`);
    }

    // pop 메서드, Stack에서 마지막 요소를 제거하고 반환
    pop(): T | undefined {
        if (this.items.length === 0) {
            console.log(`스택이 비어 있음`);
            return undefined;
        }
        const removedItem = this.items.pop();
        console.log(`${removedItem} 스택에서 제거됨`);
        return removedItem;
    }

    // peek 메서드, Stack의 마지막 요소 확인 
    peek(): T | undefined {
        if (this.items.length === 0) {
            console.log(`스택이 비어 있음`);
            return undefined;
        }
        return this.items[this.items.length - 1];
    }

    // isEmpty 메서드, Stack이 비어 있는지 확인
    isEmpty(): boolean {
        return this.items.length === 0;
    }
}

// 제네릭을 사용해 다양한 타입의 키-값 쌍을 처리할 수 있는 인터페이스 정의 
interface KeyValuePair<K, V> {
    key: K;
    value: V;
}

// 제네릭 함수 정의
// 두 개의 매개변수를 받아 KeyValuePair 객체를 생성하는 함수
function createKeyValuePair<K, V>(key: K, value: V): KeyValuePair<K, V> {
    return { key, value }; // 주어진 키와 값을 가진 객체를 반환 
}

// 제네릭 함수 사용
const numberStringPair = createKeyValuePair<number, string>(1, "하나");
console.log(numberStringPair); // 출력: { key: 1, value: '하나' }

// 제네릭 클래스 사용
const numberStack = new Stack<number>();
numberStack.push(10);
numberStack.push(20);
numberStack.pop();

// 제네릭 제약 조건
// 특정 속성을 가진 타입들만 사용할 수 있도록 제한할 수 있음
interface Lengthwise {
    length: number; // length 속성을 가져야 함
}

// 제네릭 제약 조건이 있는 함수 정의
function logWithLength<T extends Lengthwise>(item: T): void {
    console.log(`길이: ${item.length}`);
}

logWithLength(`안녕하세요`); // 출력: 길이: 5
logWithLength([1, 2, 3, 4]); // 출력: 길이: 4
// logWithLength(10); // 오류, number 타입은 length 속성을 가지고 있지 않음

// 데이터 저장소를 표현하는 인터페이스 정의
interface Repository<T> {
    add(item: T): void; // 요소 추가 메서드
    remove(item: T): void; // 요소 제거 메서드
    getAll(): T[]; // 모든 요소 가져오기 메서드 
}

// 제네릭을 사용해 다양한 타입을 지원하는 데이터 저장소 클래스 정의
class MemoryRepository<T> implements Repository<T> {
    private storage: T[] = []; // 저장을 위한 배열

    // 요소를 저장소에 추가하는 메서드
    add(item: T): void {
        this.storage.push(item);
        console.log(`${item} 저장소에 추가됨`);
    }

    // 요소를 저장소에서 제거하는 메서드
    remove(item: T): void {
        const index = this.storage.indexOf(item);
        if (index > -1) {
            this.storage.splice(index, 1);
            console.log(`${item} 저장소에서 제거됨`);
        } else {
            console.log(`${item} 저장소에 존재하지 않음`);
        }
    }

    // 저장소의 모든 요소를 반환하는 메서드
    getAll(): T[] {
        return [...this.storage];
    }
}

// 메모리 저장소 클래스 사용
const stringRepository = new MemoryRepository<string>();
stringRepository.add("타입스크립트");
stringRepository.add("제네릭");
stringRepository.remove("타입스크립트");
console.log(stringRepository.getAll());
