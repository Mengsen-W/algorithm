#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long numberOfPowerfulInt(long long start, long long finish, int limit, string s) {
    string start_ = to_string(start - 1), finish_ = to_string(finish);
    return calculate(finish_, s, limit) - calculate(start_, s, limit);
  }

  long long calculate(string x, string s, int limit) {
    if (x.length() < s.length()) {
      return 0;
    }
    if (x.length() == s.length()) {
      return x >= s ? 1 : 0;
    }

    string suffix = x.substr(x.length() - s.length(), s.length());
    long long count = 0;
    int preLen = x.length() - s.length();

    for (int i = 0; i < preLen; i++) {
      if (limit < (x[i] - '0')) {
        count += (long)pow(limit + 1, preLen - i);
        return count;
      }
      count += (long)(x[i] - '0') * (long)pow(limit + 1, preLen - 1 - i);
    }
    if (suffix >= s) {
      count++;
    }
    return count;
  }
};

int main() {
  vector<tuple<long long, long long, int, string, long long>> tests{
      {1, 6000, 4, "124", 5},
      {15, 215, 6, "10", 2},
      {1000, 2000, 4, "3000", 0},
  };

  for (auto &[start, finish, limit,s,ans] : tests) {
    assert(Solution().numberOfPowerfulInt(start, finish, limit, s) == ans);
  }
}
