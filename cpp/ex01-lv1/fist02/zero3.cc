// 열거형 (enum)
#include <iostream>
using namespace std;

enum Color { RED, GREEN, BLUE };  // 0, 1, 2

int main(int argc, char const *argv[]) {
    Color myColor = GREEN;

    if (myColor == GREEN) {
        cout << "색상: " << GREEN << endl;  // 1
    }

    return 0;
}
