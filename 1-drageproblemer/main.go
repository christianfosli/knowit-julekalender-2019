package main

import (
	"errors"
	"fmt"
	"io/ioutil"
	"log"
	"strconv"
	"strings"
)

var (
	dragonSize      int = 50
	leftoverSheep   int
	daysWithoutFood int
)

func main() {
	sheep := readSheepFromFile()
	for i, sheepCnt := range sheep {
		err := eatSheep(sheepCnt)
		if err != nil {
			fmt.Println(err)
			fmt.Printf("Days survived: %d\n", i)
			break
		}
	}
}

func readSheepFromFile() []int {
	data, err := ioutil.ReadFile("sau.txt")
	if err != nil {
		log.Fatalf("Unable to read file: %s\n", err)
	}
	sheepCountAsStr := strings.Split(string(data), ", ")
	sheepCountAsInt := make([]int, 0, len(sheepCountAsStr))
	for _, cnt := range sheepCountAsStr {
		cntAsInt, err := strconv.Atoi(cnt)
		if err != nil {
			log.Fatalf("Parsing error trying to parse to int: %s\n", err)
		}
		sheepCountAsInt = append(sheepCountAsInt, cntAsInt)
	}
	return sheepCountAsInt
}

func eatSheep(n int) error {
	n += leftoverSheep
	leftoverSheep = 0
	if n >= dragonSize {
		leftoverSheep = n - dragonSize
		dragonSize++
		daysWithoutFood = 0
	} else if n < dragonSize {
		dragonSize--
		daysWithoutFood++
	}
	if daysWithoutFood >= 5 {
		return errors.New("Dragon goes crazy and eats everyone")
	}
	return nil
}
