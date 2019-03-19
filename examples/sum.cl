let add_one = fun(x) { return x + 1; };
let sub_one = fun(x) { return x - 1; };

let add = fun(x, y) {
	if x == 0 {
		return y;
	}
	return add(sub_one(x), add_one(y));
};

print(add(20, 34));
