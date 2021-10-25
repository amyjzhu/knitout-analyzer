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


alternating_pass_h_l = op => {
	pass_h_l(s => {
		if (s%2 == front) {
			op(s);
		}
	})
}

alternating_pass_l_h = op => {
	pass_l_h(s => {
		if (s%2 != front) {
			op(s);
		}
	})
}


function cast_on() {
	alternating_pass_h_l(s => k.tuck("-", "f"+s, carrier));
	alternating_pass_l_h(s => k.tuck("+", "f"+s, carrier));
}


cast_on();

k.releasehook(carrier);

// slightly different plan of attack:
// for i = 0 to …:
// 	if i % 0 == 0 :
// 		# loops on front
// 		knit N rows
// 	else :
// 		# loops on back
// 		purl N rows
// 	“transfer i times” # whole row back and forth (direction depends on i)

let knit_op = () => knit_h_l_l_h(height);
let purl_op = () => {
	for (let i = 0; i < height; i++) {
		pass_h_l(s => k.knit("-", "b"+s, carrier));
		pass_l_h(s => k.knit("+", "b"+s, carrier));
	}
}

let current_op = knit_op;
let alternate_op = purl_op;
let starting_bed = "f"
let alternate_bed = "b"

// oh I am so tempted to make this needlessly recursive
for (iter = 0; iter < max_xfers; iter = iter + 2) {
	// whenever we transfer an odd number, we alternate between knitting and purling
	// whenever we transfer an even number, we don't alternate
	
	if (iter % 2 == 0) {
		current_op();
		// make sure it really starts as we expect
		// do even transfers
		
		// for each xfer, start by xfering over the first bed
		// then alternate 

		let xfer = (pass, next, starting, alternate, remaining) => {
			if (remaining > 0) {
				// now we just do it so that instead of a pass

				pass(s => {
					if (s % 2 == front) {
						// we are already opposite because we xfer'd!
						k.xfer(alternate+s, starting+s);
					} else {
						k.xfer(starting+s, alternate+s)
					}
				});

				xfer(next, pass, alternate, starting, remaining - 1);
			}
		}

		alternating_pass_h_l(s => k.xfer(starting_bed+s, alternate_bed+s));
		xfer(pass_h_l, pass_l_h, starting_bed, alternate_bed, iter);		
		// now we need to move the initial ones back
		alternating_pass_h_l(s => k.xfer(alternate_bed+s, starting_bed+s));

		// keep going since even

		current_op();

		// odd transfers

		alternating_pass_h_l(s => k.xfer(starting_bed+s, alternate_bed+s));
		xfer(pass_h_l, pass_l_h, starting_bed, alternate_bed, iter+1);
		// now we need to move the first side to the correct side
		alternating_pass_h_l(s => k.xfer(starting_bed+s, alternate_bed+s));

		// now switch it all around
		let tmp = current_op;
		current_op = alternate_op;
		alternate_op = tmp;

		tmp = starting_bed;
		starting_bed = alternate_bed;
		alternate_bed = tmp;
	}
}


k.outhook(carrier);

k.write('xfer-test-alternate-every-other.k');