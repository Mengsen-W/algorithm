/*
 * @Date: 2021-12-04 05:35:54
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-04 05:40:45
 */

#include <cassert>
#include <string>
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
  assert(Solution().canConstruct("a", "b") == false);
  assert(Solution().canConstruct("aa", "ab") == false);
  assert(Solution().canConstruct("aa", "aab") == true);
}