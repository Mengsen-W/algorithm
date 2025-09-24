#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> getNoZeroIntegers(int n) {
    for (int A = 1; A < n; ++A) {
      int B = n - A;
      if ((to_string(A) + to_string(B)).find('0') == string::npos) {
        return {A, B};
      }
    }
    return {};
  }
};

int main() {
  vector<tuple<int, vector<int>>> tests{
      {2, {1, 1}},
      {11, {2, 9}},
      {10000, {1, 9999}},
      {69, {1, 68}},
      {1010, {11, 999}},
  };

  for (auto &[n, expect] : tests) {
    assert(Solution().getNoZeroIntegers(n) == expect);
  }
}