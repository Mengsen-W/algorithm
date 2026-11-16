#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string digitSum(string s, int k) {
    while (s.size() > k) {
      string tmp;  // 每次操作结束的字符串
      int n = s.size();
      for (int i = 0; i < n; i += k) {
        int val = 0;
        for (int j = i; j < min(i + k, n); ++j) {
          val += s[j] - '0';
        }
        tmp.append(to_string(val));
      }
      s = tmp;
    }
    return s;
  }
};

int main() {
  vector<tuple<string, int, string>> tests{
      {"11111222223", 3, "135"},
      {"00000000", 3, "000"},
  };

  for (auto [s, k, expected] : tests) {
    assert(Solution().digitSum(s, k) == expected);
  }
}