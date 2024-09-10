// 학점 계산
// 0에서 100점 사이의 시험점수를 입력받음
// 90 이상이면 "A" 출력
// 80 이상이면 "B"
// 70 이상이면 "C"
// 60 이상이면 "D"
// 60 미만이면 "F"

#include <iostream>
using namespace std;

int main(int argc, char const *argv[]) {
    int score;

    cout << "시험점수를 입력하세요: ";
    cin >> score;

    if (score >= 90) {
        cout << "A" << endl;
    } else if (score >= 80) {
        cout << "B" << endl;
    } else if (score >= 70) {
        cout << "C" << endl;
    } else if (score >= 60) {
        cout << "D" << endl;
    } else {
        cout << "F" << endl;
    }

    return 0;
}
