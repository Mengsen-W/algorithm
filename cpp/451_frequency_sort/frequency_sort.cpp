/*
 * @Date: 2021-07-03 10:14:07
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-03 11:07:27
 */

#include <cassert>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

string frequencySort(string s) {
  unordered_map<char, int> mp;
  int maxFreq = 0;
  int length = s.size();
  for (auto &ch : s) {
    maxFreq = max(maxFreq, ++mp[ch]);
  }
  vector<string> buckets(maxFreq + 1);
  for (auto &[ch, num] : mp) {
    buckets[num].push_back(ch);
  }
  string ret;
  for (int i = maxFreq; i > 0; i--) {
    string &bucket = buckets[i];
    for (auto &ch : bucket) {
      for (int k = 0; k < i; k++) {
        ret.push_back(ch);
      }
    }
  }
  return ret;
}

int main() {
  {
    string s = "tree";
    string ans = "eert";
    assert(frequencySort(s) == ans);
  }
  {
    string s = "cccaaa";
    string ans = "aaaccc";
    assert(frequencySort(s) == ans);
  }
  {
    string s = "Aabb";
    string ans = "bbaA";
    assert(frequencySort(s) == ans);
  }
}