#include <iostream>
#include <memory>  // 스마트 포인터 사용을 위해 필요

using namespace std;

void display(shared_ptr<int> sp) {
    cout << "공유된 값: " << *sp << endl;
}

int main() {
    // 메모리 해제를 자동으로 관리해주는 스마트 포인터
    // std::unique_ptr, std::shared_ptr

    // std::unique_ptr
    // 하나의 객체에 대해 하나의 포인터만 소유할 수 있음
    // 자동으로 메모리를 해제해줌

    // unique_ptr을 사용해 동적 할당
    unique_ptr<int> ptr = make_unique<int>(10);
    // 값 출력
    cout << "unique_ptr 값: " << *ptr << endl;
    // 별도로 delete를 호출하지 않아도, 범위를 벗어나면 자동으로 메모리 해제됨
    // 스코프를 벗어나거나 포인터가 소멸할 때 자동으로 delete 호출

    // std::shared_ptr
    // 여러 포인터가 같은 메모리를 공유할 수 있음
    // 마지막 std::shared_ptr이 범위를 벗어날 때 메모리 해제됨
    // 참조 카운팅을 통해 메모리 관리, 참조 카운트가 0이 되면 메모리 해제

    // shared_ptr로 동적 할당
    shared_ptr<int> sp1 = make_shared<int>(20);
    shared_ptr<int> sp2 = sp1;  // sp1과 같은 메모리 공유

    display(sp1);
    display(sp1);

    // sp1, sp2가 모두 범위를 벗어나면 메모리 자동 해제
    return 0;
}