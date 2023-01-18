/*
 * @Date: 2022-11-17
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-17
 * @FilePath: /algorithm/792_num_matching_subseq/num_matching_subseq.cpp
 */

#include <cassert>
#include <queue>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  int numMatchingSubseq(string s, vector<string> &words) {
    vector<queue<pair<int, int>>> queues(26);
    for (int i = 0; i < words.size(); ++i) {
      queues[words[i][0] - 'a'].emplace(i, 0);
    }
    int res = 0;
    for (char c : s) {
      auto &q = queues[c - 'a'];
      int size = q.size();
      while (size--) {
        auto [i, j] = q.front();
        q.pop();
        ++j;
        if (j == words[i].size()) {
          ++res;
        } else {
          queues[words[i][j] - 'a'].emplace(i, j);
        }
      }
    }
    return res;
  }
};

int main() {
  {
    string s = "abcde";
    vector<string> words{"a", "bb", "acd", "ace"};
    int ans = 3;
    assert(Solution().numMatchingSubseq(s, words) == ans);
  }

  {
    string s = "dsahjpjauf";
    vector<string> words{"ahjpjau","ja","ahbwzgqnuk","tnmlanowax"};
    int ans = 2;
    assert(Solution().numMatchingSubseq(s, words) == ans);
  }
}