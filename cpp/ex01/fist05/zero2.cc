#include <iostream>

using namespace std;

void modifyByPointer(int* ptr) {
    *ptr = 100;  // 포인터를 통해 값 수정
}

void modifyByReference(int& ref) {
    ref = 200;  // 참조를 통해 값 수정
}

int main() {
    int num = 50;

    // 포인터를 이용한 값 수정
    // 호출 시 변수의 주소를 전달해야 함
    modifyByPointer(&num);
    cout << "포인터를 통해 수정된 num의 값: " << num << endl;

    // 참조를 이용한 값 수정
    // 호출 시 변수 자체를 전달하면 됨
    modifyByReference(num);
    cout << "참조를 통해 수정된 num의 값: " << num << endl;

    return 0;
}