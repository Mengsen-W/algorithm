/*
 * @Date: 2022-04-17 09:05:39
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-17 09:22:35
 * @FilePath: /algorithm/819_most_common_word/most_common_word.cpp
 */

#include <cassert>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  string mostCommonWord(string paragraph, vector<string> banned) {
    unordered_set<string> bannedSet;
    for (auto& word : banned) {
      bannedSet.emplace(word);
    }
    int maxFrequency = 0;
    unordered_map<string, int> frequencies;
    string word;
    int length = paragraph.size();
    for (int i = 0; i <= length; i++) {
      if (i < length && isalpha(paragraph[i])) {
        word.push_back(tolower(paragraph[i]));
      } else if (word.size() > 0) {
        if (!bannedSet.count(word)) {
          frequencies[word]++;
          maxFrequency = max(maxFrequency, frequencies[word]);
        }
        word = "";
      }
    }
    string mostCommon = "";
    for (auto& [word, frequency] : frequencies) {
      if (frequency == maxFrequency) {
        mostCommon = word;
        break;
      }
    }
    return mostCommon;
  }
};

int main() {
  assert(Solution().mostCommonWord("Bob hit a ball, the hit BALL flew far after it was hit.", vector<string>{"hit"}) ==
         "ball");
}