package main

import (
	"fmt"
	"image"
	"image/color"
	"image/png"
	"io/ioutil"
	"math"
	"os"
)

type factor struct {
	n1 int
	n2 int
}

func readFile() string {
	bytes, err := ioutil.ReadFile("img.txt")
	if err != nil {
		panic(err)
	}
	return string(bytes)
}

func possibleFactors(n int) (factors []factor) {
	factors = make([]factor, 0)
	for i := int(math.Sqrt(float64(n))); i > 1; i-- {
		if n%i == 0 {
			j := n / i
			factors = append(factors, factor{
				n1: i,
				n2: j,
			})
		}
	}
	return
}

func generateImg(bitStr string, width int, height int) *image.RGBA {
	img := image.NewRGBA(image.Rect(0, 0, width, height))
	for index, b := range bitStr {
		if b == '0' {
			img.Set(index%width, index/width, color.RGBA{
				R: 0,
				G: 0,
				B: 0,
				A: 255,
			})
		} else if b == '1' {
			img.Set(index%width, index/width, color.RGBA{
				R: 255,
				G: 255,
				B: 255,
				A: 255,
			})
		} else {
			fmt.Printf("Unexpected char: %v\n", b)
		}
	}
	return img
}

func encodePngToFile(img *image.RGBA, filename string) {
	outputFile, err := os.Create(filename)
	if err != nil {
		panic(err)
	}
	png.Encode(outputFile, img)
	outputFile.Close()
}

func main() {
	bitStr := readFile()
	possFacs := possibleFactors(len(bitStr))
	fmt.Printf("%d factors found\n", len(possFacs))
	for _, fac := range possFacs {
		img := generateImg(bitStr, fac.n1, fac.n2)
		encodePngToFile(img, fmt.Sprintf("image-%d-%d.png", fac.n1, fac.n2))
		img = generateImg(bitStr, fac.n2, fac.n1)
		encodePngToFile(img, fmt.Sprintf("image-%d-%d.png", fac.n2, fac.n1))
	}
	fmt.Println("fished generating images")
}
