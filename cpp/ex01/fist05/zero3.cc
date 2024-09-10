#include <iostream>

using namespace std;

int main() {
    int x = 5;
    int y = 10;

    // 상수 포인터 (가리키는 값은 수정 가능,
    // 포인터 자체는 다른 곳을 가리킬 수 없음)
    int* const ptr = &x;
    *ptr = 6;  // 가능 (가리키는 값 수정)
    // ptr = &y; // 불가능 (포인터가 다른 주소를 가리킬 수 없음)

    // 상수를 가리키는 포인터 (가리키는 값은 수정 불가,
    // 포인터 자체는 다른 곳을 가리킬 수 있음)
    const int* ptr2 = &x;
    // *ptr2 = 7; // 불가능 (가리키는 값 수정 불가)
    ptr2 = &y;

    // 상수 참조 (값 수정 불가)
    const int& ref = x;
    // ref = 8; // 불가능 (참조를 통해 값 변경 불가)

    cout << "x: " << x << endl;

    return 0;
}