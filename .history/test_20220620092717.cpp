#include <stdio.h>
#include <windows.h>
use namespace std;
// system F
// lambda X.lambda a:X.x
// f[int](33)
template <typename X>
X f(X a)
{
    return a;
}

int main()
{
    f<int>(33);
    printf("Hello World\n");
    system("pause");
    return 0;
}