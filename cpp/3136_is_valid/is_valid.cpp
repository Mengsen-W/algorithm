#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
public:
    bool isValid(string word) {
        if (word.size() < 3) {
            return false;
        }
        bool has_vowel = false;
        bool has_consonant = false;
        for (auto c : word) {
            if (isalpha(c)) {
                c = tolower(c);
                if (c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u') {
                    has_vowel = true;
                } else {
                    has_consonant = true;
                }
            } else if (!isdigit(c)) {
                return false;
            }
        }
        return has_vowel && has_consonant;
    }
};

int main() {
  vector<tuple<string, bool>> tests{
      {"234Adas", true},
      {"b3", false},
      {"a3$e", false},
  };

  for (auto &[word, expected] : tests) {
    assert(Solution().isValid(word) == expected);
  }
}