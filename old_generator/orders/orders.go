package orders

import (
	"encoding/json"
	"fmt"
	"math/rand"
	"time"

	"generator/customers"
	"generator/payments"
	"generator/products"
	"generator/utils"
)

type Order struct {
	Id       string
	Customer customers.Customer
	Product  products.Product
	Date     time.Time
	Payment  payments.Payment
	Status   OrderStatus
	Discount float64
	Quantity int
	Total    float64
}

type OrderStatus int

const (
	Pending OrderStatus = iota
	Completed
	Refunded
	Cancelled
)

var orderStatusNames = map[OrderStatus]string{
	Pending:   "Pending",
	Completed: "Completed",
	Cancelled: "Cancelled",
	Refunded:  "Refunded",
}

func (os OrderStatus) String() string {
	return orderStatusNames[os]
}

func (os OrderStatus) MarshalJSON() ([]byte, error) {
	return json.Marshal(os.String())
}

func getStatus(id string, payment payments.Payment, date time.Time) OrderStatus {
	if len(id) != 36 {
		return Cancelled
	}
	now := time.Now()
	if now.After(payment.Details.ExpirationDate()) {
		return Cancelled
	}
	monthAgo := now.AddDate(0, -1, 0)
	weekAgo := now.AddDate(0, 0, -7)
	if date.Before(monthAgo) {
		return Completed
	} else if date.After(monthAgo) && date.Before(weekAgo) {
		return Refunded
	} else if date.After(weekAgo) && date.Before(now) {
		return Completed
	} else if date.After(now) {
		return Pending
	}
	return Completed
}

func getQuantity() int {
	temp := rand.Intn(3) + rand.Intn(2) - rand.Intn(4) + rand.Intn(2)
	if temp > 0 {
		return temp
	} else {
		return rand.Intn(2) + 1
	}
}

func getDiscount(customer customers.Customer) float64 {
	switch customer.Status {
	case customers.NewCustomer:
		return 0.05
	case customers.ReturningCustomer:
		if rand.Intn(6)%2 != 0 {
			return 0.03
		} else {
			return 0.0
		}
	case customers.RewardsMember:
		return 0.1
	case customers.Employee:
		return 0.2
	case customers.Manager:
		return 0.5
	case customers.Owner:
		return 1
	default:
		return 0.0
	}
}

func getTotal(o *Order) {
	o.Total = utils.RoundDecimal((o.Product.Price * float64(o.Quantity)) - ((o.Product.Price * float64(o.Quantity)) * o.Discount))
}

func (o Order) ToJson() string {
	result, err := json.Marshal(o)
	if err != nil {
		fmt.Println("Unable to marshal Order to JSON.")
		return ""
	}
	return string(result)
}

func GenerateOrder() Order {
	id := utils.GenerateUuid()
	date := utils.GenerateDatetime()
	product := products.GenerateProduct()
	customer := customers.GenerateCustomer()
	payment := payments.NewPayment(customer.Name)
	if rand.Intn(1000)%13 == 0 {
		id += "0"
	}
	result := Order{
		Id:       id,
		Date:     date,
		Customer: customer,
		Product:  product,
		Status:   getStatus(id, payment, date),
		Payment:  payment,
		Discount: getDiscount(customer),
		Quantity: getQuantity(),
	}
	getTotal(&result)
	return result
}

func (o Order) Show() {
	fmt.Println("start~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~start")
	fmt.Println("Order Details:")
	fmt.Printf("Order ID: %s\nDate: %v\nStatus: %s\nDiscount: %0.2f\nQuantity: %d\nTotal: %0.2f\n\n", o.Id, o.Date, o.Status, o.Discount, o.Quantity, o.Total)
	fmt.Println("Customer Details:")
	fmt.Printf("Customer Id: %d\nName: %s\nEmail: %s\nAddress: %s\nStatus: %s\n\n", o.Customer.Id, o.Customer.Name, o.Customer.Email, o.Customer.Address, o.Customer.Status)
	fmt.Println("Payment Details:")
	fmt.Printf("Transaction ID: %s\nCard Number: %s\nExpiration Date: %v\n\n", o.Payment.TransactionId, payments.CardList[o.Customer.Id].Number, payments.CardList[o.Customer.Id].Expiration)
	fmt.Println("Product Details:")
	fmt.Printf("Product Id: %d\nName: %s\nCategory: %s\nPrice: %0.2f\n\n", o.Product.Id, o.Product.Name, o.Product.Category, o.Product.Price)
	fmt.Println("end~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~end")
	time.Sleep(10 * time.Millisecond)
}

/*
TODO:
shipping_address: Consider splitting into street, city, state, and zip code.
shipping_method: E.g., standard, expedited, overnight.
shipping_cost: Cost associated with shipping.
shipping_date: Timestamp for when the order was dispatched.
*/
