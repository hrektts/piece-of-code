struct Node {
    int data;
    Node *left;
    Node *right;
};

bool checkOrder(Node *root, int *prev) {
    bool ret = true;
    if (!root) {
        return ret;
    }
    if (root->left) {
        ret &= checkOrder(root->left, prev);
    }
    if (*prev >= root->data) {
        return false;
    }
    *prev = root->data;
    if (root->right) {
        ret &= checkOrder(root->right, prev);
    }
    return ret;
}

bool checkBST(Node *root) {
    int data = -1;
    return checkOrder(root, &data);
}

int main() {}
