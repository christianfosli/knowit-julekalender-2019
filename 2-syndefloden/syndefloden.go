package main

import (
	"fmt"
	"io/ioutil"
	"net/http"
	"regexp"
	"strings"
)

func main() {
	holdsWater := regexp.MustCompile(`^#.+#$`)
	world := getWorld()
	var cnt int
	for _, line := range world {
		line = strings.TrimSpace(line)
		if !holdsWater.MatchString(line) {
			continue
		}
		for _, ch := range line {
			if ch == ' ' {
				cnt++
			}
		}
	}
	fmt.Printf("%d routes filled\n", cnt)
}

func getWorld() []string {
	res, err := http.Get("https://knowit-julekalender.s3.eu-central-1.amazonaws.com/2019-luke2/world.txt")
	if err != nil {
		panic(err)
	}
	bytes, err := ioutil.ReadAll(res.Body)
	if err != nil {
		panic(err)
	}
	return strings.Split(string(bytes), "\n")
}
