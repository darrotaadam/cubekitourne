use raylib::prelude::*;
use std::io::{self, Write};
use std::time;
mod cubekitourne;


const BGCOLOR:Color = Color::BLACK;
const TXTCOLOR:Color = Color::WHITE;

const SQUARESIZE:i32 = 50;

const GAPFROMBORDER:f32 = 20.0;

fn draw_grid(d: &mut RaylibDrawHandle<'_>, color_a:Color, color_b:Color, zoom_factor:f32){
    let actual_square_size = SQUARESIZE + zoom_factor as i32;
    let width = d.get_screen_width();
    let height = d.get_screen_height();
    for i in 0..width{
        for j in 0..height{
            if i%actual_square_size==0 && j%actual_square_size==0{

               // d.draw_pixel(i, j, color_b);
                d.draw_rectangle(i, j, actual_square_size/2, actual_square_size/2, color_b);
                d.draw_rectangle(i+actual_square_size/2, j+actual_square_size/2, actual_square_size/2, actual_square_size/2, color_a);
            }
        }
    }
}



fn render_grid(rl:&mut RaylibHandle, thread:&mut RaylibThread){

        let mut width: i32=0; 
        let mut height: i32 =0 ;
        let mut font_height: i32=height/10;
        let mut mytext:String = String::from("");
        let mut zoom_factor:f32 = 0.0;
        let mut start_time = time::SystemTime::now();
        let mut nb_iter:u64 = 0;

        
        
        


        while !rl.window_should_close() {
            nb_iter+=1;
            match time::SystemTime::now().duration_since(start_time) {
                Ok(n) => print!("\r[] {} iter/s        (n={})", (nb_iter as f32 / n.as_secs_f32()), nb_iter ),
                Err(_) => panic!("WTF j'ai pas compris"),
            }
            if nb_iter%500 == 0{
                start_time = time::SystemTime::now();
                nb_iter = 0;
            }
            zoom_factor += rl.get_mouse_wheel_move();
            let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(thread);


            width = d.get_screen_width();
            height = d.get_screen_height();
            font_height = height/10;
            
            mytext.clear();
            mytext.push_str(&width.to_string());
            mytext.push('x');
            mytext.push_str(&height.to_string());
            
            

            d.clear_background(Color::BLANK);
            draw_grid(&mut d, 
                Color { r: (20), g: (100), b: (40), a: (0xb0) }, 
                Color { r: (20), g: (40), b: (100), a: (0xb0) },
                zoom_factor
            );

            
            d.draw_text(
                &mytext, 
                width/2 , 
                height-20 - font_height, 
                font_height, 
                TXTCOLOR);
            d.draw_fps(0, height-20);
            
            //d.draw_line(0, 0, width, 0, Color::BLUE);
            //d.draw_line(width, 0, width, height, Color::BLUE);
            //d.draw_line(width, height, 0, height, Color::BLUE);
            //d.draw_line(0, height, 0, 0, Color::BLUE);
            //d.draw_line(0, 0, width, height, Color::BLUE);
            //d.draw_line(width, 0, 0, height, Color::BLUE);
            let corners_gap : [Vector2; 4] = [
                Vector2::new(0.0 + GAPFROMBORDER, 0.0 + GAPFROMBORDER),
                Vector2::new(width as f32 - GAPFROMBORDER, 0.0 + GAPFROMBORDER),
                Vector2::new(width as f32 - GAPFROMBORDER, height as f32 - GAPFROMBORDER),
                Vector2::new(0.0 + GAPFROMBORDER, height as f32 - GAPFROMBORDER)
            ] ;
            

            let middles_gap : [Vector2; 4] = [
                Vector2::new((width/2) as f32 , 0.0 + GAPFROMBORDER), //top
                Vector2::new(width as f32 - GAPFROMBORDER, (height/2) as f32 ),  //right
                Vector2::new((width/2) as f32, height as f32 - GAPFROMBORDER),  //bottom
                Vector2::new(0.0 + GAPFROMBORDER, (height/2) as f32)    //left
            ] ;

            
            d.draw_line_bezier(corners_gap[0], corners_gap[1], 2.0, Color::MAGENTA);
            d.draw_line_bezier(corners_gap[1], corners_gap[2], 2.0, Color::MAGENTA);
            d.draw_line_bezier(corners_gap[2], corners_gap[3], 2.0, Color::MAGENTA);
            d.draw_line_bezier(corners_gap[3], corners_gap[0], 2.0, Color::MAGENTA);

            d.draw_line_ex(middles_gap[0],middles_gap[2],2.0,Color::MAGENTA);
            d.draw_line_ex(middles_gap[1],middles_gap[3],2.0,Color::MAGENTA);
            

            //print!("\rdimensions: {width}x{height}", ); 
            let flush = io::stdout().flush();
            match flush {
                Ok(_) => (),
                Err(_) => println!("AA"),
            }
            
        }
}




fn main() {
    println!("Hello, world!");
    

    let (mut rl, mut thread) = raylib::init()
        .resizable()
        .transparent()
        .size(1000, 800)
        .title("test1")
        .build();

    rl.set_target_fps(165);


    //render_grid(&mut rl, &mut thread);

    cubekitourne::render_3d(&mut rl, &mut thread);


}


