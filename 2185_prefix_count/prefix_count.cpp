/*
 * @Date: 2023-01-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-08
 * @FilePath: /algorithm/2185_prefix_count/prefix_count.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  int prefixCount(vector<string>& words, string pref) {
    int res = 0;
    for (string& word : words) {
      if (word.compare(0, pref.size(), pref) == 0) {
        res++;
      }
    }
    return res;
  }
};

int main() {
  {
    vector<string> words{"pay", "attention", "practice", "attend"};
    string pref = "at";
    int ans = 2;
    assert(Solution().prefixCount(words, pref) == ans);
  }

  {
    vector<string> words{"leetcode", "win", "loops", "success"};
    string pref = "code";
    int ans = 0;
    assert(Solution().prefixCount(words, pref) == ans);
  }
}