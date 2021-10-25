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


// tuck on alternate needles to cast on
for (var s=width; s>0; s--) {
	if (s%2 == front) {
		k.tuck("-", "f"+s, carrier);
	}
	else {
		//k.miss("-", "f"+s, carrier);
	}
}
for (var s=1; s<=width; s++) {
	if (s%2 != front) {
		k.tuck("+", "f"+s, carrier);
	}
	else {
		//k.miss("+", "f"+s, carrier);
	}
}

k.releasehook(carrier);

// knit
// TODO: double-check procedure

// let's do the test part

for (iter = 1; iter < max_xfers; iter = iter + 2) {
// initial knit
for (var h=0; h<height; h++) {
	for (var s=width; s>0; s--) {
        k.knit("-", "f"+s, carrier);
	}
    
	for (var s=1; s<=width; s++) {
		k.knit("+", "f"+s, carrier);
	}
}

// do a bunch of transfers
for (let num_xfers = 0; num_xfers < iter; num_xfers++) {
	// let's do one round of transfers and back
	for (var s=width; s>0; s--) {
        k.xfer("f"+s, "b"+s);
	}
	
	// just do a knit in between
	for (var s=width; s>0; s--) {
        k.knit("-", "f"+s, carrier);
	}

	for (var s=1; s<=width; s++) {
		k.xfer("b"+s, "f"+s);
	}

	for (var s=1; s<=width; s++) {
		k.knit("+", "f"+s, carrier);
	}
}

// knit more
for (var h=0; h<height; h++) {
	for (var s=width; s>0; s--) {
        k.knit("-", "f"+s, carrier);
	}
    
	for (var s=1; s<=width; s++) {
		k.knit("+", "f"+s, carrier);
	}
}
// TODO: trying this to fix the purls (purls normally just drop and die)
for (var s=width; s>0; s--) {
	k.knit("-", "f"+s, carrier);
}

// purl row (do you transfer and do back ?)
for (var s=width; s>0; s--) {
	k.xfer("f"+s, "b"+s);
}

// two purls (not working somehow)
for (var s=1; s<=width; s++) {
		k.knit("+", "b"+s, carrier);    
}

for (var s=width; s>0; s--) {
	k.knit("-", "b"+s, carrier);
}

for (var s=1; s<=width; s++) {
	k.xfer("b"+s, "f"+s);
}

// cushioning knit for direction
for (var s=1; s<=width; s++) {
	k.knit("+", "f"+s, carrier);
}

}

// knit to finish
for (var h=0; h<height; h++) {
	for (var s=width; s>0; s--) {
        k.knit("-", "f"+s, carrier);
	}

	for (var s=1; s<=width; s++) {
		k.knit("+", "f"+s, carrier);
	}
}


k.outhook(carrier);

k.write('xfer-test-w-knit.k');