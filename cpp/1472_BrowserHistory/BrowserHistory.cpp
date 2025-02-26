#include <cassert>
#include <string>
#include <vector>

using namespace std;

class BrowserHistory {
 public:
  BrowserHistory(string homepage) {
    this->urls.push_back(homepage);
    this->currIndex = 0;
  }

  void visit(string url) {
    while (urls.size() > currIndex + 1) {
      urls.pop_back();
    }
    urls.push_back(url);
    this->currIndex++;
  }

  string back(int steps) {
    currIndex = max(currIndex - steps, 0);
    return urls[currIndex];
  }

  string forward(int steps) {
    currIndex = min(currIndex + steps, int(urls.size() - 1));
    return urls[currIndex];
  }

 private:
  vector<string> urls;
  int currIndex;
};

int main() {
  BrowserHistory browserHistory = BrowserHistory("leetcode.com");
  browserHistory.visit("google.com");
  browserHistory.visit("facebook.com");
  browserHistory.visit("youtube.com");
  assert(browserHistory.back(1) == "facebook.com");
  assert(browserHistory.back(1) == "google.com");
  assert(browserHistory.forward(1) == "facebook.com");
  browserHistory.visit("linkedin.com");
  assert(browserHistory.forward(2) == "linkedin.com");
  assert(browserHistory.back(2) == "google.com");
  assert(browserHistory.back(7) == "leetcode.com");
}