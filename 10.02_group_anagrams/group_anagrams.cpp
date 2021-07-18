/*
 * @Date: 2021-07-18 16:52:34
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-18 19:12:49
 */

#include <cassert>
#include <iostream>
#include <numeric>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<string>> groupAnagrams(vector<string>& strs) {
    // 自定义对 array<int, 26> 类型的哈希函数
    auto arrayHash = [fn = hash<int>{}](const array<int, 26>& arr) -> size_t {
      return accumulate(arr.begin(), arr.end(), 0u, [&](size_t acc, int num) {
        return (acc << 1) ^ fn(num);
      });
    };

    unordered_map<array<int, 26>, vector<string>, decltype(arrayHash)> mp(
        0, arrayHash);
    for (string& str : strs) {
      array<int, 26> counts{};
      int length = str.length();
      for (int i = 0; i < length; ++i) {
        counts[str[i] - 'a']++;
      }
      mp[counts].emplace_back(str);
    }
    vector<vector<string>> ans;
    for (auto it = mp.begin(); it != mp.end(); ++it) {
      ans.emplace_back(it->second);
    }
    return ans;
  }
};

class Solution_2 {
 public:
  vector<vector<string>> groupAnagrams(vector<string>& strs) {
    unordered_map<uint64_t, vector<string>> m;
    uint64_t pf[26] = {1}, b = 97755331;
    for (int i = 1; i < 26; ++i) pf[i] = pf[i - 1] * b;
    for (auto& t : strs) {
      uint64_t hash = 0;
      for (char c : t) hash += c * pf[c - 'a'];
      m[hash].push_back(t);
    }
    vector<vector<string>> ans;
    for (auto& [_, vs] : m) ans.emplace_back(move(vs));
    return ans;
  }
};

int main() {
  {
    vector<string> strs{"eat", "tea", "tan", "ate", "nat", "bat"};
    for (vector<string>& group_str : Solution().groupAnagrams(strs)) {
      for (string& s : group_str) {
        cout << s << ", ";
      }
      cout << endl;
    }
  }
  {
    vector<string> strs{"eat", "tea", "tan", "ate", "nat", "bat"};
    for (vector<string>& group_str : Solution_2().groupAnagrams(strs)) {
      for (string& s : group_str) {
        cout << s << ", ";
      }
      cout << endl;
    }
  }
}