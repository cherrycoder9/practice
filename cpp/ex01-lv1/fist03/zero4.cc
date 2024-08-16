// 요일 출력
// 1부터 7사이의 정수 입력
// 해당하는 요일 출력
// 월요일 1부터 시작
// 1~7 사이의 숫자가 아닌 경우 "잘못된 입력" 출력

#include <iostream>
using namespace std;

int main(int argc, char const *argv[]) {
    int weekday;
    cin >> weekday;
    switch (weekday) {
        case 1:
            cout << "월";
            break;
        case 2:
            cout << "화";
            break;
        case 3:
            cout << "수";
            break;
        case 4:
            cout << "목";
            break;
        case 5:
            cout << "금";
            break;
        case 6:
            cout << "토";
            break;
        case 7:
            cout << "일";
            break;
        default:
            cout << "잘못된 입력";
            break;
    }
    return 0;
}
