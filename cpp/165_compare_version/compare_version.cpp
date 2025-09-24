#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int compareVersion(string version1, string version2) {
    int n = version1.length(), m = version2.length();
    int i = 0, j = 0;
    while (i < n || j < m) {
      long long x = 0;
      for (; i < n && version1[i] != '.'; ++i) {
        x = x * 10 + version1[i] - '0';
      }
      ++i;  // 跳过点号
      long long y = 0;
      for (; j < m && version2[j] != '.'; ++j) {
        y = y * 10 + version2[j] - '0';
      }
      ++j;  // 跳过点号
      if (x != y) {
        return x > y ? 1 : -1;
      }
    }
    return 0;
  }
};

int main() {
  vector<tuple<string, string, int>> tests{
      {"1.01", "1.001", 0}, {"1.0", "1.0.0", 0}, {"0.1", "1.1", -1}, {"1.0.1", "1", 1}, {"7.5.2.4", "7.5.3", -1},
  };

  for (auto &[version1, version2, ans] : tests) {
    assert(Solution().compareVersion(version1, version2) == ans);
  }

  return 0;
}
