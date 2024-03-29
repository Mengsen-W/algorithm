/*
 * @Date: 2023-02-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-26
 * @FilePath: /algorithm/cpp/1255_max_score_words/max_score_words.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxScoreWords(vector<string>& words, vector<char>& letters, vector<int>& score) {
    int n = words.size(), res = 0;
    vector<int> count(26);
    for (auto c : letters) {
      count[c - 'a']++;
    }
    for (int s = 1; s < (1 << n); s++) {
      vector<int> wordCount(26);  // 统计子集 s 所有单词的字母数目
      for (int k = 0; k < n; k++) {
        if ((s & (1 << k)) == 0) {  // words[k] 不在子集 s 中
          continue;
        }
        for (auto c : words[k]) {
          wordCount[c - 'a']++;
        }
      }
      bool ok = true;  // 判断子集 s 是否合法
      int sum = 0;     // 保存子集 s 的得分
      for (int i = 0; i < 26; i++) {
        sum += score[i] * wordCount[i];
        ok = ok && (wordCount[i] <= count[i]);
      }
      if (ok) {
        res = max(res, sum);
      }
    }
    return res;
  }
};

int main() {
  {
    vector<string> words{"dog", "cat", "dad", "good"};
    vector<char> letters{'a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'};
    vector<int> score{1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
    int ans = 23;
    assert(Solution().maxScoreWords(words, letters, score) == ans);
  }

  {
    vector<string> words{"xxxz", "ax", "bx", "cx"};
    vector<char> letters{'z', 'a', 'b', 'c', 'x', 'x', 'x'};
    vector<int> score{4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10};
    int ans = 27;
    assert(Solution().maxScoreWords(words, letters, score) == ans);
  }

  {
    vector<string> words{"leetcode"};
    vector<char> letters{'l','e','t','c','o','d'};
    vector<int> score{0,0,1,1,1,0,0,0,0,0,0,1,0,0,1,0,0,0,0,1,0,0,0,0,0,0};
    int ans = 0;
    assert(Solution().maxScoreWords(words, letters, score) == ans);
  }
}