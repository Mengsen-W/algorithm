/*
 * @Date: 2021-05-30 09:22:43
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-30 09:28:53
 */

#include <cassert>

static constexpr int BIG = 1 << 20;

bool isPowerOfTwo_1(int n) { return n > 0 && (n & (n - 1)) == 0; }
bool isPowerOfTwo_2(int n) { return n > 0 && (n & -n) == n; }
bool isPowerOfTwo_3(int n) { return n > 0 && BIG % n == 0; }

class Solution {
private:
    static constexpr int BIG = 1 << 30;

public:
    bool isPowerOfTwo(int n) {
        return n > 0 && BIG % n == 0;
    }
};

int main() {
  assert(isPowerOfTwo_1(1));
  assert(isPowerOfTwo_2(1));
  assert(isPowerOfTwo_3(1));
  assert(isPowerOfTwo_1(16));
  assert(isPowerOfTwo_2(16));
  assert(isPowerOfTwo_3(16));
  assert(!isPowerOfTwo_1(3));
  assert(!isPowerOfTwo_2(3));
  assert(!isPowerOfTwo_3(3));
  assert(isPowerOfTwo_1(4));
  assert(isPowerOfTwo_2(4));
  assert(isPowerOfTwo_3(4));
  assert(!isPowerOfTwo_1(5));
  assert(!isPowerOfTwo_2(5));
  assert(!isPowerOfTwo_3(5));
}