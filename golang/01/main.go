package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func readlinesInt(filename string) []int {
	file, err := os.Open(filename)

	if err != nil {
		log.Fatalf("failed to open")
	}

	scanner := bufio.NewScanner(file)

	scanner.Split(bufio.ScanLines)
	var text []int

	for scanner.Scan() {
		line := scanner.Text()
		num, _ := strconv.Atoi(line)
		text = append(text, num)
	}
	file.Close()
	return text
}

func part_a() {
	numbers := readlinesInt("input.txt")
	for _, number1 := range numbers {
		for _, number2 := range numbers {
			if number1+number2 == 2020 {
				fmt.Println(number1 * number2)
				return
			}
		}
	}

}

func part_b() {
	numbers := readlinesInt("input.txt")
	for _, number1 := range numbers {
		for _, number2 := range numbers {
			for _, number3 := range numbers {
				if number1+number2+number3 == 2020 {
					fmt.Println(number1)
					fmt.Println(number2)
					fmt.Println(number3)
					fmt.Println(number1 * number2 * number3)
					return
				}
			}
		}
	}
}

func main() {
	fmt.Println("Part A")
	part_a()
	fmt.Println("Part B")
	part_b()
}
