/*
 * @Date: 2023-06-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-03
 * @FilePath: /algorithm/cpp/1156_max_rep_opt1/max_rep_opt1.cpp
 */

#include <cassert>
#include <string>
#include <unordered_map>
using namespace std;

class Solution {
 public:
  int maxRepOpt1(string text) {
    unordered_map<char, int> count;
    for (auto c : text) {
      count[c]++;
    }

    int res = 0;
    for (int i = 0; i < text.size();) {
      // step1: 找出当前连续的一段 [i, j)
      int j = i;
      while (j < text.size() && text[j] == text[i]) {
        j++;
      }
      int cur_cnt = j - i;

      // step2: 如果这一段长度小于该字符出现的总数，并且前面或后面有空位，则使用 cur_cnt + 1 更新答案
      if (cur_cnt < count[text[i]] && (j < text.size() || i > 0)) {
        res = max(res, cur_cnt + 1);
      }

      // step3: 找到这一段后面与之相隔一个不同字符的另一段 [j + 1, k)，如果不存在则 k = j + 1
      int k = j + 1;
      while (k < text.size() && text[k] == text[i]) {
        k++;
      }
      res = max(res, min(k - i, count[text[i]]));
      i = j;
    }
    return res;
  }
};

int main() {
  {
    string text = "ababa";
    int ans = 3;
    assert(Solution().maxRepOpt1(text) == ans);
  }

  {
    string text = "aaabaaa";
    int ans = 6;
    assert(Solution().maxRepOpt1(text) == ans);
  }

  {
    string text = "aaabbaaa";
    int ans = 4;
    assert(Solution().maxRepOpt1(text) == ans);
  }

  {
    string text = "aaaaa";
    int ans = 5;
    assert(Solution().maxRepOpt1(text) == ans);
  }

  {
    string text = "abcdef";
    int ans = 1;
    assert(Solution().maxRepOpt1(text) == ans);
  }
}
