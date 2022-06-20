#include <stdio.h>
#include <windows.h>
using namespace std;
// system F
// lambda X.lambda a:X.x
// f[int](33)
template <typename X>
X f(X a)
{
    return a;
}

template <typename X, typename Y>
class Pair
{
    X a;
    Y b;
};

int main()
{
    f<int>(33);
    Pair<int, int> p;
    printf("Hello World\n");
    system("pause");
    return 0;
}