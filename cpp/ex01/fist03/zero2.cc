// 숫자 범위 검사
// 사용자로부터 정수를 입력받아, 양수, 음수, 0인지 출력

#include <iostream>
using namespace std;

int main(int argc, char const *argv[]) {
    int inputNumber;

    cin >> inputNumber;
    if (inputNumber > 0) {
        cout << "양수" << endl;
    } else if (inputNumber < 0) {
        cout << "음수" << endl;
    } else {
        cout << "0" << endl;
    }

    return 0;
}
