use raylib::math::Vector3;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;
/*use std::fs::File;
use std::io::BufReader;
use std::path::Path;
*/
//use serde_json::{Value,Result};
//use serde::Deserialize;

const POINT_WIDTH:i32 = 5;

/*
#[derive(Deserialize)]
struct JsonVector3{
    x: f32,
    y: f32,
    z: f32
}

#[derive(Deserialize)]
struct JsonShape{
    center: JsonVector3,
    points: Vec<JsonVector3>
}
*/


struct Camera3d{
    position:Vector3,
    direction:Vector3,
    fov : f32
}

impl Camera3d{
    fn new()->Camera3d{
        Camera3d { 
            position: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 0.0),
            fov : 90.0 
        }
    }
}


struct Shape3d {
    points : Vec<Vector3>,
    center : Vector3,
    angle_x : f32,
    angle_y : f32,
    angle_z : f32
}

impl Shape3d{
    fn new(x:f32, y:f32, z:f32)->Shape3d{
        Shape3d { 
            points: Vec::new(), 
            center: Vector3::new(x, y, z) ,
            angle_x : 0.0,
            angle_y : 0.0,
            angle_z : 0.0
        }
    }



    fn rotate_y(&self, point:&Vector3, angle:f32)->Vector3{
        let cos = angle.cos();
        let sin = angle.sin();
        Vector3::new(
            point.x*cos - point.z*sin,
            point.y ,
            point.x*sin + point.z*cos
        )
    }
    fn rotate_z(&self, point:&Vector3, angle:f32)->Vector3{
        // tourne autour de l'axe y_
        let cos = angle.cos();
        let sin = angle.sin();
        Vector3::new(
            point.x*cos - point.y*sin,
            point.x*sin + point.y*cos,
            point.z
        )
    }
    fn rotate_x(&self, point:&Vector3, angle:f32)->Vector3{
        let cos = angle.cos();
        let sin = angle.sin();
        Vector3::new(
            point.x,
            point.y*cos - point.z*sin,
            point.y*sin + point.z *cos
        )
    }




    fn show(&mut self, d: &mut RaylibDrawHandle<'_>, camera: &mut Camera3d){
        let mut object_rotated: Vector3;
        let mut object_translated: Vector3;
        let mut camera_rotated: Vector3;
        let mut camera_translated: Vector3;
        
        let mut other_object_rotated:Vector3;
        let mut other_object_translated:Vector3;
        let mut other_camera_rotated:Vector3;
        let mut other_camera_translated:Vector3;
        let mut _x:f32;
        let mut _y:f32;

        const CENTER_AXIS_LENGTH:f32 = 1.0;

        let mut fov_text: String = String::new();
        
        //self.angle_x += 0.005;
        //self.angle_y += 0.005;
        //self.angle_z += 0.005
        ;

        //affichage
        for v in &self.points {
            // rotate selon l'angle de la Shape3d
            object_rotated = self.rotate_z(&v,self.angle_z);
            object_rotated = self.rotate_y(&object_rotated,self.angle_y);
            object_rotated = self.rotate_x(&object_rotated,self.angle_x);
            
            object_translated = Vector3::new(object_rotated.x + self.center.x , object_rotated.y + self.center.y , object_rotated.z + self.center.z);


            // rotate selon l'angle de la caméra (soustrait)
            camera_rotated = self.rotate_z(&object_translated,-camera.direction.z);
            camera_rotated = self.rotate_y(&camera_rotated,-camera.direction.y);
            camera_rotated = self.rotate_x(&camera_rotated,-camera.direction.x);

            
            
            camera_translated = Vector3::new(camera_rotated.x - camera.position.x, camera_rotated.y - camera.position.y, camera_rotated.z - camera.position.z);
            (_x, _y) = to2d(&camera_translated, &camera);


            let coords_screen: (i32, i32) = ortho_to_screen(_x, _y, d);
            //d.draw_rectangle(coords_screen.0 - POINT_WIDTH/2 , coords_screen.1 - POINT_WIDTH/2, POINT_WIDTH, POINT_WIDTH, Color::RED);
            
            for other in &self.points {
                other_object_rotated = self.rotate_z(&other,self.angle_z);
                other_object_rotated = self.rotate_y(&other_object_rotated,self.angle_y);
                other_object_rotated = self.rotate_x(&other_object_rotated,self.angle_x);

                other_object_translated = Vector3::new(other_object_rotated.x + self.center.x , other_object_rotated.y + self.center.y , other_object_rotated.z + self.center.z);

                other_camera_rotated = self.rotate_z(&other_object_translated,-camera.direction.z);
                other_camera_rotated = self.rotate_y(&other_camera_rotated,-camera.direction.y);
                other_camera_rotated = self.rotate_x(&other_camera_rotated,-camera.direction.x);

                
                other_camera_translated = Vector3::new(other_camera_rotated.x - camera.position.x, other_camera_rotated.y - camera.position.y, other_camera_rotated.z - camera.position.z);
                let (__x, __y) = to2d(&other_camera_translated, &camera);
                let other_coords = ortho_to_screen(__x, __y, d);
                d.draw_line(
                    coords_screen.0,
                    coords_screen.1,
                    other_coords.0,
                    other_coords.1,
                    Color::ORANGE
                );
            }
        }


        //affichage du centre (0,0,0)
        

        let mut x_start:Vector3 = Vector3::new(-CENTER_AXIS_LENGTH - camera.position.x, 0.0 - camera.position.y, 0.0 - camera.position.z);
        let mut x_end:Vector3 = Vector3::new( CENTER_AXIS_LENGTH - camera.position.x, 0.0 - camera.position.y, 0.0 - camera.position.z);
        x_start = self.rotate_x(&x_start, -camera.direction.x);
        x_start = self.rotate_y(&x_start, -camera.direction.y);
        x_start = self.rotate_z(&x_start, -camera.direction.z);

        x_end = self.rotate_x(&x_end, -camera.direction.x);
        x_end = self.rotate_y(&x_end, -camera.direction.y);
        x_end = self.rotate_z(&x_end, -camera.direction.z);

        let (_x_start_x, _x_start_y) = to2d(&x_start, &camera);
        let (_x_end_x, _x_end_y) = to2d(&x_end, &camera);

        let x_start_coords = ortho_to_screen(_x_start_x, _x_start_y, d);
        let x_end_coords = ortho_to_screen(_x_end_x, _x_end_y, d);

        d.draw_line(
                    x_start_coords.0 ,
                    x_start_coords.1,
                    x_end_coords.0,
                    x_end_coords.1,
                    Color::RED
                );



        let mut y_start:Vector3 = Vector3::new(0.0 - camera.position.x, -CENTER_AXIS_LENGTH - camera.position.y, 0.0 - camera.position.z);
        let mut y_end:Vector3 = Vector3::new( 0.0 - camera.position.x, CENTER_AXIS_LENGTH - camera.position.y, 0.0 - camera.position.z);
        y_start = self.rotate_x(&y_start, -camera.direction.x);
        y_start = self.rotate_y(&y_start, -camera.direction.y);
        y_start = self.rotate_z(&y_start, -camera.direction.z);

        y_end = self.rotate_x(&y_end, -camera.direction.x);
        y_end = self.rotate_y(&y_end, -camera.direction.y);
        y_end = self.rotate_z(&y_end, -camera.direction.z);

        let (_y_start_x, _y_start_y) = to2d(&y_start, &camera);
        let (_y_end_x, _y_end_y) = to2d(&y_end, &camera);

        let y_start_coords = ortho_to_screen(_y_start_x, _y_start_y, d);
        let y_end_coords = ortho_to_screen(_y_end_x, _y_end_y, d);

        d.draw_line(
                    y_start_coords.0 ,
                    y_start_coords.1,
                    y_end_coords.0,
                    y_end_coords.1,
                    Color::BLUE
                );


        let mut z_start:Vector3 = Vector3::new(0.0 - camera.position.x, 0.0 - camera.position.y, -CENTER_AXIS_LENGTH - camera.position.z);
        let mut z_end:Vector3 = Vector3::new( 0.0 - camera.position.x, 0.0 - camera.position.y, CENTER_AXIS_LENGTH - camera.position.z);
        z_start = self.rotate_x(&z_start, -camera.direction.x);
        z_start = self.rotate_y(&z_start, -camera.direction.y);
        z_start = self.rotate_z(&z_start, -camera.direction.z);

        z_end = self.rotate_x(&z_end, -camera.direction.x);
        z_end = self.rotate_y(&z_end, -camera.direction.y);
        z_end = self.rotate_z(&z_end, -camera.direction.z);

        let (_z_start_x, _z_start_y) = to2d(&z_start, &camera);
        let (_z_end_x, _z_end_y) = to2d(&z_end, &camera);

        let z_start_coords = ortho_to_screen(_z_start_x, _z_start_y, d);
        let z_end_coords = ortho_to_screen(_z_end_x, _z_end_y, d);

        d.draw_line(
                    z_start_coords.0 ,
                    z_start_coords.1,
                    z_end_coords.0,
                    z_end_coords.1,
                    Color::GREEN
                );

        


        fov_text.clear();
        fov_text.push_str("fov: ");
        fov_text.push_str(&camera.fov.to_string());
        fov_text.push('°');
        d.draw_text(&fov_text, d.get_screen_width()-50, 10, 10, Color::DARKGRAY);
    }

}


fn create_scene()->Vec<Shape3d>{
    let mut shapes: Vec<Shape3d> = Vec::new();
    

    // CUBE
    let mut cube:Shape3d = Shape3d::new(0.25, 0.0, 0.25);

    cube.points.push(Vector3::new(0.25, 0.25,0.25 ));   // haut droit fond
    cube.points.push(Vector3::new(-0.25, 0.25,0.25 ));  // haut fauche fond
    cube.points.push(Vector3::new(0.25, -0.25,0.25 ));  // bas droit fond
    cube.points.push(Vector3::new(-0.25, -0.25,0.25 )); // bas gauche fond 

    cube.points.push(Vector3::new(0.25, 0.25,-0.25 ));
    cube.points.push(Vector3::new(-0.25, 0.25,-0.25 ));
    cube.points.push(Vector3::new(0.25, -0.25,-0.25 ));
    cube.points.push(Vector3::new(-0.25, -0.25,-0.25 ));

    
 

    // DIAMAND
    let mut diamand:Shape3d = Shape3d::new(-0.25, 0.0, -0.25);
 
    diamand.points.push(Vector3::new(0.0, 0.20, 0.0));
    
    diamand.points.push(Vector3::new(0.20, 0.20, 0.00));
    diamand.points.push(Vector3::new(0.18, 0.20, 0.08));
    diamand.points.push(Vector3::new(0.14, 0.20, 0.14));
    diamand.points.push(Vector3::new(0.08, 0.20, 0.18));
    diamand.points.push(Vector3::new(0.00, 0.20, 0.20));
    diamand.points.push(Vector3::new(-0.08, 0.20, 0.18));
    diamand.points.push(Vector3::new(-0.14, 0.20, 0.14));
    diamand.points.push(Vector3::new(-0.18, 0.20, 0.08));
    diamand.points.push(Vector3::new(-0.20, 0.20, 0.00));
    diamand.points.push(Vector3::new(-0.18, 0.20, -0.08));
    diamand.points.push(Vector3::new(-0.14, 0.20, -0.14));
    diamand.points.push(Vector3::new(-0.08, 0.20, -0.18));
    diamand.points.push(Vector3::new(0.00, 0.20, -0.20));
    diamand.points.push(Vector3::new(0.08, 0.20, -0.18));
    diamand.points.push(Vector3::new(0.14, 0.20, -0.14));
    diamand.points.push(Vector3::new(0.18, 0.20, -0.08));

    diamand.points.push(Vector3::new(0.35, 0.10, 0.00));
    diamand.points.push(Vector3::new(0.32, 0.10, 0.14));
    diamand.points.push(Vector3::new(0.25, 0.10, 0.25));
    diamand.points.push(Vector3::new(0.14, 0.10, 0.32));
    diamand.points.push(Vector3::new(0.00, 0.10, 0.35));
    diamand.points.push(Vector3::new(-0.14, 0.10, 0.32));
    diamand.points.push(Vector3::new(-0.25, 0.10, 0.25));
    diamand.points.push(Vector3::new(-0.32, 0.10, 0.14));
    diamand.points.push(Vector3::new(-0.35, 0.10, 0.00));
    diamand.points.push(Vector3::new(-0.32, 0.10, -0.14));
    diamand.points.push(Vector3::new(-0.25, 0.10, -0.25));
    diamand.points.push(Vector3::new(-0.14, 0.10, -0.32));
    diamand.points.push(Vector3::new(0.00, 0.10, -0.35));
    diamand.points.push(Vector3::new(0.14, 0.10, -0.32));
    diamand.points.push(Vector3::new(0.25, 0.10, -0.25));
    diamand.points.push(Vector3::new(0.32, 0.10, -0.14));

    diamand.points.push(Vector3::new(0.0, -0.4, 0.0));
    

    shapes.push(diamand);
    shapes.push(cube);
    shapes
}



pub fn render_3d(rl:&mut RaylibHandle, thread : &mut RaylibThread){
   // let mut zoom_factor: f32 = 0.0;
    

    let mut camera:Camera3d = Camera3d::new();
    camera.position.z = -1.0;

    /*  
   // let mut mouse_movement:raylib::ffi::Vector2;
   // let mut all_vertex: Vec<Vector3> = Vec::new();


    //let file = File::open(path);
    //let reader = BufReader::new(file);
    //SHAPES:Value = serde_json::from_reader(rdr)
    
    let json = r#"
    [
        {
            "center" : {
                "x" : 0.25,
                "y" : 0.25,
                "z" : 1.0
            },
            "points" : [
                {"x" : 0.25, "y": 0.25, "z" : 0.25},
                {"x" : -0.25, "y": 0.25, "z" :0.25},
                {"x" : 0.25, "y": -0.25, "z" :0.25},
                {"x" : -0.25, "y": -0.25, "z" :0.25},
                {"x" : 0.25, "y": 0.25, "z" :-0.25},
                {"x" : -0.25, "y": 0.25, "z" :-0.25},
                {"x" : 0.25, "y": -0.25, "z" :-0.25},
                {"x" : -0.25, "y": -0.25, "z" :-0.25}
            ]
        }
    ]"#;


    let shapes: Vec<JsonShape> = serde_json::from_str(json).unwrap();
 
    */
    
    let mut shapes: Vec<Shape3d> = create_scene();

    
    while !rl.window_should_close(){
        
        camera.fov += rl.get_mouse_wheel_move(); 
        camera.fov = camera.fov.clamp(30.0, 150.0);

        if rl.is_key_down(KeyboardKey::KEY_W){
            if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT){
                camera.position.z +=0.01;
            }
            else{
                camera.position.y +=0.01;
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_S){
            if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT){
                camera.position.z -=0.01;
            }
            else{
                camera.position.y -=0.01;
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_A){
            camera.position.x -=0.01
        }
        if rl.is_key_down(KeyboardKey::KEY_D){
            camera.position.x +=0.01
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT){
            if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT){
                camera.direction.z +=0.01
            }else{
                camera.direction.y +=0.01
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT){
            if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT){
                camera.direction.z -=0.01
            }else{
                camera.direction.y -=0.01
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_UP){
            camera.direction.x +=0.01
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN){
            camera.direction.x -=0.01
        }
 

      


        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(thread);
        d.clear_background(Color::BLANK);
        
        
        for shape in &mut shapes{
            //shape.center.z += zoom_factor;
            //shape.center.y += altitude;
            //shape.center.x += gauchedroite;
            
            shape.show(&mut d, &mut camera);
        }



        thread::sleep(Duration::from_secs_f32(0.01));
    }

}



fn ortho_to_screen(x:f32, y:f32, d: & RaylibDrawHandle<'_>)-> (i32, i32){
    let width:f32 = d.get_screen_width() as f32;
    let height:f32 = d.get_screen_height() as f32;
    
    let scr_x: f32 = (width/2.0) + x * (width/2.0);
	let scr_y: f32 = (height/2.0) + y * (height/2.0) * -1.0;
    
    //println!("   from    ({},{})",x, y);
    //println!("  to      ({},{})", scr_x, scr_y);
	return (scr_x as i32, scr_y as i32);
}






fn to2d(point:&Vector3, camera:& Camera3d) -> (f32, f32){
    let half_fov:f32 = camera.fov.to_radians()/2.0;

    return (
        point.x / ( point.z * half_fov.tan() ) ,
        point.y/ ( point.z * half_fov.tan() )
    );
}