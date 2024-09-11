// 정수 배열을 동적으로 생성하고 입력받은 크기와 값에 따라
// 배열을 초기화한 후, 역순으로 출력하는 프로그램 작성
// 사용이 끝나면 반드시 메모리 해제할 것
// 포인터 연산을 사용해 배열을 순회하고 역순 출력할 것

#include <iostream>

using namespace std;

void reversePrintArray(int* arr, int size) {
    // 배열을 역순으로 출력하는 함수
    // arr: 배열의 시작을 가리키는 포인터
    // size: 배열의 크기

    cout << "배열을 역순으로 출력: " << endl;

    for (int i = size - 1; i >= 0; --i) {
        cout << arr[i] << " ";  // 현재 인덱스의 값을 출력
    }

    cout << endl;
}

int main() {
    int size;  // 배열 크기를 저장할 변수 선언

    cout << "배열의 크기를 입력하세요: ";
    cin >> size;  // 자용자로부터 배열 크기 입력 받음

    // new 연산자를 사용해 입력받은 크기만큼 동적 배열 할당
    int* arr =
        new int[size];  // int형 배열의 첫 번째 원소를 가리키는 포인터 생성

    // 배열의 각 요소에 대한 값을 입력받음
    for (int i = 0; i < size; ++i) {
        cout << "배열의 " << i + 1 << "번째 값을 입력하세요: ";
        cin >> arr[i];  // 입력값을 배열의 i번째 위치에 저장
    }

    // 배열을 역순으로 출력하는 함수 호출
    reversePrintArray(arr, size);

    // 동적으로 할당된 메모리를 해제
    delete[] arr;

    return 0;
}