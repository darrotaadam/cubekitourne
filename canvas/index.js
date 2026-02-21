console.log(area);

const WIDTH = 1000;
const HEIGHT = 1000;
const FPS = 60;
const RECTWIDTH = 20;
const BGCOLOR = "#222222FF";
const DURATION = 10; 
const MAXDIST = 1000;


area.width = WIDTH;
area.height = HEIGHT;
area.style.backgroundColor = BGCOLOR;
ctx = area.getContext("2d");
const COLOR = "#00AAFFFF";
ctx.fillStyle = COLOR;
ctx.strokeStyle = COLOR;
points = [
	{ x: 0.5, y: 0.5, z: 3},
	{ x: 0.5, y: -0.5, z: 3},
	{x: -0.5, y: 0.5, z:3},
	{x: -0.5, y: -0.5, z:3},
	{ x: 0.5, y: 0.5, z: 2},
	{ x: 0.5, y: -0.5, z: 2},
	{x: -0.5, y: 0.5, z:  2},
	{x: -0.5, y: -0.5, z: 2}
]



function xy(point){
	const x_ = point.x / point.z;
	const y_ = point.y / point.z;
	return {x: x_, y: y_};
}
	

function wh(xy){
	const w = (WIDTH/2) + xy.x * (WIDTH/2);
	const h = (HEIGHT/2) + xy.y * (HEIGHT/2) * -1;
	return {w: w, h:h};
}

function clear(){
	ctx.fillStyle = BGCOLOR;
	ctx.fillRect(0, 0, WIDTH, HEIGHT);
	ctx.fillStyle = COLOR;
}


function dl(s, d){
	const cs = wh(xy(s));
	const cd = wh(xy(d));
	ctx.beginPath();
	ctx.moveTo(cs.w, cs.h);
	ctx.lineTo(cd.w, cd.h);
	ctx.closePath();
	ctx.stroke();
}


function rt(p, a){
	// tourne autour de l'axe y_
	const cos = Math.cos(a);
	const sin = Math.sin(a);

	return {
		x : p.x*cos - p.z*sin,
		y : p.y ,
		z : p.x*sin + p.z*cos
	}
	
}

function z(p, v){
	p.z = v;
}


let c2d;
let c;
let dt = 1/FPS ;
let a = 0;
let ea;





function showPoints(lim){

	clear();
	points.forEach((e)=>{
		//e.z += e.z*dt;
		z(e,10)
		a = 2 * Math.PI * dt / 5;
		ea = rt(e, a);
		e.x = ea.x;
		e.z = ea.z;

		c = wh(xy(e));
		console.log(` width : ${c.w} , height : ${c.h} `);
		console.log(` z : ${e.z} `);
		ctx.fillRect(c.w - RECTWIDTH/2, c.h - RECTWIDTH/2, RECTWIDTH, RECTWIDTH);
		
	});
	/*	
	points.forEach((s)=>{
		points.forEach((d)=>{
			dl(s,d);
		});	
	});
	*/


	if(lim<MAXDIST){
		setTimeout(showPoints, 1000/FPS, lim+1);
	}
}

setTimeout(showPoints, 1000/FPS, 0);
/*
let z = 1;

let i = 0;
while (i<100){
	z ++;
	z = z%50;
	setTimeout(showPoints, 1000/FPS, z);
	i++;
	console.log("z: ",z);
}
*/
