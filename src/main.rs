use raylib;
mod cubekitourne;





fn main() {
    println!("Hello, world!");
    

    let (mut rl, mut thread) = raylib::init()
        .resizable()
        .transparent()
        .size(1000, 800)
        .title("test1")
        .build();

    rl.set_target_fps(165);


    

    cubekitourne::render_3d(&mut rl, &mut thread);


}


