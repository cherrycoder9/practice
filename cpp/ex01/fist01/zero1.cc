// 변수 선언
#include <iostream>

using namespace std;

int main(int argc, char const *argv[]) {
    // 정수형 변수 a 선언, 선언시 값 초기화하지 않으면
    // 쓰레기 값을 가질 수 있음.
    int a;
    cout << a << endl;
    int b = 10;  // 정수형 변수 b 선언하면서 10으로 초기화

    char c = 'A';        // 문자형 변수 c 선언 및 초기화
    float d = 3.14f;     // 실수형 변수 d 선언 및 초기화
    double e = 2.71828;  // 배정밀도 실수형 변수 e 선언 및 초기화
    bool f = true;       // 불리언 변수 f 선언 및 초기화

    // 한 줄에서 여러 변수 선언할 수 있음
    int x = 1, y = 2, z = 3;

    // auto 키워드
    // C++11 부터는 auto 키워드를 사용해 컴파일러가
    // 변수의 타입을 자동으로 추론하게 할 수 있음
    auto g = 10;       // int형으로 추론
    auto h = 3.14;     // double형으로 추론
    auto i = "hello";  // const char*형으로 추론

    // 상수 변수 선언
    // 상수 변수는 한번 초기화된 후 값을 변경할 수 없음
    // const 를 사용함
    const int j = 100;  // 정수형 상수 변수 j 선언 및 초기화
    // j = 200; // 오류뜸

    // 참조자 변수
    // 참조자 변수는 다른 변수의 별명을 제공함
    int l = 30;
    int &ref = l;  // l에 대한 참조자 ref 선언
    cout << l << endl;
    cout << ref << endl;

    ref = 40;             // ref를 통해 l의 값을 변경
    cout << l << endl;    // 40
    cout << ref << endl;  // 40

    // 포인터 변수
    // 메모리 주소를 저장하는 변수
    int m = 50;
    int *ptr = &m;  // m의 주소를 저장하는 포인터 ptr 선언
    cout << m << endl;
    cout << *ptr << endl;

    *ptr = 60;  // 포인터를 통해 m의 값을 변경함
    cout << "호호" << endl;
    cout << m << endl;
    cout << *ptr << endl;

    return 0;
}
