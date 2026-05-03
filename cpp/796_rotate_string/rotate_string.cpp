#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  bool rotateString(string s, string goal) {
    return s.size() == goal.size() && (s + s).find(goal) != string::npos;
  }
};

int main() {
  assert(Solution().rotateString("abcde", "cdeab") == true);
  assert(Solution().rotateString("abcde", "abced") == false);
}