package trie

type Trie struct {
	subTries [26]*Trie
	isEnd    bool
}

func Constructor() Trie {
	return Trie{}
}

func (t *Trie) Insert(word string) {
	node := t
	for _, c := range word {
		if node.subTries[c-'a'] == nil {
			node.subTries[c-'a'] = &Trie{}
		}
		node = node.subTries[c-'a']
	}
	node.isEnd = true
}

func (t *Trie) Search(word string) bool {
	node := t
	for _, c := range word {
		if node.subTries[c-'a'] == nil {
			return false
		}
		node = node.subTries[c-'a']
	}
	return node.isEnd
}

func (t *Trie) StartsWith(prefix string) bool {
	node := t
	for _, c := range prefix {
		if node.subTries[c-'a'] == nil {
			return false
		}
		node = node.subTries[c-'a']
	}
	return true
}

/**
 * Your Trie object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Insert(word);
 * param_2 := obj.Search(word);
 * param_3 := obj.StartsWith(prefix);
 */
