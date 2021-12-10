/*
 * @Date: 2021-12-10 09:28:52
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-10 09:36:58
 */

#include <array>
#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  string shortestCompletingWord(string licensePlate, vector<string> words) {
    array<int, 26> cnt{};
    for (char ch : licensePlate)
      if (isalpha(ch)) ++cnt[tolower(ch) - 'a'];
    int idx = -1;
    int words_size = words.size();
    for (int i = 0; i < words_size; ++i) {
      array<int, 26> c{};
      for (char ch : words[i]) {
        ++c[ch - 'a'];
      }
      bool ok = true;
      for (int j = 0; j < 26; ++j) {
        if (c[j] < cnt[j]) {
          ok = false;
          break;
        }
      }
      if (ok && (idx < 0 || words[i].length() < words[idx].length())) idx = i;
    }
    return words[idx];
  }
};

int main() {
  assert(Solution().shortestCompletingWord(
             "1s3 PSt", vector<string>{"step", "steps", "stripe", "stepple"}) ==
         "steps");
  assert(Solution().shortestCompletingWord(
             "1s3 456", vector<string>{"looks", "pest", "stew", "show"}) ==
         "pest");
  assert(Solution().shortestCompletingWord(
             "Ah71752", vector<string>{"suggest", "letter", "of", "husband",
                                       "easy", "education", "drug", "prevent",
                                       "writer", "old"}) == "husband");
  assert(Solution().shortestCompletingWord(
             "OgEu755",
             vector<string>{"enough", "these", "play", "wide", "wonder", "box",
                            "arrive", "money", "tax", "thus"}) == "enough");
  assert(Solution().shortestCompletingWord(
             "iMSlpe4", vector<string>{"claim", "consumer", "student", "camera",
                                       "public", "never", "wonder", "simple",
                                       "thought", "use"}) == "simple");
}