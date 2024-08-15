// 기본 자료형 합계 구하기
// 세 가지 기본 자료형 int, float, double 변수 선언
// 각각의 값에 대해 합계를 구해 출력

#include <iostream>
using namespace std;

int main(int argc, char const *argv[]) {
    int a = 10;
    float b = 20.5f;
    double c = 30.5;

    double sum = 0;

    sum = static_cast<double>(a) + static_cast<double>(b) + c;
    cout << sum << endl;

    return 0;
}
