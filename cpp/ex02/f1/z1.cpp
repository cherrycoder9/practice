#include <vector>
#include <string>
#include <iostream>

int main() {
    using std::vector, std::string, std::cout;
    const vector<int> x{1, 2, 3};
    const string s{"Vector: "};
    cout << s << x[0] << ", " << x[1] << ", " << x[2] << "\n";


    return 0;
}
