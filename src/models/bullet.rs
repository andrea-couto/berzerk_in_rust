extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use opengl_graphics::GlGraphics;
use std::f64;
use models::vector::Vector;
use models::enemy::Enemy;
use game::Direction;
use models::player::Player;

pub const BULLET_SPEED:f64 = 5.0;
pub const BULLET_SIZE:f64 = 5.0;
pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

/// Bullets are used by enemies and the player
/// pos: the position of the bullet in the window
/// alive: used to remove bullets when they collide with game objects
/// dir: the direction the bullet is heading
pub struct Bullet {
    pos: Vector,
    pub alive: bool,
    dir: Direction,    
}

impl Bullet {
    /// creates a new bullet
    pub fn new(xpos: f64, ypos: f64, dir: Direction) -> Self {
        Bullet {
            pos: Vector::new(xpos, ypos),
            alive: true,
            dir,
        }
    }

    /// draws the bullet as a rectangle on the screen
    /// the function uses the global BULLET_SIZE
    pub fn draw(&self, c: graphics::Context, gl: &mut GlGraphics) {
        use self::graphics::*;
        let square = rectangle::square(0.0, 0.0, BULLET_SIZE);
        let transform = c.transform.trans(self.pos.x-20.0, self.pos.y);
        rectangle(WHITE, square, transform, gl);
    }

    /// adjusts the direction of the bullet
    pub fn update(&mut self) {
        match self.dir {
            Direction::EAST => self.pos.x += BULLET_SPEED,
            Direction::NORTH => self.pos.y -= BULLET_SPEED,
            Direction::WEST => self.pos.x -= BULLET_SPEED,
            Direction::SOUTH => self.pos.y += BULLET_SPEED,
        }
    }  

    /// checks for collision with enemey
    /// based on a generic way to test for collision in games
    pub fn collides_enemy(&self, other: &Enemy) -> bool {
        // make into bounding boxes if time
        let x2 = self.pos.x - other.pos.x;
        let y2 = self.pos.y - other.pos.y;
        let sum = x2.powf(2.0) + y2.powf(2.0);

        let r_start = BULLET_SIZE/2.0 - other.size/2.0;
        let r_end = BULLET_SIZE/2.0 + other.size/2.0;

        r_start.powf(2.0) <= sum && sum <= r_end.powf(2.0)
    } 

    /// checks for collision with player
    pub fn collides_p(&self, other: &Player) -> bool {
        // make into bounding boxes if time
        let x2 = self.pos.x - other.pos.x;
        let y2 = self.pos.y - other.pos.y;
        let sum = x2.powf(2.0) + y2.powf(2.0);

        let r_start = BULLET_SIZE/2.0 - other.player_y_size/2.0;
        let r_end = BULLET_SIZE/2.0 + other.player_y_size/2.0;

        r_start.powf(2.0) <= sum && sum <= r_end.powf(2.0)
    }   

    /// check for collision with wall
    pub fn collides_wall(&self, wall: &[f64;4]) -> bool{
        let collision_x: bool = self.pos.x + BULLET_SIZE >= wall[0] &&
        wall[0]+(wall[2]-wall[0]) >= self.pos.x;
        let collision_y: bool = self.pos.y + BULLET_SIZE >= wall[1] &&
            wall[1] + (wall[3]-wall[1]) >= self.pos.y;
        return collision_x && collision_y        
    }  

}

#[cfg(test)] 
mod berzerk_test {
    use super::*;
    #[test]
    fn test_update_east() {
        let mut t_bullet = Bullet::new(0.0,0.0, Direction::EAST);
        t_bullet.update();
        assert!(t_bullet.pos.x == BULLET_SPEED);
    }

    #[test]
    fn test_update_west() {
        let mut t_bullet = Bullet::new(0.0,0.0, Direction::WEST);
        t_bullet.update();
        assert!(t_bullet.pos.x == -BULLET_SPEED);
    }

    #[test]
    fn test_update_north() {
        let mut t_bullet = Bullet::new(0.0,100.0, Direction::NORTH);
        t_bullet.update();
        let expected = 100.0 - BULLET_SPEED;
        assert!(t_bullet.pos.y == expected);
    }

    #[test]
    fn test_update_south() {
        let mut t_bullet = Bullet::new(0.0,100.0, Direction::SOUTH);
        t_bullet.update();
        let expected = 100.0 + BULLET_SPEED;
        assert!(t_bullet.pos.y == expected);
    }   
}


