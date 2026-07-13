#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> sequentialDigits(int low, int high) {
    vector<int> ans;
    for (int i = 1; i <= 9; ++i) {
      int num = i;
      for (int j = i + 1; j <= 9; ++j) {
        num = num * 10 + j;
        if (num >= low && num <= high) {
          ans.push_back(num);
        }
      }
    }
    sort(ans.begin(), ans.end());
    return ans;
  }
};

int main() {
  vector<tuple<int, int, vector<int>>> tests{
      {100, 300, {123, 234}},
      {1000, 13000, {1234, 2345, 3456, 4567, 5678, 6789, 12345}},
  };

  for (auto [low, high, expected] : tests) {
    assert(Solution().sequentialDigits(low, high) == expected);
  }

  return 0;
}