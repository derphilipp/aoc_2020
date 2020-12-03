package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

type passwordline struct {
	password     string
	firstNumber  int
	secondNumber int
	character    string
}

func readlines(filename string) []passwordline {
	file, err := os.Open(filename)

	if err != nil {
		log.Fatalf("failed to open")
	}

	scanner := bufio.NewScanner(file)

	scanner.Split(bufio.ScanLines)
	var lines []passwordline

	for scanner.Scan() {
		line := scanner.Text()
		p := passwordline{}
		fmt.Sscanf(line, "%d-%d %1s: %s", &p.firstNumber, &p.secondNumber, &p.character, &p.password)
		lines = append(lines, p)
	}
	file.Close()
	return lines
}

func part_a() {
	i := 0
	passwords := readlines("input.txt")
	for _, password := range passwords {
		amount := strings.Count(password.password, password.character)
		if password.firstNumber <= amount && amount <= password.secondNumber {
			i++
		}
	}
	fmt.Printf("Total: %d\n", i)
}

func part_b() {
	i := 0
	passwords := readlines("input.txt")
	for _, password := range passwords {
		if (password.password[password.firstNumber-1] == password.character[0]) != (password.password[password.secondNumber-1] == password.character[0]) {
			i++
		}
	}
	fmt.Printf("Total: %d\n", i)
}

func main() {
	fmt.Println("Part A")
	part_a()
	fmt.Println("Part B")
	part_b()
}
