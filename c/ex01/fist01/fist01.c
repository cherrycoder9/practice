#include <stdio.h>
#include <string.h>

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
        printf("1");
    }
    if (strcmp(cmd, paste_cmd) == 0) {
        printf("2");
    }
    if (strcmp(cmd, cut_cmd) == 0) {
        printf("3");
    }

    return 0;
}
