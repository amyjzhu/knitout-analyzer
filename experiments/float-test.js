// import the knitoutWriter code and instantiate it as an object
var knitout = require('knitout');
k = new knitout.Writer({carriers:["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]});

// add some headers relevant to this job
k.addHeader('Machine','SWG091N2');
k.addHeader('Gauge','5');
k.addHeader('Width', '250');
k.addHeader('Position', 'Center');

// swatch variables
let height = 40; // 10 rows
// var width = 40; 
var width = 20; 
var carrier = "2";

// bring in carrier using yarn inserting hook
k.inhook(carrier);


var front = width%2;

let float_width = 4
let sts = [1, 2, 3, 4, 5, 6, 7, 8, 13, 14, 15, 16, 17, 18, 19, 20]


// tuck on alternate needles to cast on
for (var s of sts.reverse()) {
	if (s%2 == front) {
		k.tuck("-", "f"+s, carrier);
	}
	else {
		//k.miss("-", "f"+s, carrier);
	}
}
for (var s of sts.reverse()) {
	if (s%2 != front) {
		k.tuck("+", "f"+s, carrier);
	}
	else {
		//k.miss("+", "f"+s, carrier);
	}
}

k.releasehook(carrier);


// initial knit
for (var h=0; h<height; h++) {
	for (var s of sts.reverse()) {
        k.knit("-", "f"+s, carrier);
	}
    
	for (var s of sts.reverse()) {
		k.knit("+", "f"+s, carrier);
	}
}


k.outhook(carrier);

k.write('float-test.k');