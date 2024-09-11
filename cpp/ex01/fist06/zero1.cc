#include <iostream>

using namespace std;

int main() {
    // new 연산자는 힙 메모리에서 동적으로 메모리 할당
    // delete 연산자는 메모리를 해제함
    int* ptr = new int;
    *ptr = 100;
    cout << "동적 할당된 값: " << *ptr << endl;
    delete ptr;

    // 정수형 배열 동적 할당
    int* arr = new int[5];
    // 배열에 값 저장
    for (int i = 0; i < 5; ++i) {
        arr[i] = i * 10;
    }
    // 배열 출력
    for (int i = 0; i < 5; ++i) {
        cout << "arr[" << i << "]: " << arr[i] << endl;
    }
    delete[] arr;

    return 0;
}