#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
public:
    char nextGreatestLetter(vector<char> &letters, char target) {
        return target < letters.back() ? *upper_bound(letters.begin(), letters.end() - 1, target) : letters[0];
    }
};

int main() {
  vector<tuple<vector<char>, char, char>> tests{
      {{'c', 'f', 'j'}, 'a', 'c'},
      {{'c', 'f', 'j'}, 'c', 'f'},
      {{'x', 'x', 'y', 'y'}, 'z', 'x'},
  };

  for (auto [letters, target, expected] : tests) {
    assert(Solution().nextGreatestLetter(letters, target) == expected);
  }
}