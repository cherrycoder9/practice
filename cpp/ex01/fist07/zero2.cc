#include <iostream>
#include <string>

using namespace std;

void greet(string name, string greeting = "안녕!") {
    cout << greeting << ", " << name << "!" << endl;
}

int multiply(int a, int b = 2) {
    return a * b;
}

int main() {
    greet("철수");
    greet("영희", "안녕!!!!");

    cout << multiply(3) << endl;
    cout << multiply(3, 4) << endl;

    return 0;
}