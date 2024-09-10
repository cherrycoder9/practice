// 역순 출력
// 양의 정수를 입력받아 그 숫자부터 1까지 역순 출력

#include <iostream>
using namespace std;

int main(int argc, char const *argv[]) {
    int num;
    cin >> num;

    for (int i = num; i != 0; i--) {
        cout << i << endl;
    }

    return 0;
}
