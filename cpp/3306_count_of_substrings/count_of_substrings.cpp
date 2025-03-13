#include <cassert>
#include <map>
#include <set>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long countOfSubstrings(string word, int k) {
    set<char> vowels = {'a', 'e', 'i', 'o', 'u'};
    auto count = [&](int m) -> long long {
      int n = word.size(), consonants = 0;
      long long res = 0;
      map<char, int> occur;
      for (int i = 0, j = 0; i < n; i++) {
        while (j < n && (consonants < m || occur.size() < vowels.size())) {
          if (vowels.count(word[j])) {
            occur[word[j]]++;
          } else {
            consonants++;
          }
          j++;
        }
        if (consonants >= m && occur.size() == vowels.size()) {
          res += n - j + 1;
        }
        if (vowels.count(word[i])) {
          occur[word[i]]--;
          if (occur[word[i]] == 0) {
            occur.erase(word[i]);
          }
        } else {
          consonants--;
        }
      }
      return res;
    };
    return count(k) - count(k + 1);
  }
};

int main() {
  vector<tuple<string, int, long long>> tests{
      {"aeioqq", 1, 0},
      {"aeiou", 0, 1},
      {"ieaouqqieaouqq", 1, 3},
  };

  for (auto &[word, k, ans] : tests) {
    assert(Solution().countOfSubstrings(word, k) == ans);
  }
}