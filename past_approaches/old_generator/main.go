package main

import (
	"fmt"
	"generator/orders"
	"strconv"
	"os"
	"runtime"
	"strings"
	"sync"
)

// One order in JSON is roughly 473 bytes.
// 1GB of data is just over 2 million iterations.

func main() {
	var numOrders int
	if len(os.Args) > 1 {
		temp := os.Args[1]
		temp2, err := (strconv.ParseInt(temp, 10, 0))
		if err != nil {
			fmt.Println("Non-numeric argument passed for # of orders.")
			os.Exit(1)
		}
		numOrders = int(temp2)
	} else {
		numOrders = 2_000_000
	}
	ordersList := make([]string, numOrders)
	var wg sync.WaitGroup
	var mu sync.Mutex
	numWorkers := runtime.NumCPU()
	chunkSize := numOrders / numWorkers
	for w := range numWorkers {
		start := w * chunkSize
		end := start + chunkSize
		if w == numWorkers-1 {
			end = numOrders // handle any leftover orders
		}
		wg.Add(1)
		go func(start, end int) {
			defer wg.Done()
			tempList := make([]string, end-start)
			for i := start; i < end; i++ {
				order := orders.GenerateOrder()
				tempList[i-start] = order.ToJson()
			}
			// Safely copy tempList into the main ordersList
			mu.Lock()
			copy(ordersList[start:end], tempList)
			mu.Unlock()
		}(start, end)
	}
	wg.Wait()
	ordersStr := strings.Join(ordersList, "\n")
	file, err := os.Create("./data.json")
	if err != nil {
		fmt.Println("Unable to open file:", err)
		os.Exit(1)
	}
	defer file.Close()
	_, err = file.WriteString(ordersStr)
	if err != nil {
		fmt.Println("Error writing file:", err)
	}
}
