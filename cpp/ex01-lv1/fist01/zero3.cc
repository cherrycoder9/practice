// 상수와 변수 사용
// 정수형 상수 MAX 선언하고 100을 할당
// 그 다음 정수형 변수 value를 선언하고 50으로 초기화
// 그다음 value 값에 MAX를 더한 결과를 다시 value에 저장
// 최종 값을 출력

#include <iostream>

using namespace std;

int main(int argc, char const *argv[]) {
    const int MAX = 100;  // 상수 선언시 const 사용
    int value = 50;
    value += MAX;

    cout << value << endl;

    return 0;
}
