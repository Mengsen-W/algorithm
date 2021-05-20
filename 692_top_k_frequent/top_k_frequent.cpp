/*
 * @Date: 2021-05-20 08:48:51
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-20 09:09:07
 */

#include <cassert>
#include <queue>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

vector<string> topKFrequent(const vector<string>& words, int k) {
  unordered_map<string, int> cnt;
  for (auto& word : words) {
    cnt[word]++;
  }
  auto cmp = [](const pair<string, int>& a, const pair<string, int>& b) {
    return a.second == b.second ? a.first < b.first : a.second > b.second;
  };
  priority_queue<pair<string, int>, vector<pair<string, int>>, decltype(cmp)>
      que(cmp);
  for (auto& it : cnt) {
    que.emplace(it);
    if (que.size() > k) {
      que.pop();
    }
  }
  vector<string> ret(k);
  for (int i = k - 1; i >= 0; i--) {
    ret[i] = que.top().first;
    que.pop();
  }
  return ret;
}

int main() {
  vector<string> words{"i", "love", "leetcode", "i", "love", "coding"};
  vector<string> ans{"i", "love"};
  assert(topKFrequent(words, 2) == ans);
  words = {"the", "day", "is",    "sunny", "the",
           "the", "the", "sunny", "is",    "is"};
  ans = {"the", "is", "sunny", "day"};
  assert(topKFrequent(words, 4) == ans);
}