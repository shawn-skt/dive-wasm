#include <stdio.h>
#include <emscripten/bind.h>
#include <emscripten/emscripten.h>

using namespace emscripten;

class Fib
{
public:
    int Compute(int n)
    {
        float nan1 = 0.0;
        float nan2 = 0.0;
        float nan3 = (nan1 / nan2);
        unsigned char charArray[4];
        unsigned char *pdatas = (unsigned char *)&nan3;
        for (int i = 0; i < 4; i++)
        {
            charArray[i] = *pdatas++;
            // cout << charArray[i] << i << "\t";
            printf("%d\t", int(charArray[i]));
        }
        cout << endl;
        cout << to_string(nan3) << endl;
        // cout << to_string(nan3) << endl;
        return 0;
    }
};

EMSCRIPTEN_BINDINGS(fib)
{
    class_<Fib>("Fib")
        .constructor<>()
        .function("compute", &Fib::Compute);
}