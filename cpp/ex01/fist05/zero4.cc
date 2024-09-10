#include <iostream>

using namespace std;

// # 포인터 배열과 다중 포인터
// 포인터는 배열처럼 여러 개의 변수를 가리킬 수 있음
// 포인터 자신이 다른 포인터를 가리키는 다중 포인터 가능

int main() {
    int arr[3] = {1, 2, 3};  // 정수 배열 선언
    int* ptr = arr;  // 배열의 첫 번째 원소를 가리키는 포인터

    for (int i = 0; i < 3; ++i) {
        cout << "arr[" << i << "]: " << *(ptr + i) << endl;
    }

    int x = 5;
    int* p1 = &x;    // p1은 x를 가리킴
    int** p2 = &p1;  // p2는 p1을 가리킴

    cout << "p1이 가리키는 값: " << *p1 << endl;  // x 값 출력
    cout << "p2가 가리키는 포인터가 가리키는 값: " << **p2 << endl;  // x값 출력

    return 0;
}