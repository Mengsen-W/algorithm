#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string lexGreaterPermutation(string s, string target) {
    vector<int> cnt(26, 0);
    for (char c : s) {
      cnt[c - 'a']++;
    }

    string res;
    int n = target.size();
    for (int i = 0; i < n; i++) {
      int targetChar = target[i] - 'a';

      // 情况1：先尝试在当前位置放置与 target[i] 相同的字符
      if (cnt[targetChar] > 0) {
        cnt[targetChar]--;
        // 检查剩余字符能否构成大于 target[i+1:] 的字符串
        if (canFormGreater(cnt, target, i + 1)) {
          res.push_back(target[i]);
          continue;
        }
        // 不能构成更大的字符串，回溯
        cnt[targetChar]++;
      }

      // 情况2：在当前位置放置一个大于 target[i] 的字符
      for (int j = targetChar + 1; j < 26; j++) {
        if (cnt[j] > 0) {
          cnt[j]--;
          res.push_back('a' + j);
          // 剩余位置按最小字典序填充
          res += getMinString(cnt);
          return res;
        }
      }

      // 无法找到可行方案, 直接返回
      return "";
    }

    return "";
  }

 private:
  // 检查剩余字符是否能构成大于 suffix 的字符串
  bool canFormGreater(const vector<int>& cnt, const string& target, int start) {
    string maxStr = getMaxString(cnt);
    string suffix = target.substr(start);
    return maxStr > suffix;
  }

  // 获取最大字典序字符串（降序排列）
  string getMaxString(const vector<int>& cnt) {
    string res;
    for (int i = 25; i >= 0; i--) {
      res.append(cnt[i], 'a' + i);
    }
    return res;
  }

  // 获取最小字典序字符串（升序排列）
  string getMinString(const vector<int>& cnt) {
    string res;
    for (int i = 0; i < 26; i++) {
      res.append(cnt[i], 'a' + i);
    }
    return res;
  }
};

int main() {
  vector<tuple<string, string, string>> tests{
      {"abc", "bba", "bca"},
      {"leet", "code", "eelt"},
      {"baba", "bbaa", ""},
  };

  for (auto [s, target, expected] : tests) {
    assert(Solution().lexGreaterPermutation(s, target) == expected);
  }
}