var expr = 2+3*3-4;

var b = expr * (4 - 2);

var c = 0;

if b >= 8 {				# Nested loops!!
	if b < 15 {
		c = 2*(5-2);
	} else if b < 14 {
		c = 7 + b;
	} else {
		c = b - 2;
	}
} else {
	c = 10;
}

