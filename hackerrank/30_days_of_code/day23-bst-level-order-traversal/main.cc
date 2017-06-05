#include <iostream>
#include <cstddef>
#include <queue>
#include <string>
#include <cstdlib>

using namespace std;

class Node {
 public:
  int data;
  Node *left, *right;
  explicit Node(int d) {
    data = d;
    left = right = NULL;
  }
};

class Solution {
 public:
  Node* insert(Node* root, int data) {
    if (root == NULL) {
      return new Node(data);
    } else {
      Node* cur;
      if (data <= root->data) {
        cur = insert(root->left, data);
        root->left = cur;
      } else {
        cur = insert(root->right, data);
        root->right = cur;
      }
      return root;
    }
  }

  void levelOrder(Node * root) {
    vector<int> data;
    queue<Node *> q;

    q.push(root);

    while (!q.empty()) {
      Node *n = q.front();
      data.push_back(n->data);
      q.pop();
      if (n->left) {
        q.push(n->left);
      }
      if (n->right) {
        q.push(n->right);
      }
    }

    for (auto v : data) {
      cout << v << " ";
    }
  }
};

int main() {
  Solution myTree;
  Node* root = NULL;
  int T, data;
  cin >> T;
  while (T-- > 0) {
    cin >> data;
    root = myTree.insert(root, data);
  }
  myTree.levelOrder(root);
  return 0;
}
