#include <cmath>
#include <cstdio>
#include <vector>
#include <iostream>
#include <algorithm>

using namespace std;

class Difference {
 private:
  vector<int> elements;

 public:
  int maximumDifference;

  explicit Difference(vector<int> elements) : elements(elements) {
  }

  void computeDifference() {
    int min = 30000;
    int max = -30000;
    for (int i = 0; i < static_cast<int>(elements.size()); i++) {
      if (elements[i] < min) {
         min = elements[i];
      }
      if (elements[i] > max) {
         max = elements[i];
      }
    }
    this->maximumDifference = max - min;
  }
};

int main() {
  int N;
  cin >> N;

  vector<int> a;

  for (int i = 0; i < N; i++) {
    int e;
    cin >> e;

    a.push_back(e);
  }

  Difference d(a);

  d.computeDifference();

  cout << d.maximumDifference;

  return 0;
}
