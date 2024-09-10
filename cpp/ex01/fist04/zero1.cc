
#include <iostream>
using namespace std;

int main(int argc, char const *argv[]) {
    // for 반복문
    // 반복 횟수가 명확할 때 주로 사용
    // 초기화, 조건식, 증감식을 한 줄에 작성해 반복을 제어
    // 1부터 5까지 숫자를 출력하는 for 반복문
    for (size_t i = 1; i <= 5; i++) {
        cout << i << endl;
    }

    // while 반복문
    // 반복 횟수가 명확하지 않을 때 주로 사용
    // 조건식이 참일 동안 반복 실행
    int i = 1;
    while (i <= 5) {
        cout << i << endl;
        i++;
    }

    // do-while 반복문
    // 조건을 나중에 검사하기 때문에, 적어도 한번은 반드시 실행되는 반복문
    i = 1;
    do {
        cout << i << endl;
        i++;
    } while (i < 0);

    // 반복문 제어 구문 break
    // 가까운 반복문을 즉시 종료하고, 다음 코드로 이동
    for (size_t i = 1; i <= 5; i++) {
        if (i == 3) {
            break;  // i가 3일 때 반복문 종료
        }
        cout << i << endl;
    }

    // continue
    // 현재 반복을 건너뛰고, 다음 반복을 계속함
    for (size_t i = 1; i <= 5; i++) {
        if (i == 3) {
            continue;  // i가 3일 때 아래 코드를 건너뛰고 다음 반복으로 넘어감
        }
        cout << i << endl;
    }

    return 0;
}
