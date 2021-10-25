// import the knitoutWriter code and instantiate it as an object
var knitout = require('knitout');
k = new knitout.Writer({carriers:["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]});

// add some headers relevant to this job
k.addHeader('Machine','SWG091N2');
k.addHeader('Gauge','5');
k.addHeader('Width', '250');
k.addHeader('Position', 'Center');

// swatch variables
let height = 10; // 10 rows
// var width = 40; 
var width = 20; 
var carrier = "2";

// bring in carrier using yarn inserting hook
k.inhook(carrier);

var front = width%2;

let max_xfers = 10;
let iter = 1;

let pass_h_l = op => {
	for (var s=width; s > 0; s--) {
		op(s);
	}
}

let pass_l_h = op => {
	for (var s=1; s<=width; s++) {
		op(s);
	}
}

let knit_h_l_l_h = rows => {
	for (i = 0; i < rows; i++) {
		pass_h_l(s => k.knit("-", "f"+s, carrier));
		pass_l_h(s => k.knit("+", "f"+s, carrier));
	}
}


pass_h_l(s => {
	if (s%2 == front) {
		k.tuck("-", "f"+s, carrier);
	}
})

pass_l_h(s => {
	if (s%2 != front) {
		k.tuck("+", "f"+s, carrier);
	}
})

k.releasehook(carrier);

for (iter = 1; iter < max_xfers; iter = iter + 2) {
	
	knit_h_l_l_h(height);

// xfers
	for (let num_xfers = 0; num_xfers < iter; num_xfers++) {
	// let's do one round of transfers and back
		pass_h_l(s => k.xfer("f"+s, "b"+s));

		pass_h_l(s => k.knit("-", "f"+s, carrier));

		pass_l_h(s => k.xfer("b"+s, "f"+s));

		pass_l_h(s => k.knit("+", "f"+s, carrier));
	}

// knit more

	knit_h_l_l_h(height);

	// purls 
	// transfer and then knit on the back
	// transfers don't change carrier direction
	pass_h_l(s => k.xfer("f"+s, "b"+s));

	pass_h_l(s => k.knit("-", "b"+s, carrier));
	pass_l_h(s => k.knit("+", "b"+s, carrier));

	pass_l_h(s => k.xfer("b"+s, "f"+s));
}

k.outhook(carrier);

k.write('xfer-test-purl.k');