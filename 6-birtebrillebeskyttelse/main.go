package main

import (
	"image"
	"image/color"
	"image/png"
	"os"
)

func main() {
	img := readFile()
	corrected := parseback(img)
	outputFile, err := os.Create("birtes_brillebeskyttelse.png")
	if err != nil {
		panic(err)
	}
	png.Encode(outputFile, corrected)
	outputFile.Close()
}

func readFile() image.Image {
	reader, err := os.Open("mush.png")
	if err != nil {
		panic(err)
	}
	defer reader.Close()
	image, err := png.Decode(reader)
	if err != nil {
		panic(err)
	}
	return image
}

func parseback(i image.Image) image.Image {
	newImg := image.NewRGBA(i.Bounds())
	var lastRed uint32
	var lastGreen uint32
	var lastBlue uint32
	for y := i.Bounds().Min.Y; y < i.Bounds().Max.Y; y++ {
		for x := i.Bounds().Min.X; x < i.Bounds().Max.X; x++ {
			r, g, b, a := i.At(x, y).RGBA()
			if y == i.Bounds().Min.Y && x == i.Bounds().Min.X {
				newImg.Set(x, y, color.RGBA{
					R: uint8(r),
					G: uint8(g),
					B: uint8(b),
					A: uint8(a),
				})
			} else {
				newImg.Set(x, y, color.RGBA{
					R: uint8(r ^ lastRed),
					G: uint8(g ^ lastGreen),
					B: uint8(b ^ lastBlue),
					A: uint8(a),
				})
			}
			lastRed = r
			lastGreen = g
			lastBlue = b
		}
	}
	return newImg
}
