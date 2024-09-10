// 타입 변환
#include <iostream>
using namespace std;

int main(int argc, char const *argv[]) {
    int a = 10;
    double b = static_cast<double>(a);  // 명시적 변환

    cout << "a (int): " << a << endl;
    cout << "b (double): " << b << endl;

    // static_cast는 명시적 타입 변환 연산자 중 하나
    // 주로 기본 자료형 간의 변환이나 상속 관계에 있는
    // 클래스들 간의 변환을 수행할 때 사용함

    // C++의 다른 타입 변환 연산자들
    // * dynamic_cast
    // 주로 다형성(가상 함수가 있는) 클래스 계층에서 안전하게
    // 다운캐스팅을 수행할 때 사용함
    // 변환이 실패하면 포인터는 nullptr, 참조는 예외를 발생시킴
    // * const_cast
    // const 또는 volatile 속성을 제거하거나 추가할 때 사용함
    // 주로 상수성을 무시하고 수정해야 할 필요가 있을때 사용
    // * reinterpret_cast
    // 포인터나 참조를 전혀 다른 타입으로 변환할 때 사용
    // 일반적으로 포인터 간의 변환에 사용되며
    // 메모리 주소 자체를 변경하지는 않음
    // 메모리 주소를 다른 타입으로 해석할 때
    return 0;
}
