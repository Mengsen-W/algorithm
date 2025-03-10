#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int divisorSubstrings(int num, int k) {
    string s = to_string(num);  // num 十进制表示字符串
    int n = s.size();
    int res = 0;
    for (int i = 0; i <= n - k; ++i) {
      // 枚举所有长度为 k 的子串
      int tmp = stoi(s.substr(i, k));
      if (tmp && num % tmp == 0) {
        ++res;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<int, int, int>> tests{
      {240, 2, 2},
      {430043, 2, 2},
  };

  for (auto &[num, k, ans] : tests) {
    assert(Solution().divisorSubstrings(num, k) == ans);
  }
}