#include "reverse_string.h"
// #include <string>
using namespace std;

namespace reverse_string {
    string reverse_string(string& input) {
        char temp, first, second;
        int length = input.length();
        if (length ==0) { return input;}
        for (int i = 0; i < length/2; i++) {
            first = input[i];
            second = input[length-i-1];
            swap(first, second);
            return input;
        }

    }
}  // namespace reverse_string
