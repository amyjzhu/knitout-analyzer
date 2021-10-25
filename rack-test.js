// import the knitoutWriter code and instantiate it as an object
var knitout = require('knitout');
k = new knitout.Writer({carriers:["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]});

// add some headers relevant to this job
k.addHeader('Machine','SWG091N2');
k.addHeader('Gauge','15');

// swatch variables
let height = 10; // 10 rows
var width = 41; 
var carrier = "2";

// bring in carrier using yarn inserting hook
k.inhook(carrier);

var front = width%2;

let max_racks = 25;
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

for (iter = 1; iter < max_racks; iter = iter + 2) {
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
for (let num_racks = 0; num_racks < iter; num_racks++) {
	// let's do one round of transfers and back
	k.rack(num_racks);
	k.rack(num_racks * -1);
	k.rack(0);
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

// purl row (do you transfer and do back ?)
for (var s=width; s>0; s--) {
	k.xfer("f"+s, "b"+s);
}

for (let i = 0; i < 2; i++) {
	for (var s=1; s<=width; s++) {
		k.knit("+", "b"+s, carrier);
	}
    
}

for (var s=1; s<=width; s++) {
	k.xfer("b"+s, "f"+s);
}
}

k.outhook(carrier);

k.write('rack-test.k');