// 함수 템플릿
// 여러 타입에 대해 동작하는 일반화된 함수를 정의할 수 있도록 하는 기능
// 함수의 인자 타입을 구체화하지 않고, 컴파일러가 호출 시점에 타입을 결정
// 다양한 타입의 데이터에 대해 같은 논리의 함수를 적용

#include <iostream>

using namespace std;

// T는 템플릿 매개변수, 함수를 호출할 때 컴파일러에 의해 실제 타입으로 대체됨
template <typename T>
// T 타입의 매개변수 a와 b를 받아들이며, 동일한 타입의 결과를 반환
T add(T a, T b) {
    return a + b;
}

// 템플릿 특수화
template <>
const char* add(const char* a, const char* b) {
    // 두 문자열을 연결하는 코드
}

int main(int argc, char const* argv[]) {
    int result1 = add(3, 4);         // T는 int로 대체됨
    double result2 = add(3.5, 4.5);  // T는 double로 대체됨
    cout << result1 << endl << result2 << endl;
    return 0;
}
