#include <algorithm>
#include <cassert>
#include <string>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
  unordered_set<char> vowels = {'a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'};

 public:
  string sortVowels(string s) {
    string tmp;
    for (char ch : s) {
      if (vowels.contains(ch)) {
        tmp.push_back(ch);
      }
    }
    sort(tmp.begin(), tmp.end());
    int idx = 0;
    for (char& ch : s) {
      if (vowels.contains(ch)) {
        ch = tmp[idx++];
      }
    }
    return s;
  }
};

int main() {
  vector<tuple<string, string>> tests{
      {"lEetcOde", "lEOtcede"},
      {"lYmpH", "lYmpH"},
  };

  for (auto [s, expect] : tests) {
    assert(Solution().sortVowels(s) == expect);
  }
}