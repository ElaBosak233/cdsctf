package utils

import "fmt"

const (
	Cyan   = "\033[1;36m"
	BgCyan = "\033[46m"
	Reset  = "\033[0m"
)

func Welcome() {
	asciiArt := `
   ___                             _       
  / _ \  __ _  ___   /\  /\ _   _ | |__   
 / /_)/ / _\ |/ __| / /_/ /| | | || '_ \ 
/ ___/ | (_| | \__ \/ __  / | |_| || |_) |
\/      \__, ||___/\/ /_/   \__,_||_.__/   
        |___/`
	fmt.Println(Cyan + asciiArt + Reset)
}
