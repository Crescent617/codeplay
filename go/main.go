package main

type alphabetCounter [26]int

func (a *alphabetCounter) add(c byte) {
	a[c-'a']++
}

func (a *alphabetCounter) remove(c byte) {
	a[c-'a']--
}

func (a *alphabetCounter) check() bool {
	prev := -1
	for _, count := range a {
		if count == 0 {
			continue
		}
		if prev == -1 {
			prev = count
		} else if prev != count {
			return false
		}
	}
	return true
}

func minimumSubstringsInPartition(s string) int {
	n := len(s)
	if n == 0 {
		return 0
	}
	dp := make([]int, n)
	for i := range n {
		dp[i] = -1
	}
	dp[0] = 1

	for i := 1; i < n; i++ {
		ac := &alphabetCounter{}
		for j := i; j >= 0; j-- {
			ac.add(s[j])
			if ac.check() {
				if j == 0 {
					dp[i] = 1
				} else if dp[j-1] != -1 {
					if dp[i] == -1 || dp[i] > dp[j-1]+1 {
						dp[i] = dp[j-1] + 1
					}
				}
			}
		}
	}
	return dp[n-1]
}

func main() {
	minimumSubstringsInPartition("abababaccddb")
}
