#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string addSpaces(string s, vector<int>& spaces) {
    int n = s.size();
    string ans;
    ans.reserve(n + spaces.size());

    int ptr = 0;
    for (int i = 0; i < n; ++i) {
      if (ptr < spaces.size() && spaces[ptr] == i) {
        ans.push_back(' ');
        ++ptr;
      }
      ans.push_back(s[i]);
    }
    return ans;
  }
};

int main() {
  vector<tuple<string, vector<int>, string>> tests{
      {"LeetcodeHelpsMeLearn", {8, 13, 15}, "Leetcode Helps Me Learn"},
      {"icodeinpython", {1, 5, 7, 9}, "i code in py thon"},
      {"spacing", {0, 1, 2, 3, 4, 5, 6}, " s p a c i n g"},
  };

  for (auto& [s, spaces, ans] : tests) {
    assert(Solution().addSpaces(s, spaces) == ans);
  }
}