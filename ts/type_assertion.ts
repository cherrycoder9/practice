import { JSDOM } from 'jsdom';

// jsdom을 사용해 브라우저처럼 document와 window 객체를 생성
const { document } = (new JSDOM(`<!DOCTYPE html><body></body>`)).window;

// 타입 단언은 개발작 특정 값의 타입을 컴파일러에게 보증할 때 사용
// 다른 언어의 강제 형변환과 유사하지만, 런타임에 실제 변환이 일어나는게 아니라 컴파일러를 속이는 형태

function getStringLength(value: unknown): number {
    // unknown 타입은 모든 타입을 받을 수 있는 슈퍼타입
    // value가 문자열이란 것을 컴파일러에 알려줌
    const str = value as string;
    console.log(`입력된 문자열 길이: ${str.length}`);
    return str.length;
}

getStringLength(`Hello, TS!`);

// angle-bracket 문법을 이용한 타입 단언
// TS에서는 as 키워드 외에 angle-bracket 문법도 사용 가능하지만,
// React와의 혼동을 피하기 위해 주로 as를 권장함
function isHTMLInputElement(element: HTMLElement): boolean {
    // HTMLElement를 HTMLInputElement로 단언한 후, tagName을 이용해 input인지 확인 
    const inputElement = <HTMLInputElement>element;
    if (inputElement.tagName === 'INPUT') {
        console.log(`HTMLInputElement 이름 속성: ${inputElement.name}`);
        return true;
    }
    return false;
}

const testElement = document.createElement('input');
testElement.name = "exampleInput";
console.log(isHTMLInputElement(testElement)); // true

const divElement = document.createElement('div');
console.log(isHTMLInputElement(divElement)); // false

// 잘못된 타입 단언
// 타입 단언은 타입이 보장된다고 컴파일러에게 알려주는 역할을 하기 때문에
// 개발자가 잘못 사용할 경우 런타임 에러 발생 가능
function wrongTypeAssertion(value: number): string {
    // 숫자를 문자열로 단언함 (실제 변환은 일어나지 않으므로 주의)
    const strValue = value as unknown as string;
    console.log(`잘못된 타입 단언 후 값: ${strValue}`);
    return strValue; // 숫자를 문자열로 취급하므로 이후의 작업에서 에러 발생 가능성
}

wrongTypeAssertion(123);

// 타입 가드를 사용한 안전한 타입 단언 
function getElementValue(element: HTMLElement | null): string {
    if (element && element.tagName === 'INPUT') {
        // tagName을 사용해 input 요소인지 확인
        const value = (element as HTMLInputElement).value;
        console.log(`안전하게 타입 단언 후 값: ${value}`);
        return value;
    }
    return "요소가 없거나, 잘못된 타입입니다.";
}

const inputElement = document.createElement('input');
inputElement.value = "테스트 값";
getElementValue(inputElement);

// strong 타입 단언
function getInputElementValueById(id: string): string | null {
    const element = document.getElementById(id);
    if (element && element.tagName === 'INPUT') {
        const inputElement = element as HTMLInputElement;
        console.log(`id가 ${id}인 요소의 값: ${inputElement.value}`);
        return inputElement.value;
    }
    return null;
}

const input = document.createElement('input');
input.id = "myInput";
input.value = "example value";
document.body.appendChild(input);
console.log(getInputElementValueById("myInput"));

// 제네릭과 함께 사용하는 타입 단언
function convertToArray<T>(value: T): T[] {
    // 단일 값을 배열로 변환해 반환
    // 배열 리터럴을 사용해 새 배열을 생성하는 문법
    const array = [value] as T[];
    console.log(`단언된 배열: ${array}`);
    return array;
}

convertToArray<number>(5);
convertToArray<number[]>([5, 7, 8]);
convertToArray<string>("Hello");


// 타입 캐스팅과 타입 단언 차이점
// JS에서 런타임시 발생하는 실제 타입 캐스팅과 다르게
// TS의 타입 단언은 컴파일러 수준에서의 타입 확인에 불과함

// 유틸리티 타입과 타입 단언 
interface UserProfile {
    name: string;
    age?: number;
}

function printUserProfile(profile: Partial<UserProfile>): void {
    // Partial<UserProfile> 타입을 사용하면 모든 속성이 선택적임 
    console.log(`사용자 이름: ${(profile as UserProfile).name}`);
    if (profile.age) {
        console.log(`사용자 나이: ${(profile as UserProfile).age}`);
    }
    // console.log(`사용자 나이: ${(profile as UserProfile).age}`);

}

printUserProfile({ name: "철수" });
printUserProfile({ name: "영희", age: 25 });
