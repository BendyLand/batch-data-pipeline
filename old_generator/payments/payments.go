package payments

import (
	"generator/utils"
	"math/rand"
	"time"
)

type PaymentMethod interface {
	ExpirationDate() time.Time
}

type CardDetails struct {
	Number     string
	Expiration time.Time
}

type Wallet struct {
	Expiration time.Time
}

func (c CardDetails) ExpirationDate() time.Time {
	return c.Expiration
}

func (w Wallet) ExpirationDate() time.Time {
	return w.Expiration
}

type Payment struct {
	Details       PaymentMethod
	TransactionId string
}

func parseExpDate(expStr string) time.Time {
	layout := "01/06"
	t, _ := time.Parse(layout, expStr)
	// Add 1 month, then subtract 1 day to get the last day of the original month
	firstOfNextMonth := t.AddDate(0, 1, 0)
	endOfMonth := firstOfNextMonth.AddDate(0, 0, -1)
	return endOfMonth
}

var CardList = map[int64]CardDetails{
	0:  {"************5171", parseExpDate("05/25")},
	1:  {"************7690", parseExpDate("05/27")},
	2:  {"************2763", parseExpDate("06/24")},
	3:  {"************3315", parseExpDate("04/26")},
	4:  {"************4072", parseExpDate("01/28")},
	5:  {"************6869", parseExpDate("11/25")},
	6:  {"************2781", parseExpDate("08/26")},
	7:  {"************0904", parseExpDate("09/25")},
	8:  {"************4469", parseExpDate("09/27")},
	9:  {"************5164", parseExpDate("08/26")},
	10: {"************3341", parseExpDate("04/25")},
	11: {"************1349", parseExpDate("07/26")},
	12: {"************4167", parseExpDate("03/25")},
	13: {"************7168", parseExpDate("09/25")},
	14: {"************8239", parseExpDate("10/26")},
	15: {"************0932", parseExpDate("11/26")},
	16: {"************8186", parseExpDate("08/27")},
	17: {"************7668", parseExpDate("11/25")},
	18: {"************9486", parseExpDate("09/27")},
	19: {"************1093", parseExpDate("02/27")},
	20: {"************0238", parseExpDate("01/26")},
	21: {"************9611", parseExpDate("02/28")},
	22: {"************7295", parseExpDate("09/26")},
	23: {"************8346", parseExpDate("08/27")},
	24: {"************2464", parseExpDate("01/26")},
}

var DigitalWalletList = map[int64]Wallet{
	0:  {parseExpDate("05/25")},
	1:  {parseExpDate("05/27")},
	2:  {parseExpDate("06/24")},
	3:  {parseExpDate("04/26")},
	4:  {parseExpDate("01/28")},
	5:  {parseExpDate("11/25")},
	6:  {parseExpDate("08/26")},
	7:  {parseExpDate("09/25")},
	8:  {parseExpDate("09/27")},
	9:  {parseExpDate("08/26")},
	10: {parseExpDate("04/25")},
	11: {parseExpDate("07/26")},
	12: {parseExpDate("03/25")},
	13: {parseExpDate("09/25")},
	14: {parseExpDate("10/26")},
	15: {parseExpDate("11/26")},
	16: {parseExpDate("08/27")},
	17: {parseExpDate("11/25")},
	18: {parseExpDate("09/27")},
	19: {parseExpDate("02/27")},
	20: {parseExpDate("01/26")},
	21: {parseExpDate("02/28")},
	22: {parseExpDate("09/26")},
	23: {parseExpDate("08/27")},
	24: {parseExpDate("01/26")},
}

func ChoosePaymentMethod(name string) PaymentMethod {
	paymentMethods := map[string][]PaymentMethod{
		"Ava Whitaker":     {CardList[0], DigitalWalletList[0]},
		"Liam Caldwell":    {CardList[1], DigitalWalletList[1]},
		"Isabella Greene":  {CardList[2], DigitalWalletList[2]},
		"Ethan Morrell":    {CardList[3], DigitalWalletList[3]},
		"Maya Ellison":     {CardList[4], DigitalWalletList[4]},
		"Noah Blackwood":   {CardList[5], DigitalWalletList[5]},
		"Chloe Hartman":    {CardList[6], DigitalWalletList[6]},
		"Lucas Pennington": {CardList[7], DigitalWalletList[7]},
		"Sofia Langford":   {CardList[8], DigitalWalletList[8]},
		"Oliver Drayton":   {CardList[9], DigitalWalletList[9]},
		"Harper Linwood":   {CardList[10], DigitalWalletList[10]},
		"Sebastian Knox":   {CardList[11], DigitalWalletList[11]},
		"Amelia Fairbanks": {CardList[12], DigitalWalletList[12]},
		"Julian Royce":     {CardList[13], DigitalWalletList[13]},
		"Nora Halston":     {CardList[14], DigitalWalletList[14]},
		"Elijah Trent":     {CardList[15], DigitalWalletList[15]},
		"Zoe Merrick":      {CardList[16], DigitalWalletList[16]},
		"Caleb Winslow":    {CardList[17], DigitalWalletList[17]},
		"Lily Hargrove":    {CardList[18], DigitalWalletList[18]},
		"Milo Carrington":  {CardList[19], DigitalWalletList[19]},
		"Aria Templeton":   {CardList[20], DigitalWalletList[20]},
		"Declan Shore":     {CardList[21], DigitalWalletList[21]},
		"Vivian Leclair":   {CardList[22], DigitalWalletList[22]},
		"Grayson Holt":     {CardList[23], DigitalWalletList[23]},
		"Clara Redmond":    {CardList[24], DigitalWalletList[24]},
	}
	options := paymentMethods[name]
	choice := rand.Intn(2)
	return options[choice]
}

func NewPayment(name string) Payment {
	method := ChoosePaymentMethod(name)
	return Payment{method, utils.GenerateUuid()}
}
