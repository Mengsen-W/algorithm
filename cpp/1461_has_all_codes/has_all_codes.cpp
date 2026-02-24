#include <cassert>
#include <string>
#include <tuple>
#include <unordered_set>
#include <vector>
using namespace std;

class Solution {
 public:
  bool hasAllCodes(string s, int k) {
    if (s.size() < (1 << k) + k - 1) {
      return false;
    }

    int num = stoi(s.substr(0, k), nullptr, 2);
    unordered_set<int> exists = {num};

    for (int i = 1; i + k <= s.size(); ++i) {
      num = (num - ((s[i - 1] - '0') << (k - 1))) * 2 + (s[i + k - 1] - '0');
      exists.insert(num);
    }
    return exists.size() == (1 << k);
  }
};

int main() {
  vector<tuple<string, int, bool>> tests{
      {"00110110", 2, true},
      {"0110", 1, true},
      {"0110", 2, false},
  };

  for (auto [s, k, expected] : tests) {
    assert(Solution().hasAllCodes(s, k) == expected);
  }
}