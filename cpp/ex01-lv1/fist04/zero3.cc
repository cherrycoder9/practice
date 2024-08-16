// 특정 숫자 건너뛰기
// 1~20까지 숫자 출력, 10의 배수는 건너뛰기

#include <iostream>
using namespace std;

int main(int argc, char const *argv[]) {
    for (int i = 1; i <= 20; i++) {
        if (i % 10 == 0) {
            continue;
        }
        cout << i << endl;
    }

    return 0;
}
