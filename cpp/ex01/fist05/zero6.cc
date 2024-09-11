// 다중 포인터를 이용한 값 교환
// 두 개의 정수 변수를 입력받고, 이 변수를 다중 포인터를
// 이용해 서로 값을 교환하는 프로그램 작성

#include <iostream>

using namespace std;

// 다중 포인터를 이용해 서로의 값을 교환하는 함수
void swapWithDoublePointer(int** p1, int** p2) {
    // 포인터가 가리키는 주소의 값을 변경하는게 아니라, 포인터 자체를 교환
    int* temp = *p1;
    *p1 = *p2;
    *p2 = temp;
}

int main() {
    int a = 5;
    int b = 10;

    int* ptr1 = &a;
    int* ptr2 = &b;

    cout << "교환 전 값" << endl;
    cout << "a: " << a << endl;
    cout << "b: " << b << endl;

    swapWithDoublePointer(&ptr1, &ptr2);

    cout << "교환 후 값" << endl;
    cout << "a: " << *ptr1 << endl;
    cout << "b: " << *ptr2 << endl;
}