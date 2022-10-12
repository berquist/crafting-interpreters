#include <stdio.h>

int main() {
    int a = 1;
    {
        int a = a + 2;
        printf("%d\n", a);
    }
    printf("%d\n", a);

    return 0;
}
