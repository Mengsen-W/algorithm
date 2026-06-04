#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int totalWaviness(int num1, int num2) {
    auto getWaviness = [](int x) -> int {
      string s = to_string(x);
      int waviness = 0;

      for (size_t i = 1; i < s.size() - 1; ++i) {
        bool isPeak = s[i] > s[i - 1] && s[i] > s[i + 1];
        bool isValley = s[i] < s[i - 1] && s[i] < s[i + 1];
        if (isPeak || isValley) {
          ++waviness;
        }
      }

      return waviness;
    };

    int total = 0;
    for (int i = num1; i <= num2; ++i) {
      total += getWaviness(i);
    }

    return total;
  }
};

int main() {
  vector<tuple<int, int, int>> testCases{
      {120, 130, 3},
      {198, 202, 3},
      {4848, 4848, 2},
  };

  for (const auto& [num1, num2, expected] : testCases) {
    assert(Solution().totalWaviness(num1, num2) == expected);
  }
}