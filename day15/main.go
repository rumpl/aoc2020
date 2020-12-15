package main

import (
	"fmt"
	"log"
)

func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
}

func run() error {
	input := []int{14, 3, 1, 0, 9, 5}
	m := map[int][]int{}

	for i, v := range input {
		m[v] = []int{i + 1}
	}

	for i := len(input); i < 30000000; i++ {
		ii := len(input) + 1
		n := input[len(input)-1]

		if len(m[n]) == 1 {
			input = append(input, 0)
			ma := m[0]
			ma = append(ma, ii)
			m[0] = ma
		} else {
			ma := m[n]
			nn := ma[len(ma)-1] - ma[len(ma)-2]
			input = append(input, nn)
			mb := m[nn]
			mb = append(mb, ii)
			m[nn] = mb
		}
	}

	fmt.Println(input[len(input)-1])

	return nil
}
