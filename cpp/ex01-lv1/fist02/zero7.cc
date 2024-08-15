// 구조체 활용하기
// 학생 정보를 저장하는 구조체 Student 정의
// 학생 한 명의 이름, 학번, 평균 점수를 저장한 후 출력

#include <iostream>
using namespace std;

struct Student {
    string name;
    string id;
    double averageScore;
};

int main(int argc, char const *argv[]) {
    Student student = {"Cherry", "20240199", 99.999};
    cout << student.name << endl;
    cout << student.id << endl;
    cout << student.averageScore << endl;

    return 0;
}
