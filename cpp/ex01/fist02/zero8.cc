// 열거형과 조건문 사용하기
// 요일을 나타내는 열거형 Weekday 정의
// 특정 요일이 주말인지 평일인지 출력

#include <iostream>
using namespace std;

enum Weekday { Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday };

int main(int argc, char const *argv[]) {
    Weekday weekday = Saturday;

    if (weekday >= 5) {
        cout << "주말입니다 !!!" << endl;
    } else {
        cout << "평일입니다" << endl;
    };

    return 0;
}
