// int형 2차원 배열을 동적으로 할당하고, 사용자로부터
// m x n 크기의 배열을 입력받아 배열의 각 요소에 값을 입력
// 및 출력하는 프로그램 작성, 출력후 메모리 해제

#include <iostream>

using namespace std;

// 행과 열의 크기를 입력받아 힙 메모리 상에서 2차원 배열 할당
int** allocate2DArray(int m, int n) {
    // 배열의 첫 번째 차원(행)을 동적으로 할당
    int** array = new int*[m];

    // 각 행에 대해 두 번째 차원(열)을 동적으로 할당
    for (int i = 0; i < m; ++i) {
        array[i] = new int[n];
    }

    cout << "메모리 할당 완료: " << m << "x" << n << " 크기의 배열" << endl;

    return array;  // 할당된 2차원 배열의 포인터 반환
}

// 배열의 행 수 m을 받아 각 행에 대해 메모리를 해제하고
// 마지막으로 전체 배열 해제
void deallocate2DArray(int** array, int m) {
    // 할당된 각 행의 메모리부터 해제
    for (int i = 0; i < m; ++i) {
        delete[] array[i];  // 각 행을 해제
    }
    // 1차원 배열의 포인터 배열 해제
    delete[] array;

    cout << "메모리 해제 완료" << endl;
}

int main() {
    int m, n;

    // 사용자로부터 배열의 크기(행, 열)를 입력받음
    cout << "배열의 행 크기를 입력하세요: ";
    cin >> m;
    cout << "배열의 열 크기를 입력하세요: ";
    cin >> n;

    // 2차원 배열 동적 할당
    int** array = allocate2DArray(m, n);

    // 배열에 값 입력
    cout << "배열의 값을 입력하세요." << endl;
    for (int i = 0; i < m; ++i) {
        for (int j = 0; j < n; ++j) {
            cout << "array[" << i << "][]" << j << "] = ";
            cin >> array[i][j];  // 각 위치에 대해 값 입력
        }
    }

    // 배열 출력
    cout << "입력된 배열 값:" << endl;
    for (int i = 0; i < m; ++i) {
        for (int j = 0; j < n; ++j) {
            cout << array[i][j] << " ";  // qodufdml rkqt cnffur
        }
        cout << endl;
    }

    // 배열 메모리 해제
    deallocate2DArray(array, m);

    return 0;
}