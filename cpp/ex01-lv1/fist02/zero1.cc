// 자료형
#include <iostream>

using namespace std;

int main(int argc, char const* argv[]) {
    // 정수형 자료형
    int a = 10;      // 4바이트 정수
    short b = 20;    // 2바이트 정수
    long c = 30000;  // 4 또는 8바이트 정수 (플랫폼에 따라 다름)
    long long d = 90000;  // 8바이트 정수

    // 부호 없는 정수형 자료형
    unsigned int e = 50;    // 0 이상만 표현 가능한 4바이트 정수
    unsigned short f = 60;  // 0 이상만 표현 가능한 2바이트 정수

    // 실수형 자료형
    float g = 3.14f;     // 4바이트 실수 (정밀도 낮음)
    double h = 3.14159;  // 8바이트 실수 (정밀도 높음)
    long double i = 3.141592653589793238L;  // 더 큰 정밀도의 실수

    // 문자형 자료형
    char j = 'A';       // 1바이트 문자
    wchar_t k = L'가';  // 2바이트 또는 4바이트 (플랫폼마다 다름)
    // cout << k << endl; // 이렇게 출력하면 오류남
    // wide character stream을 사용하여 출력
    wcout << k << endl;

    // 논리형 자료형
    bool l = false;     // 1바이트 논리형 true / false
    cout << l << endl;  // 0

    return 0;
}