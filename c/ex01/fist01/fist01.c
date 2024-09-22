#include <stdio.h>
#include <string.h>

// 매크로를 확장할 때 # 연산자는 매개변수를 한 쌍의 따옴표로 둘러싼
// 문자 형태로 변환

// ## 연산자는 매크로 정의에서 매개변수와
// 다른 요소를 문자열로 결합해 변수 이름을 만든다

#define CMD(NAME)              \
    char NAME##_cmd[256] = ""; \
    strcpy(NAME##_cmd, #NAME);

int main(int argc, char const *argv[]) {
    CMD(copy)
    CMD(paste)
    CMD(cut)

    char cmd[256];
    scanf("%s", cmd);

    if (strcmp(cmd, copy_cmd) == 0) {
        // copy 입력시
        printf("1");
    }
    if (strcmp(cmd, paste_cmd) == 0) {
        // paste 입력시
        printf("2");
    }
    if (strcmp(cmd, cut_cmd) == 0) {
        // cut 입력시
        printf("3");
    }

    return 0;
}
