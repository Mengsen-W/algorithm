#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
public:
    string makeFancyString(string s) {
        string res;   // 删除后的字符串
        // 遍历 s 模拟删除过程
        for (char ch : s){
            int n = res.size();
            if (n >= 2 && res[n-1] == ch && res[n-2] == ch){
                // 如果 res 最后两个字符与当前字符均相等，则不添加
                continue;
            }
            // 反之则添加
            res.push_back(ch);
        }
        return res;
    }
};

int main() {
  vector<tuple<string, string>> tests{
      {"leeetcode", "leetcode"},
      {"aaabaaaa", "aabaa"},
      {"aab", "aab"},
  };

  for (auto& [s, expect] : tests) {
    assert(Solution().makeFancyString(s) == expect);
  }
}