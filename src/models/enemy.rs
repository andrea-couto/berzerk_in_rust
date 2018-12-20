extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;
extern crate find_folder;

use opengl_graphics::{GlGraphics, Texture};
use std::f64;
use models::vector::Vector;
use self::rand::Rng;
use game::Direction; //where is player in relation to enemy shoot in that direction

pub const ENEMY_SIZE: f64 = 40.0;
const ENEMY_SPEED: f64 = 5.0;
const ENEMY_PROB_MOVEMENT: u32= 30;


/// struct contains mutable settings for enemies
/// pos: position in window
/// alive: whether the enemy should be removed or not
/// texture: contains the image for enemy
/// size: size of enemy
/// dir: direction the enemy is moving towards
pub struct Enemy {
    pub pos: Vector,
    pub alive: bool,    
    texture: Result<Texture, String>,
    pub size: f64,
    pub dir: Direction,
}


impl Enemy {
    /// creates a new enemy 
    pub fn new(x: f64, y: f64 ) -> Self {
        Enemy {
            pos: Vector::new(x, y),
            alive: true,
            texture: Texture::from_path(find_folder::Search::ParentsThenKids(3, 3)
                .for_folder("assets")
                .unwrap()
                .join("enemy.png")),
            size: ENEMY_SIZE,
            dir: Direction::EAST,
        }
    }        

    ///draws the enemy on the screen
    pub fn draw(&mut self, c: self::graphics::Context, gl: &mut GlGraphics) {
        use self::graphics::*;

        let transform = c.transform
            .trans(self.pos.x, self.pos.y)
            .trans(-ENEMY_SIZE / 2.0, -ENEMY_SIZE / 2.0);

        match self.texture {
            Ok(ref t) => image(t, transform, gl),
            _ => {}
        }
    }

    /// randomly picks whether the enemy should move toward the player
    pub fn update(&mut self, playerx:f64,playery:f64) {
        let num: u32 = rand::thread_rng().gen_range(1, ENEMY_PROB_MOVEMENT);

        if num == 3 {
            self.move_toward_player(playerx,playery);            
        }
    }

    fn set_direction(&mut self, dx: f64, dy:f64){
        if dx*dx > dy*dy {
            if dx >  0.0 {
                self.dir = Direction::WEST;
            } else {
                self.dir = Direction::EAST;
            }
        } else {
            if dy > 0.0 {
                self.dir = Direction::NORTH;
            } else {
                self.dir = Direction::SOUTH;
            }
        }        
    }

    /// handles the movement toward the player
    fn move_toward_player(&mut self, playerx:f64,playery:f64) {
        let mut dx = self.pos.x - playerx;
        let mut dy = self.pos.y - playery;
        let dist =  (dx*dx + dy*dy).sqrt();
        dx = dx/dist;
        dy = dy/dist;

        self.set_direction(dx,dy);

        // if dy > 0.0 {println!("going up");}
        self.pos.x -= dx * ENEMY_SPEED;
        self.pos.y -= dy * ENEMY_SPEED;
    } 

    ///checks for enemy collision with wall
    pub fn collides(&self, wall: &[f64;4]) -> bool {
        let collision_x: bool = self.pos.x + ENEMY_SIZE >= wall[0] &&
        wall[0]+(wall[2]-wall[0]) >= self.pos.x;

        let collision_y: bool = self.pos.y + ENEMY_SIZE >= wall[1] &&
            wall[1] + (wall[3]-wall[1]) >= self.pos.y;
            
        return collision_x && collision_y
    }        
}




#[cfg(test)]
mod berzerk_test {
    use piston::window::WindowSettings;
    use glutin_window::GlutinWindow as Window;
    use opengl_graphics::OpenGL;
    use super::*;

    //TODO: Find a way to make a test enemy that I could use in all the functions

    #[test]
    fn test_new_enemy() {
        // set up that is needed to make the enemy
        // required because I am using texture which depends on this set up
        let opengl = OpenGL::V3_2;
        //OpenGL function pointers must be loaded before creating the `Gl` backend
        //That is why this window is created
        let _window: Window = WindowSettings::new("create test", [500, 500])
            .exit_on_esc(true)
            .build()
            .expect("Error creating window"); 
        let _gl = GlGraphics::new(opengl);
        let _t_enemy = Enemy::new(0.0,100.0);
    }

    #[test]
    // I am testing this private function instead of the public update function
    // because this is where the logic is and update has a random chance of running this logic
    fn test_move_toward_player() {
        let opengl = OpenGL::V3_2;
        let _window: Window = WindowSettings::new("create test", [500, 500])
            .exit_on_esc(true)
            .build()
            .expect("Error creating window"); 
        let _gl = GlGraphics::new(opengl);

        let t_pos_before = 50.0;
        let mut t_enemy = Enemy::new(t_pos_before,t_pos_before);
        t_enemy.move_toward_player(10.0,10.0);
        assert!(t_enemy.pos.x < t_pos_before);
        assert!(t_enemy.pos.y < t_pos_before);
    }

}

