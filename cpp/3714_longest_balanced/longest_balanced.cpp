#include <cassert>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
  int case2Helper(string &s, char x, char y) {
    int n = s.size();
    int res = 0;
    unordered_map<int, int> h;
    for (int i = 0; i < n; i++) {
      if (s[i] != x && s[i] != y) {
        continue;
      }

      h.clear();
      // 分割后的子串开头，两种字符出现次数之差为 0，需要提前存放至哈希表中
      h[0] = i - 1;
      int diff = 0;
      while (i < n && (s[i] == x || s[i] == y)) {
        diff += (s[i] == x) ? 1 : -1;
        if (h.contains(diff)) {
          res = max(res, i - h[diff]);
        } else {
          h[diff] = i;
        }
        i++;
      }
    }
    return res;
  }

 public:
  int longestBalanced(string s) {
    int n = s.size();
    int res = 0;

    // 情况一，仅包括一种字符
    int last = 0;
    for (int i = 0; i < s.size(); i++) {
      if (i > 0 && s[i] == s[i - 1]) {
        last++;
      } else {
        last = 1;
      }
      res = max(res, last);
    }

    // 情况二，包含两种字符
    res = max(res, case2Helper(s, 'a', 'b'));
    res = max(res, case2Helper(s, 'b', 'c'));
    res = max(res, case2Helper(s, 'a', 'c'));

    // 情况三，包含三种字符

    // 将二元组压缩成长整型，方便作为键值存放至哈希表
    // 由于前缀和之差存在负数，所以统一增加 n
    auto getId = [&](int x, int y) -> long long { return 1ll * (x + n) << 32 | (y + n); };

    // 字符串开头，位置为 -1 的地方，键值为 getId(0, 0)
    unordered_map<long long, int> h = {{getId(0, 0), -1}};
    int pre[3] = {0, 0, 0};
    for (int i = 0; i < n; i++) {
      pre[s[i] - 'a']++;
      long long id = getId(pre[1] - pre[0], pre[1] - pre[2]);
      if (h.contains(id)) {
        res = max(res, i - h[id]);
      } else {
        h[id] = i;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"abbac", 4},
      {"aabcc", 3},
      {"aba", 2},
  };

  for (auto &[s, ans] : tests) {
    assert(Solution().longestBalanced(s) == ans);
  }
}
