extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;
extern crate find_folder;

use opengl_graphics::GlGraphics;
use opengl_graphics::Texture;
use std::f64;
use models::vector::Vector;
use game::Direction;
use models::enemy::Enemy;
use self::rand::Rng;

pub const PLAYER_X_SIZE: f64 = 20.0;
pub const PLAYER_Y_SIZE: f64 = 33.0;
const PLAYER_SPEED: f64 = 5.0;

/// contains mutable settings for the player
/// pos: position in window
/// dir: direction the player is heading
/// health: starts with 3 health
/// texture: image for the player
/// heart_texture: is the image for health hearts
/// is_moving: used when player is moving
/// collided: if the player has collided
/// player_x_size & y_size is the player height and width 
pub struct Player {
    pub pos: Vector,
    pub dir: Direction,    
    pub health: u32,    
    texture: Result<Texture, String>,
    heart_texture: Result<Texture, String>,    
    pub is_moving: bool,
    pub collided: bool,
    pub player_x_size: f64,
    pub player_y_size:f64
}


impl Player {

    /// creates a new player 
    /// is hard coded to find image in assets for player and heart
    /// starts with 3 lives
    pub fn new(x: f64, y: f64 ) -> Self {
        Player {
            pos: Vector::new(x, y),
            dir: Direction::EAST,                                 
            health: 3,
            texture: Texture::from_path(find_folder::Search::ParentsThenKids(3, 3)
                .for_folder("assets")
                .unwrap()
                .join("player.png")),
            heart_texture:   Texture::from_path(find_folder::Search::ParentsThenKids(3, 3)
                .for_folder("assets")
                .unwrap()
                .join("heart.png")),   
            is_moving: false,
            collided: false,
            player_x_size: PLAYER_X_SIZE,
            player_y_size: PLAYER_Y_SIZE,
        }
    }    

    /// resets the player position and health
    pub fn reset(&mut self, x: f64, y: f64) {
        self.pos.x = x;
        self.pos.y = y;
        self.health = 3;
    }   

    /// draws the player moving in the direction it is going
    pub fn draw(&mut self, c: self::graphics::Context, gl: &mut GlGraphics) {
        use self::graphics::*;

        let transform = c.transform
            .trans(self.pos.x, self.pos.y)
            .trans(-PLAYER_X_SIZE / 2.0, -PLAYER_Y_SIZE / 2.0);

        match self.texture {
            Ok(ref t) => image(t, transform, gl),
            _ => {}
        }

        self.handle_moving();
    }

    fn handle_moving(&mut self) {
        if self.is_moving {
            self.texture = Texture::from_path(find_folder::Search::ParentsThenKids(3, 3)
                .for_folder("assets")
                .unwrap()
                .join("player_move.png"));
            match self.dir {
                Direction::WEST => self.pos.x -= PLAYER_SPEED,
                Direction::NORTH => self.pos.y -=PLAYER_SPEED,
                Direction::EAST => self.pos.x += PLAYER_SPEED,
                Direction::SOUTH => self.pos.y += PLAYER_SPEED,
            }
        } else {
            self.texture =Texture::from_path(find_folder::Search::ParentsThenKids(3, 3)
                .for_folder("assets")
                .unwrap()
                .join("player.png"));        
        }
    }

    /// checks for collision with wall
    pub fn collides(&self, wall: &[f64;4]) -> bool {
        let collision_x: bool = self.pos.x + PLAYER_X_SIZE >= wall[0] &&
        wall[0]+(wall[2]-wall[0]) >= self.pos.x;
        let collision_y: bool = self.pos.y + PLAYER_Y_SIZE >= wall[1] &&
            wall[1] + (wall[3]-wall[1]) >= self.pos.y;
        return collision_x && collision_y
    } 

    /// checks for collision with enemy
    pub fn collides_enemy(&self, enemy: &Enemy) -> bool {
        let collision_x: bool = self.pos.x + PLAYER_X_SIZE >= enemy.pos.x &&
        enemy.pos.x+enemy.size >= self.pos.x;
        let collision_y: bool = self.pos.y + PLAYER_Y_SIZE >= enemy.pos.y &&
            enemy.pos.y + enemy.size >= self.pos.y;
        return collision_x && collision_y
    }    

    /// places the enemies randomly in the window
    /// only choose from places where walls do not exist
    pub fn place_random(&mut self, dimensions: [f64;2]){
        let rand_block: u32 = rand::thread_rng().gen_range(1, 4);
        let mut randx: f64;
        let mut randy: f64;

        match rand_block {
            1 => { //left
                randx= rand::thread_rng().gen_range(45.0, dimensions[0]/4.0-20.0);
                randy= rand::thread_rng().gen_range(40.0, dimensions[1]-135.0);
            },
            2 => { //top
                randx= rand::thread_rng().gen_range(dimensions[0]/4.0+50.0, (dimensions[0]/4.0)*3.0);
                randy= rand::thread_rng().gen_range(40.0, dimensions[1]/4.0);
            },
            3 => { //right
                randx= rand::thread_rng().gen_range((dimensions[0]/4.0)*3.0+50.0, dimensions[0]-50.0);
                randy= rand::thread_rng().gen_range(40.0, dimensions[1]-135.0);                
            },
            4 => { //bottom
                randx= rand::thread_rng().gen_range(dimensions[0]/4.0+50.0, (dimensions[0]/4.0)*3.0);
                randy= rand::thread_rng().gen_range((dimensions[1]/4.0)*3.0, dimensions[1]-135.0);                
            },
            _ => {
                randx= rand::thread_rng().gen_range(45.0, dimensions[0]/4.0-20.0);
                randy = rand::thread_rng().gen_range(40.0, dimensions[1]-135.0);            
            }
        }

        //if enemy too close to the player starting try again 
        while randx > 40.0 && randx < 90.0 && randy > dimensions[1] / 2.0 - 50.0 && randy < dimensions[1] / 2.0 + 50.0 {
            randx = rand::thread_rng().gen_range(45.0, dimensions[0]/4.0-10.0); 
            randy = rand::thread_rng().gen_range(40.0, dimensions[1]-135.0); 
        }  
        self.pos.x =  randx;
        self.pos.y = randy;
    } 

    /// draws the hearts on the bottom of the screen showing lives left
    pub fn draw_lives(& self, posx: f64, posy: f64, c: self::graphics::Context, gl: &mut GlGraphics) {
        use self::graphics::*;

        let transform = c.transform
            .trans(posx, posy)
            .trans(-30.0 / 2.0, -30.0 / 2.0);

        match self.heart_texture {
            Ok(ref t) => image(t, transform, gl),
            _ => { println!("something went wrong unwrapping heart texture.");}
        }                
    }
}

#[cfg(test)] 
mod berzerk_test {
    use piston::window::WindowSettings;
    use glutin_window::GlutinWindow as Window;
    use opengl_graphics::OpenGL;
    use super::*;

    #[test]
    fn test_new_player() {
        let opengl = OpenGL::V3_2;
        //OpenGL function pointers must be loaded before creating the `Gl` backend
        //That is why this window is created
           let _window: Window = WindowSettings::new("create test",
                                                     [500, 500])
            .exit_on_esc(true)
            .build()
            .expect("Error creating window"); 
        let _gl = GlGraphics::new(opengl);
        let _t_player = Player::new(0.0,100.0);
    }

    // #[test]
    // fn test_collides_enemy() {
    // }

    // #[test]
    // fn test_place_random() {}

}