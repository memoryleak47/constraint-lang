# assuming it is not 0^0
let power = fun(x, y) {
	if y == 0 {
		return 1;
	}
	return x*power(x, y-1);
};

print(power(3, 3));
