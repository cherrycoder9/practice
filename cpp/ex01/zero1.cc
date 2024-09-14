// 함수 오버로딩: 동일한 함수 이름을 사용하되, 매개변수의 타입이나 개수를
// 다르게 정의해 여러 형태의 함수를 구현하는 기능

#include <iostream>
#include <string>

using namespace std;

// 두 정수를 더하는 함수
int add(int a, int b) {
    return a + b;
}

// 두 실수를 더하는 함수
double add(double a, double b) {
    return a + b;
}

// 세 개의 정수를 더하는 함수
int add(int a, int b, int c) {
    return a + b + c;
}

int main() {
    cout << "정수 두 개의 합: " << add(3, 5) << endl;
    cout << "실수 두 개의 합: " << add(2.5, 4.3) << endl;
    cout << "정수 세 개의 합: " << add(1, 3, 5) << endl;

    return 0;
}