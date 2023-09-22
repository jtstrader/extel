#include <stdio.h>
#include <stdlib.h>

int main(int argc, char** argv) {
    if (argc == 1) {
        printf("no arg passed\n");
    } else if (argc == 2) {
        printf("n passed: %d\n", atoi(argv[1]));
    } else if (argc > 2) {
        printf("too many args passed! expected <= 2, got %d\n", argc);
        return 1;
    }

    return 0;
}
