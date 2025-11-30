#include <algorithm>
#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int numberOfBeams(vector<string>& bank) {
    int last = 0, ans = 0;
    for (const string& line : bank) {
      int cnt = count_if(line.begin(), line.end(), [](char ch) { return ch == '1'; });
      if (cnt != 0) {
        ans += last * cnt;
        last = cnt;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<string>, int>> tests{
      {{"011001", "000000", "010100", "001000"}, 8},
      {{"000", "111", "000"}, 0},
  };

  for (auto& [bank, ans] : tests) {
    assert(Solution().numberOfBeams(bank) == ans);
  }
}
