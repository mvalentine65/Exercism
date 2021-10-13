package gigasecond
import "time"
func AddGigasecond(t time.Time) time.Time {
    // this is my AddGigasecond
	// there are many like it, but this one is mine
	// it adds 10^9 seconds to a given time
	return t.Add(time.Second * 1000000000)
}
