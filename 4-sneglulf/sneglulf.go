package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
	"time"
)

type coord struct {
	x int
	y int
}

var (
	current      *coord        = &coord{}
	visitedTimes map[coord]int = make(map[coord]int, 0)
	minutesUsed  int
)

func parseCoords() []coord {
	bytes, err := ioutil.ReadFile("coords.csv")
	if err != nil {
		panic(err)
	}
	rows := strings.Split(string(bytes), "\n")
	coords := make([]coord, 0, len(rows))
	for i, row := range rows {
		if i == 0 {
			continue
		}
		nums := strings.Split(row, ",")
		if len(nums) != 2 {
			break
		}
		x, err := strconv.Atoi(nums[0])
		if err != nil {
			panic(err)
		}
		y, err := strconv.Atoi(nums[1])
		if err != nil {
			panic(err)
		}
		coords = append(coords, coord{
			x: x,
			y: y,
		})
	}
	return coords
}

func main() {
	startTime := time.Now()
	coords := parseCoords()
	fmt.Printf("%d coordinates read\n", len(coords))
	visitedTimes[coord{x: 0, y: 0}] = 1
	for _, c := range coords {
		moveTo(c)
	}
	fmt.Printf("Minutes used: %d\n", minutesUsed)
	fmt.Printf("time used: %v\n", time.Since(startTime))
}

func moveTo(c coord) {
	moveX(c.x)
	moveY(c.y)
}

func moveX(newX int) {
	move := 1
	if newX < current.x {
		move = -1
	}
	for current.x != newX {
		minutesUsed += visitedTimes[*current]
		current.x += move
		visitedTimes[*current]++
		if current.x == newX {
			break
		}
	}
}

func moveY(newY int) {
	move := 1
	if newY < current.y {
		move = -1
	}
	for current.y != newY {
		minutesUsed += visitedTimes[*current]
		current.y += move
		visitedTimes[*current]++
		if current.y == newY {
			break
		}
	}
}
