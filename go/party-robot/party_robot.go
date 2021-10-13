package partyrobot

import (
		"fmt"
)

// Welcome greets a person by name.
func Welcome(name string) string {
		return fmt.Sprintf("Welcome to my party, %s!", name);
}

// HappyBirthday wishes happy birthday to the birthday person and stands out their age.
func HappyBirthday(name string, age int) string {
		return fmt.Sprintf("Happy birthday %s! You are now %d years old!" ,name, age);
}
// "Welcome to my party, Christiane!\nYou have been assigned to table 1B. Your table is on the left, exactly 23.8 meters from here.\nYou wi    ll be sitting next to Frank.
// AssignTable assigns a table to each guest.
func AssignTable(name string, table int, neighbour, direction string, distance float64) string {
		line1 := fmt.Sprintf("Welcome to my party, %s!\n", name);
		line2 := fmt.Sprintf("You have been assigned to table %X. ",table);
		line2_2 := fmt.Sprintf("Your table is %s, exactly %.1f meters from here.\n",direction,distance);
		line3 := fmt.Sprintf("You will be sitting next to %s.", neighbour);
		return line1 + line2 + line2_2 + line3;
}
