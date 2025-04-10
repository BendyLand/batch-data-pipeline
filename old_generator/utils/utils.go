package utils

import (
	"github.com/google/uuid"
	"math"
	"math/rand"
	"time"
)

func GenerateUuid() string {
	return uuid.NewString()
}

func GenerateDatetime() time.Time {
	now := time.Now()
	currentYear := now.Year()
	start := time.Date(currentYear, time.January, 1, 0, 0, 0, 0, time.UTC)
	end := now.AddDate(0, 0, 7)
	secondsRange := end.Unix() - start.Unix()
	randomSeconds := rand.Int63n(secondsRange)
	return start.Add(time.Duration(randomSeconds) * time.Second)
}

func RoundDecimal(val float64) float64 {
	return math.Round(val*100) / 100
}
