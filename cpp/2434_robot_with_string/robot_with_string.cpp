#include <cassert>
#include <stack>
#include <string>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  string robotWithString(string s) {
    unordered_map<char, int> cnt;
    for (char c : s) {
      cnt[c]++;
    }

    stack<char> stk;
    string res;
    char minCharacter = 'a';
    for (char c : s) {
      stk.emplace(c);
      cnt[c]--;
      while (minCharacter != 'z' && cnt[minCharacter] == 0) {
        minCharacter++;
      }
      while (!stk.empty() && stk.top() <= minCharacter) {
        res.push_back(stk.top());
        stk.pop();
      }
    }

    return res;
  }
};

int main() {
  vector<tuple<string, string>> tests {
    {"zza", "azz"},
    {"bac", "abc"},
    {"bdda", "addb"},
  };

  for (auto &[s, ans] : tests) {
    assert(Solution().robotWithString(s) == ans);
  }
}