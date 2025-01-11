#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int generateKey(int num1, int num2, int num3) {
    int key = 0;
    for (int p = 1; num1 && num2 && num3; p *= 10) {
      key += min({num1 % 10, num2 % 10, num3 % 10}) * p;
      num1 /= 10;
      num2 /= 10;
      num3 /= 10;
    }
    return key;
  }
};

int main() {
  vector<tuple<int, int, int, int>> tests{
      {1, 10, 1000, 0},
      {987, 879, 798, 777},
      {1, 2, 3, 1},
  };

  for (auto &[num1, num2, num3, ans] : tests) {
    assert(Solution().generateKey(num1, num2, num3) == ans);
  }
}