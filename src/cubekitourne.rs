use raylib::math::Vector3;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;

const POINT_WIDTH:i32 = 10;



pub fn render_3d(rl:&mut RaylibHandle, thread : &mut RaylibThread){
    let mut all_vertex: Vec<Vector3> = Vec::new();
    let mut _x:f32;
    let mut _y:f32;
    let mut __x:f32;
    let mut __y:f32;
    let mut CUBE_DISTANCE:f32 = 0.7;
    let mut ALTITUDE:f32 = -0.2;
    let mut rotated: Vector3;
    let mut angle_y:f32=0.0;
    let mut angle_z:f32=0.0;
    let mut angle_x:f32=0.0;

    let mut camera_position:Vector3= Vector3::new(0.0, 0.0, 0.0);
    let mut camera_direction:Vector3 = Vector3::new(0.0, 0.0, 0.0);
/*
    // CUBE
    all_vertex.push(Vector3::new(0.25, 0.25,0.25 ));
    all_vertex.push(Vector3::new(-0.25, 0.25,0.25 ));
    all_vertex.push(Vector3::new(0.25, -0.25,0.25 ));
    all_vertex.push(Vector3::new(-0.25, -0.25,0.25 ));

    all_vertex.push(Vector3::new(0.25, 0.25,-0.25 ));
    all_vertex.push(Vector3::new(-0.25, 0.25,-0.25 ));
    all_vertex.push(Vector3::new(0.25, -0.25,-0.25 ));
    all_vertex.push(Vector3::new(-0.25, -0.25,-0.25 ));
 */
 
    // DIAMAND
    all_vertex.push(Vector3::new(0.0, 0.20, 0.0));
    
    all_vertex.push(Vector3::new(0.20, 0.20, 0.00));
    all_vertex.push(Vector3::new(0.18, 0.20, 0.08));
    all_vertex.push(Vector3::new(0.14, 0.20, 0.14));
    all_vertex.push(Vector3::new(0.08, 0.20, 0.18));
    all_vertex.push(Vector3::new(0.00, 0.20, 0.20));
    all_vertex.push(Vector3::new(-0.08, 0.20, 0.18));
    all_vertex.push(Vector3::new(-0.14, 0.20, 0.14));
    all_vertex.push(Vector3::new(-0.18, 0.20, 0.08));
    all_vertex.push(Vector3::new(-0.20, 0.20, 0.00));
    all_vertex.push(Vector3::new(-0.18, 0.20, -0.08));
    all_vertex.push(Vector3::new(-0.14, 0.20, -0.14));
    all_vertex.push(Vector3::new(-0.08, 0.20, -0.18));
    all_vertex.push(Vector3::new(0.00, 0.20, -0.20));
    all_vertex.push(Vector3::new(0.08, 0.20, -0.18));
    all_vertex.push(Vector3::new(0.14, 0.20, -0.14));
    all_vertex.push(Vector3::new(0.18, 0.20, -0.08));

    all_vertex.push(Vector3::new(0.35, 0.10, 0.00));
    all_vertex.push(Vector3::new(0.32, 0.10, 0.14));
    all_vertex.push(Vector3::new(0.25, 0.10, 0.25));
    all_vertex.push(Vector3::new(0.14, 0.10, 0.32));
    all_vertex.push(Vector3::new(0.00, 0.10, 0.35));
    all_vertex.push(Vector3::new(-0.14, 0.10, 0.32));
    all_vertex.push(Vector3::new(-0.25, 0.10, 0.25));
    all_vertex.push(Vector3::new(-0.32, 0.10, 0.14));
    all_vertex.push(Vector3::new(-0.35, 0.10, 0.00));
    all_vertex.push(Vector3::new(-0.32, 0.10, -0.14));
    all_vertex.push(Vector3::new(-0.25, 0.10, -0.25));
    all_vertex.push(Vector3::new(-0.14, 0.10, -0.32));
    all_vertex.push(Vector3::new(0.00, 0.10, -0.35));
    all_vertex.push(Vector3::new(0.14, 0.10, -0.32));
    all_vertex.push(Vector3::new(0.25, 0.10, -0.25));
    all_vertex.push(Vector3::new(0.32, 0.10, -0.14));

    all_vertex.push(Vector3::new(0.0, -0.4, 0.0));
    



    
    while !rl.window_should_close(){
        CUBE_DISTANCE += rl.get_mouse_wheel_move()/50.0;
        match rl.get_char_pressed(){
            None => (),
            Some(c) => {
                if c == 'h' {
                    ALTITUDE+=0.005;
                } 
                else if c == 'b' {
                    ALTITUDE -=0.005
                }
            },
        }
        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(thread);
        d.clear_background(Color::BLANK);

        angle_z += 0.005;
        angle_y += 0.005;
        angle_x += 0.005;
        //affichage
        for v in &all_vertex {
            
            
            rotated = rotate_z(&v,angle_z);
            rotated = rotate_y(&rotated,angle_y);
            rotated = rotate_x(&rotated,angle_x);

            let translated:Vector3 = Vector3::new(rotated.x, rotated.y + ALTITUDE, rotated.z + CUBE_DISTANCE);
            (_x, _y) = to2d(&translated);


            let coords_screen: (i32, i32) = ortho_to_screen(_x, _y, &mut d);
            //d.draw_rectangle(coords_screen.0, coords_screen.1, POINT_WIDTH, POINT_WIDTH, Color::RED);
            let mut other_rotated:Vector3;
            for other in &all_vertex {
                other_rotated = rotate_z(&other,angle_z);
                other_rotated = rotate_y(&other_rotated,angle_y);
                other_rotated = rotate_x(&other_rotated,angle_x);
                let other_translated:Vector3 = Vector3::new(other_rotated.x, other_rotated.y + ALTITUDE, other_rotated.z + CUBE_DISTANCE);
                let (__x, __y) = to2d(&other_translated);
                let other_coords = ortho_to_screen(__x, __y, &mut d);
                d.draw_line(
                    coords_screen.0,
                    coords_screen.1,
                    other_coords.0,
                    other_coords.1,
                    Color::ORANGE
                );
            }
        }



        thread::sleep(Duration::from_secs_f32(0.001));
    }

}



fn ortho_to_screen(x:f32, y:f32, d: & RaylibDrawHandle<'_>)-> (i32, i32){
    let WIDTH:f32 = d.get_screen_width() as f32;
    let HEIGHT:f32 = d.get_screen_height() as f32;
    
    let scr_x: f32 = (WIDTH/2.0) + x * (WIDTH/2.0);
	let scr_y: f32 = (HEIGHT/2.0) + y * (HEIGHT/2.0) * -1.0;
    
    //println!("   from    ({},{})",x, y);
    //println!("  to      ({},{})", scr_x, scr_y);
	return (scr_x as i32, scr_y as i32);
}



fn rotate_y(point:&Vector3, angle:f32)->Vector3{
	// tourne autour de l'axe y_
	let cos = angle.cos();
	let sin = angle.sin();

	Vector3::new(
		point.x*cos - point.z*sin,
		point.y ,
		point.x*sin + point.z*cos
    )
}



fn rotate_z(point:&Vector3, angle:f32)->Vector3{
	// tourne autour de l'axe y_
	let cos = angle.cos();
	let sin = angle.sin();

	Vector3::new(
		point.x*cos - point.y*sin,
		point.x*sin + point.y*cos,
		point.z
    )
}


fn rotate_x(point:&Vector3, angle:f32)->Vector3{
	// tourne autour de l'axe y_
	let cos = angle.cos();
	let sin = angle.sin();

	Vector3::new(
		point.x,
		point.y*cos - point.z*sin,
		point.y*sin + point.z *cos
    )
}



fn to2d(point:&Vector3) -> (f32, f32){
    return (point.x / point.z , point.y/point.z);
}