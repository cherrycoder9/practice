// ������ (enum)
#include <iostream>
using namespace std;

enum Color { RED, GREEN, BLUE };  // 0, 1, 2

int main(int argc, char const *argv[]) {
    Color myColor = GREEN;

    if (myColor == GREEN) {
        cout << "����: " << GREEN << endl;  // 1
    }

    return 0;
}
