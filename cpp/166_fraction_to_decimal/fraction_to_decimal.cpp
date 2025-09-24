#include <cassert>
#include <string>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  string fractionToDecimal(int numerator, int denominator) {
    if (static_cast<int64_t>(numerator) % denominator == 0)
      return to_string(static_cast<int64_t>(numerator) / denominator);

    int64_t up = abs(static_cast<int64_t>(numerator)), down = abs(static_cast<int64_t>(denominator));
    string ans(((numerator < 0) ^ (denominator < 0) ? "-" : "") + to_string(up / down) + '.');
    unordered_map<int64_t, int> index;

    for (int i = ans.size(); (up = up % down * 10); ++i) {
      if (index.count(up)) {
        ans.insert(begin(ans) + index[up], '(');
        ans.push_back(')');
        break;
      }
      index[up] = i;
      ans.push_back('0' + up / down);
    }
    return ans;
  }
};

int main() {
  vector<tuple<int, int, string>> tests{
      {1, 2, "0.5"},
      {2, 1, "2"},
      {4, 333, "0.(012)"},
      {1, 5, "0.2"},
  };

  for (auto& [numerator, denominator, expect] : tests) {
    assert(Solution().fractionToDecimal(numerator, denominator) == expect);
  }
}