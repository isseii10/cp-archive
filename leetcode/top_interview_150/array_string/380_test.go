package arraystring

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestRandomizedSet_case6(t *testing.T) {
	randomizedSet := Constructor()

	b := randomizedSet.Insert(0)
	require.Equal(t, true, b)
	b = randomizedSet.Insert(1)
	require.Equal(t, true, b)
	b = randomizedSet.Remove(0)
	require.Equal(t, true, b)
	b = randomizedSet.Insert(2)
	require.Equal(t, true, b)
	b = randomizedSet.Remove(1)
	require.Equal(t, true, b)
	v := randomizedSet.GetRandom()
	require.Equal(t, 2, v)
}
