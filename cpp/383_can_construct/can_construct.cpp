/*
 * @Date: 2021-12-04 05:35:54
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-07
 */

#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool canConstruct(string ransomNote, string magazine) {
    if (ransomNote.size() > magazine.size()) return false;

    vector<int> cnt(26);
    for (auto& c : magazine) cnt[c - 'a']++;

    for (auto& c : ransomNote) {
      cnt[c - 'a']--;
      if (cnt[c - 'a'] < 0) return false;
    }
    return true;
  }
};

int main() {
  vector<tuple<string, string, bool>> tests{
      {"a", "b", false},
      {"aa", "ab", false},
      {"aa", "aab", true},
  };
  for (auto& [ransomNote, magazine, ans] : tests) {
    assert(Solution().canConstruct(ransomNote, magazine) == ans);
  }
}