package lasagna

const OvenTime int = 40; // max time to cook lasagnaa

func RemainingOvenTime(time_in int) int {
		// Given the time a lasagna has spent in the oven,
		// retuns the remaining time needed to cook the lasagna
		return OvenTime-time_in;
}
func PreparationTime(layers int) int {
		// Returns the time to layer the lasagna given the number of layers.
		const per_layer int = 2;
		return layers*per_layer;
}
func ElapsedTime(layers, time_in int) int {
		// Returns the time passed since the start of a lasagna
		return PreparationTime(layers) + time_in;
}
