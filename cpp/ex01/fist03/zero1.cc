#include <iostream>
using namespace std;

int main(int argc, char const *argv[]) {
    int num = 7;

    if (num > 10) {
        cout << "num이 5보다 큼" << endl;
    } else if (num > 5) {
        cout << "num은 5보다 크고 10보다 작거나 같음" << endl;
    } else {
        cout << "num은 5보다 작거나 같음" << endl;
    }

    int day = 3;
    // switch문, 주로 정수형 또는 문자형 변수에 사용
    switch (day) {
        case 1:
            cout << "월요일" << endl;
            break;

        case 2:
            cout << "화요일" << endl;
            break;

        case 3:
            cout << "수요일" << endl;
            break;

        case 4:
            cout << "목요일" << endl;
            break;

        case 5:
            cout << "금요일" << endl;
            break;

        default:
            cout << "주말" << endl;
            break;
    }

    // 삼항 연산자
    // if-else 문을 간단하게 표현할 때 사용
    // 형식은 조건? 참일 때 값 : 거짓일 때 값;
    string result = (num > 5) ? "5보다 큼" : "5보다 작거나 같음";
    cout << result << endl;

    return 0;
}
