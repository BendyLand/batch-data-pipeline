package customers

import (
	"encoding/json"
	"math/rand"
)

type CustomerStatus int

const (
	NewCustomer CustomerStatus = iota
	ReturningCustomer
	RewardsMember
	Employee
	Manager
	Owner
)

type Customer struct {
	Id      int64
	Name    string
	Email   string
	Address string
	Status  CustomerStatus
}

var customerStatusNames = map[CustomerStatus]string{
	NewCustomer:       "New Customer",
	ReturningCustomer: "Returning Customer",
	RewardsMember:     "Rewards Member",
	Employee:          "Employee",
	Manager:           "Manager",
	Owner:             "Owner",
}

func (cs CustomerStatus) String() string {
	return customerStatusNames[cs]
}

func (s CustomerStatus) MarshalJSON() ([]byte, error) {
	return json.Marshal(s.String())
}

func getName() string {
	names := []string{
		"Ava Whitaker",
		"Liam Caldwell",
		"Isabella Greene",
		"Ethan Morrell",
		"Maya Ellison",     // returning
		"Noah Blackwood",   // returning
		"Chloe Hartman",    // returning
		"Lucas Pennington", // returning
		"Sofia Langford",   // returning
		"Oliver Drayton",   // returning
		"Harper Linwood",   // returning
		"Sebastian Knox",   // rewards
		"Amelia Fairbanks", // rewards
		"Julian Royce",     // rewards
		"Nora Halston",     // rewards
		"Elijah Trent",     // rewards
		"Zoe Merrick",      // rewards
		"Caleb Winslow",    // employee
		"Lily Hargrove",    // employee
		"Milo Carrington",  // employee
		"Aria Templeton",   // employee
		"Declan Shore",     // employee
		"Vivian Leclair",   // manager
		"Grayson Holt",     // manager
		"Clara Redmond",    // owner
	}
	choice := rand.Intn(42) % 25
	shift := rand.Intn(50)%5 == 0
	if shift {
		choice -= rand.Intn(10)
	}
	if choice < 0 {
		choice += 25
	}
	return names[choice]
}

func getEmail(name string) string {
	emails := map[string]string{
		"Ava Whitaker":     "whitaker.a@email.com",
		"Liam Caldwell":    "caldwelll@email.com",
		"Isabella Greene":  "isabella_greene@email.com",
		"Ethan Morrell":    "morrelle@email.com",
		"Maya Ellison":     "ellison_maya@email.com",
		"Noah Blackwood":   "blackwood3noah@email.com",
		"Chloe Hartman":    "chloehartman54321@emailprovider.com",
		"Lucas Pennington": "lucky_penny10@upmail.com",
		"Sofia Langford":   "sofie_lang123@email.com",
		"Oliver Drayton":   "drayton.o@email.com",
		"Harper Linwood":   "harper.lindwood@downmail.com",
		"Sebastian Knox":   "sebknox@email.com",
		"Amelia Fairbanks": "the_fairest_of_banks@email.com",
		"Julian Royce":     "royce_j@email.com",
		"Nora Halston":     "halston.n@emailprovider.com",
		"Elijah Trent":     "eli_trent123@upmail.com",
		"Zoe Merrick":      "zoe.merrick@email.com",
		"Caleb Winslow":    "employee1@store.com",
		"Lily Hargrove":    "employee2@store.com",
		"Milo Carrington":  "employee3@store.com",
		"Aria Templeton":   "employee4@store.com",
		"Declan Shore":     "employee5@store.com",
		"Vivian Leclair":   "manager1@store.com",
		"Grayson Holt":     "manager2@store.com",
		"Clara Redmond":    "owner@store.com",
	}
	return emails[name]
}

func getCustomerId(name string) int64 {
	ids := map[string]int64{
		"Ava Whitaker":     24,
		"Liam Caldwell":    23,
		"Isabella Greene":  22,
		"Ethan Morrell":    21,
		"Maya Ellison":     20,
		"Noah Blackwood":   19,
		"Chloe Hartman":    18,
		"Lucas Pennington": 17,
		"Sofia Langford":   16,
		"Oliver Drayton":   15,
		"Harper Linwood":   14,
		"Sebastian Knox":   13,
		"Amelia Fairbanks": 12,
		"Julian Royce":     11,
		"Nora Halston":     10,
		"Elijah Trent":     9,
		"Zoe Merrick":      8,
		"Caleb Winslow":    7,
		"Lily Hargrove":    6,
		"Milo Carrington":  5,
		"Aria Templeton":   4,
		"Declan Shore":     3,
		"Vivian Leclair":   2,
		"Grayson Holt":     1,
		"Clara Redmond":    0,
	}
	return ids[name]
}

func getAddress(name string) string {
	addresses := map[string]string{
		"Ava Whitaker":     "1281 Marigold Ln, Boulder, CO 80304",
		"Liam Caldwell":    "760 Pine Hollow Rd, Albany, NY 12205",
		"Isabella Greene":  "3947 Sycamore Dr, San Diego, CA 92103",
		"Ethan Morrell":    "2420 Ridgeway Ave, Memphis, TN 38104",
		"Maya Ellison":     "87 Hilltop Cir, Eugene, OR 97405",
		"Noah Blackwood":   "511 Wren St, Madison, WI 53703",
		"Chloe Hartman":    "19 Cypress View Ct, Tampa, FL 33629",
		"Lucas Pennington": "933 Windmere Way, Austin, TX 78731",
		"Sofia Langford":   "1604 Meadowlark Ln, Salt Lake City, UT 84106",
		"Oliver Drayton":   "43 Hollow Creek Rd, Durham, NC 27707",
		"Harper Linwood":   "670 Bramblewood Dr, Indianapolis, IN 46220",
		"Sebastian Knox":   "3582 Auburn Ridge Ct, Scottsdale, AZ 85251",
		"Amelia Fairbanks": "1047 Birch Haven Dr, Minneapolis, MN 55406",
		"Julian Royce":     "213 Ivy Brook Ln, Charleston, SC 29414",
		"Nora Halston":     "585 Oak Crest Blvd, Des Moines, IA 50310",
		"Elijah Trent":     "402 Clearwater Ct, Kansas City, MO 64111",
		"Zoe Merrick":      "144 Harborstone Dr, Anchorage, AK 99501",
		"Caleb Winslow":    "318 Willow Bend Dr, Raleigh, NC 27609",
		"Lily Hargrove":    "4902 Maple Grove Ln, Cary, NC 27513",
		"Milo Carrington":  "1023 Briarcliff Cir, Durham, NC 27705",
		"Aria Templeton":   "710 Pine Ridge Dr, Chapel Hill, NC 27514",
		"Declan Shore":     "855 Brookview Rd, Raleigh, NC 27606",
		"Vivian Leclair":   "6307 Oakdale Way, Apex, NC 27502",
		"Grayson Holt":     "214 Forest Glen Ct, Garner, NC 27529",
		"Clara Redmond":    "1201 Amberwood Dr, Holly Springs, NC 27540",
	}
	return addresses[name]
}

func getStatus(name string) CustomerStatus {
	switch name {
	case "Caleb Winslow", "Lily Hargrove", "Milo Carrington", "Aria Templeton", "Declan Shore":
		return Employee
	case "Vivian Leclair", "Grayson Holt":
		return Manager
	case "Clara Redmond":
		return Owner
	case "Sebastian Knox", "Amelia Fairbanks", "Julian Royce", "Nora Halston", "Elijah Trent", "Zoe Merrick":
		return RewardsMember
	case "Maya Ellison", "Noah Blackwood", "Chloe Hartman", "Lucas Pennington", "Sofia Langford", "Oliver Drayton", "Harper Linwood":
		return ReturningCustomer
	default:
		return NewCustomer
	}
}

func GenerateCustomer() Customer {
	name := getName()
	return Customer{
		Id:      getCustomerId(name),
		Name:    name,
		Email:   getEmail(name),
		Address: getAddress(name),
		Status:  getStatus(name),
	}
}

