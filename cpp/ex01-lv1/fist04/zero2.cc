// 숫자 합계 구하기
// 양의 정수를 입력받아 1부터 입력된 숫자까지의 합 출력

#include <iostream>
using namespace std;

int main(int argc, char const *argv[]) {
    int count;
    int sum = 0;
    cin >> count;

    for (size_t i = 1; i <= count; i++) {
        sum += i;
    }

    cout << sum << endl;
    return 0;
}
