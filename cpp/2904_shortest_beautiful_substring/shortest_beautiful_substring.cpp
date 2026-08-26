#include <algorithm>
#include <cassert>
#include <string>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  string shortestBeautifulSubstring(string s, int k) {
    if (ranges::count(s, '1') < k) {
      return "";
    }
    string ans = s;
    int cnt = 0;
    for (int left = 0, right = 0; right < s.length(); right++) {
      cnt += s[right] - '0';
      while (cnt > k || s[left] == '0') {
        cnt -= s[left] - '0';
        left++;
      }
      if (cnt == k) {
        string t = s.substr(left, right - left + 1);
        if (t.length() < ans.length() || t.length() == ans.length() && t < ans) {
          ans = move(t);
        }
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<string, int, string>> tests{
      {"100011001", 3, "11001"},
      {"1011", 2, "11"},
      {"000", 1, ""},
  };

  for (auto [s, k, expected] : tests) {
    assert(Solution().shortestBeautifulSubstring(s, k) == expected);
  }
}
