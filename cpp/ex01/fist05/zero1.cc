#include <iostream>
using namespace std;
// 포인터는 메모리 주소를 저장하는 변수
// 포인터는 값이 아니라 값이 저장된 메모리 주소를 가리키고
// 이것을 통해 해당 값을 간접적 접근하거나 변경할 수 있음

int main() {
    int a = 10;
    int* ptr = &a;  // a의 주소를 ptr에 저장 (포인터 선언)

    cout << "a의 값: " << a << endl;
    cout << "ptr이 가리키는 값: " << *ptr
         << endl;  // 포인터를 역참조해 a값 출력

    *ptr = 20;  // 포인터를 통해 a값 변경
    cout << "포인터를 통해 변경된 a의 값: " << a << endl;

    int b = 30;
    int& ref = b;  // b에 대한 참조 ref 선언

    cout << "b의 값: " << b << endl;
    cout << "ref의 값: " << ref << endl;  // ref를 통해 b 값 출력

    ref = 40;  // 참조를 통해 b의 값 변경
    cout << "참조를 통해 변경된 b의 값: " << b << endl;

    // ! 참조는 포인터와 달리 null 상태가 될 수 없음
    // ! 값이 없는 상태로 초기화할 수도 없음. 항상 유효한 변수를 참조해야 함

    // # 포인터와 참조의 차이
    // # 포인터는 값을 가리키는 메모리 주소를 저장
    // # 참조는 변수를 대체하는 별칭
    // # 포인터는 선언 후 다른 메모리 주소를 가리킬 수 있음
    // # 참조는 한번 선언후 다른 변수로 변경 불가

    return 0;
}