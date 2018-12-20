extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;
extern crate find_folder;

use piston::event_loop::*;
use piston::input::*;
use opengl_graphics::glyph_cache::GlyphCache;
use opengl_graphics::GlGraphics;
use glutin_window::GlutinWindow as Window;
use self::rand::Rng;

use models::player::Player;
use models::bullet::Bullet;
use models::enemy::Enemy;

use music;
use std::thread;

const FIRE_COOLDOWN: f64 = 1.5;

/// houses the direction that a game object may point in
#[derive(Copy, Clone)]
pub enum Direction {
    WEST,
    NORTH,
    EAST,
    SOUTH
}

/// Contains states and objects used in berzerk
pub struct Game {
	player: Player,
    player_bullets: Vec<Bullet>,
    enemy_bullets: Vec<Bullet>,    
    enemies: Vec<Enemy>,    
	dimensions: [f64;2],
	game_over: bool,
	score: u32,
    level:u32,
    fire_cooldown: f64,    
    pub walls: Vec<[f64;4]>, //make [f64;4] a Wall object with [x0 y0 x1 y1]
    new_level: bool,
    won: bool,
}

pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const YELLOW: [f32; 4] = [1.0, 1.0, 0.5, 1.0];
pub const BLUE: [f32; 4] = [0.5, 0.6, 0.7, 1.0];
pub const FPS: u64 = 60;

impl Game {
	pub fn new(width:f64, height: f64) -> Self {
		Game {
			player: Player::new(75.0, height / 2.0),
			player_bullets:Vec::<Bullet>::new(),
            enemy_bullets:Vec::<Bullet>::new(),            
			dimensions: [width,height],
			game_over: false,
            enemies: Vec::new(),            
            score: 0,
            level:1,
            fire_cooldown: 0.0,            
            walls: Vec::new(),  
            new_level: false,
            won: false,
		}
	}


//pos[x0, y0, x1, y1] for opposite points of rect
    pub fn make_border(&self, gl: &mut GlGraphics, c: graphics::Context, pos: [f64;4]) {
        use self::graphics::*;
        let square = rectangle::rectangle_by_corners(pos[0], pos[1],pos[2],pos[3]);
        let (x,y) = (0.0, 0.0);
        let transform = c.transform.trans(x,y);
        rectangle(BLUE, square, transform, gl);        
    }

    pub fn make_level_borders(&mut self,gl: &mut GlGraphics, c: graphics::Context,) {
        let half_width =self.dimensions[0]/2.0;
        let half_height =self.dimensions[1]/2.0;            
        let quarter_width = self.dimensions[0]/4.0;     
        let quarter_height = self.dimensions[1]/4.0;  

        let left_vertical = [5.0,5.0,30.0,self.dimensions[1]-75.0];
        let left_top =[30.0,5.0,half_width-125.0,30.0];
        let right_top = [half_width+125.0,5.0,self.dimensions[0]-5.0,30.0];
        let right_vertical = [self.dimensions[0]-30.0,25.0,self.dimensions[0]-5.0,self.dimensions[1]-75.0];
        let left_bottom = [25.0,self.dimensions[1]-100.0,(half_width)-125.0,self.dimensions[1]-75.0];
        let right_bottom = [half_width+125.0,self.dimensions[1]-100.0,self.dimensions[0]-5.0,self.dimensions[1]-75.0];
        let middle_top_vert = [quarter_width,quarter_height,quarter_width+25.0,quarter_height*3.0-75.0];
        let middle_right_vert = [quarter_width*3.0,quarter_height,quarter_width*3.0+25.0,quarter_height*3.0-75.0];
        let middle_middle = [quarter_width+25.0,half_height-50.0,quarter_width*3.0,half_height-25.0];

        // in update we use this to check for collision with enemy and eventually player
        self.add_walls(left_vertical,left_top,right_top,right_vertical,left_bottom,right_bottom,
            middle_top_vert, middle_right_vert, middle_middle);

        //border pieces of level 1
        self.make_border(gl,c, left_vertical);
        self.make_border(gl,c, left_top); 
        self.make_border(gl,c, right_top);
        self.make_border(gl,c, right_vertical);
        self.make_border(gl,c, left_bottom); 
        self.make_border(gl,c, right_bottom);

        //middle part of level 1
        self.make_border(gl,c, middle_top_vert); //left vertical
        self.make_border(gl,c, middle_right_vert); //right vertical
        self.make_border(gl,c, middle_middle); //middle bar
    }

//middle_top_vert, middle_right_vert, middle_middle
    pub fn add_walls(&mut self, lf:[f64;4],lt:[f64;4],rt:[f64;4],rv:[f64;4],lb:[f64;4],rb:[f64;4],mtv:[f64;4],mrv:[f64;4],mm:[f64;4],) {
        self.walls.push(lf);
        self.walls.push(lt);        
        self.walls.push(rt);
        self.walls.push(rv);
        self.walls.push(lb);
        self.walls.push(rb);
        self.walls.push(mtv);
        self.walls.push(mrv);
        self.walls.push(mm);
    }

	fn on_draw(&mut self, args: &RenderArgs, gl: &mut GlGraphics, glyph_cache: &mut GlyphCache) {
        use self::graphics::*;
        gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);
            for bullet in &self.player_bullets {
                bullet.draw(c, gl);
            }

            for bullet in &self.enemy_bullets {
                bullet.draw(c, gl);
            }

            for enemy in &mut self.enemies {
                enemy.draw(c,gl);
            }

            self.make_level_borders(gl,c);
            self.player.draw(c, gl);

            if self.player.health > 0 {
                let mut pos_heart = (self.dimensions[1]/4.0)*3.5;
                for _ in 0..self.player.health {
                    pos_heart +=35.0;
                    self.player.draw_lives(pos_heart, self.dimensions[1]-35.0, c,gl);
                }                
            }

            text(YELLOW, 38, format!("{}", self.score).as_str(), 
                glyph_cache, 
                c.transform.trans(self.dimensions[0]/2.0,self.dimensions[1]-25.0),
                gl);

            text(YELLOW, 38, format!("{}", self.level).as_str(), 
                glyph_cache, 
                c.transform.trans(50.0,self.dimensions[1]-25.0),
                gl);            


            if self.game_over {
                text(YELLOW, 38, format!("GAME OVER PRESS R TO RESTART").as_str(), 
                    glyph_cache, 
                    c.transform.trans(self.dimensions[0]/2.0-95.0,self.dimensions[1]/2.0),
                    gl);                 
            } 

            if self.won {
                text(YELLOW, 38, format!("CONGRATS YOU WON").as_str(), 
                    glyph_cache, 
                    c.transform.trans(self.dimensions[0]/2.0-95.0,self.dimensions[1]/2.0),
                    gl);                  
            }
        });
    }

    fn player_bullet_check(&mut self) {
        for bullet in &mut self.player_bullets {
            bullet.update();
            for enemy in &mut self.enemies {
                if bullet.collides_enemy(enemy) {
                    bullet.alive = false;
                    enemy.alive = false;
                    if self.fire_cooldown <= 0.0 {                    
                        music::play(2);
                        self.fire_cooldown = FIRE_COOLDOWN;
                    }
                    self.score += 50;
                }
            }  
            for wall in &self.walls {
                if bullet.collides_wall(wall){
                    bullet.alive = false;
                }                 
            }             
        }         
    }

    fn enemy_bullet_check(&mut self) {
        for bullet in &mut self.enemy_bullets {
            bullet.update();         
            if bullet.collides_p(&self.player) {
                bullet.alive = false;
                self.player.health -=1;
                if self.fire_cooldown <= 0.0 {                    
                    music::play(1);
                    self.fire_cooldown = FIRE_COOLDOWN;                    
                }
            }
            for wall in &self.walls {
                if bullet.collides_wall(wall){
                    bullet.alive = false;
                }                 
            }                      
        }         
    }

    fn enemy_chance_shoot(&mut self) {
        if self.enemies.len() != 0 {
            let chance_shot: u32 = rand::thread_rng().gen_range(1, 100-(3*self.level));
            if chance_shot == 5 {
                let index_enemy_shooting = rand::thread_rng().gen_range(0, self.enemies.len());
                let enemy_shooting = &self.enemies[index_enemy_shooting];
                if self.fire_cooldown <= 0.0 {                    
                    thread::spawn(|| {
                        music::play(3);
                    }); 
                    self.fire_cooldown = FIRE_COOLDOWN; //so two shooting threads dont start SDL                            
                    self.enemy_bullets.push(
                        Bullet::new(enemy_shooting.pos.x, enemy_shooting.pos.y, enemy_shooting.dir)
                    );                     
                }   
            }
        }        
    }

    fn enemy_update(&mut self) {
        for enemy in &mut self.enemies {
            enemy.update(self.player.pos.x, self.player.pos.y);            
            for wall in &self.walls {
                if enemy.collides(wall) {
                    enemy.alive = false;
                    if self.fire_cooldown <= 0.0 {                    
                        music::play(2);
                        self.fire_cooldown = FIRE_COOLDOWN;                        
                    }
                    self.score += 50;
                    return
                }
                if self.player.collides_enemy(enemy) {
                    enemy.alive = false;
                    self.player.health -=1;
                    self.player.place_random(self.dimensions); 
                    if self.fire_cooldown <= 0.0 {
                        music::play(1);   
                        self.fire_cooldown = FIRE_COOLDOWN;                                                   
                    }                    
                    return                 
                }
            }
        }        
    }

    fn wall_update(&mut self) {
        for wall in &self.walls {
            if self.player.collides(wall){
                self.player.health -= 1;
                self.player.place_random(self.dimensions);  
                if self.fire_cooldown <= 0.0 {
                   music::play(1); 
                   self.fire_cooldown = FIRE_COOLDOWN;
                }
                match self.player.dir {
                    Direction::NORTH => self.player.pos.y += 50.0,
                    Direction::SOUTH => self.player.pos.y -= 50.0,
                    Direction::EAST => self.player.pos.x -= 50.0,
                    Direction::WEST => self.player.pos.x += 50.0                }
            }          
        }        
    }

    fn on_update(&mut self, args: &UpdateArgs) {

        self.player_bullet_check();
        self.enemy_bullet_check();
        
        if self.fire_cooldown > 0.0 {
            self.fire_cooldown -= args.dt;
        }         

        self.player_bullets.retain(|b| b.alive); 
        self.enemy_bullets.retain(|b| b.alive);         
        self.enemies.retain(|enemy| enemy.alive);

        self.enemy_chance_shoot();
        self.enemy_update();
        self.wall_update();

        if self.player.health == 0 {
            if self.fire_cooldown <= 0.0 {
               music::play(4); 
               self.fire_cooldown = FIRE_COOLDOWN;
            }
            self.game_over = true;
        }   

        self.check_win();  

        if self.new_level {
            if self.level == 5 {
                self.won = true;
            } else {
                self.player.reset(75.0, self.dimensions[1] / 2.0);
                self.player_bullets.clear(); 
                let num_of_enemies = 4+(2*self.level);
                for _ in 0..num_of_enemies {
                    self.gameobject_random_placement();  
                }     
            }

            self.new_level = false;
        }   
 
    }

    fn input(&mut self, button: &Button, is_press: bool) {
        if is_press && !self.game_over && !self.won {
            if let Button::Keyboard(key) = *button {
                match key {
                    Key::Up => {
                        self.player.is_moving = true;
                        self.player.dir = Direction::NORTH;
                    },
                    Key::Down => {
                        self.player.is_moving= true;
                        self.player.dir = Direction::SOUTH;
                    },
                    Key::Left => {
                        self.player.is_moving= true;
                        self.player.dir = Direction::WEST;
                    },
                    Key::Right => {
                        self.player.is_moving= true;
                        self.player.dir = Direction::EAST;
                    },                   

                    Key::Space => {
                        self.player.is_moving= false;
                        if self.fire_cooldown <= 0.0 {
                            thread::spawn(|| {
                                music::play(0);
                            });     
                            self.fire_cooldown = FIRE_COOLDOWN; 
                            self.player_bullets.push(
                                Bullet::new(self.player.pos.x, self.player.pos.y, self.player.dir)
                            ); 
                        } else {
                            self.player_bullets.push(
                                Bullet::new(self.player.pos.x, self.player.pos.y, self.player.dir)
                            );                             
                        }
                    },

                    Key::R => {
                        self.hard_reset();
                    }
                    _ => (),
                }
            }
        } else { 
            self.player.is_moving= false;
            if let Button::Keyboard(key) = *button {
                match key {
                    Key::Up | Key::Down | Key::Left | Key::Right => self.player.is_moving =false,   
                    Key::R => {
                        self.hard_reset();
                    },
                    _ => (),
                }
            }
        }
    }    

	pub fn run(&mut self, window: &mut Window,
               mut gl: &mut GlGraphics,
               mut glyph_cache: &mut GlyphCache) {

		let mut events = Events::new(EventSettings::new());
        events.set_ups(FPS);
        let num_of_enemies = 4+(2*self.level);

        for _ in 0..num_of_enemies {
            self.gameobject_random_placement();  
        }        
      
        while let Some(e) = events.next(window) {
            if !self.game_over && !self.won {
                if let Some(r) = e.update_args() {
                    self.on_update(&r);
                }               
            }  

            if let Some(k) = e.press_args() {
                self.input(&k,true);
            } else if let Some(k) = e.release_args() {
                self.input(&k, false);
            }                 

            if let Some(u) = e.render_args() {
                self.on_draw(&u, &mut gl, &mut glyph_cache);
            }
        }

	}      

    fn gameobject_random_placement(&mut self) {
        let rand_block: u32 = rand::thread_rng().gen_range(1, 4);
        let mut randx: f64;
        let mut randy: f64;

        match rand_block {
            1 => { //left
                randx= rand::thread_rng().gen_range(45.0, self.dimensions[0]/4.0-20.0);
                randy= rand::thread_rng().gen_range(40.0, self.dimensions[1]-135.0);
            },
            2 => { //top
                randx= rand::thread_rng().gen_range(self.dimensions[0]/4.0+50.0, (self.dimensions[0]/4.0)*3.0);
                randy= rand::thread_rng().gen_range(40.0, self.dimensions[1]/4.0);
            },
            3 => { //right
                randx= rand::thread_rng().gen_range((self.dimensions[0]/4.0)*3.0+50.0, self.dimensions[0]-50.0);
                randy= rand::thread_rng().gen_range(40.0, self.dimensions[1]-135.0);                
            },
            4 => { //bottom
                randx= rand::thread_rng().gen_range(self.dimensions[0]/4.0+50.0, (self.dimensions[0]/4.0)*3.0);
                randy= rand::thread_rng().gen_range((self.dimensions[1]/4.0)*3.0, self.dimensions[1]-135.0);                
            },
            _ => {
                randx= rand::thread_rng().gen_range(45.0, self.dimensions[0]/4.0-20.0);
                randy = rand::thread_rng().gen_range(40.0, self.dimensions[1]-135.0);            
            }
        }

        //if enemy too close to the player starting try again 
        while randx > 40.0 && randx < 90.0 && randy > self.dimensions[1] / 2.0 - 50.0 && randy < self.dimensions[1] / 2.0 + 50.0 {
            randx = rand::thread_rng().gen_range(45.0, self.dimensions[0]/4.0-10.0); 
            randy = rand::thread_rng().gen_range(40.0, self.dimensions[1]-135.0); 
        }
        self.enemies.push(Enemy::new(randx, randy));            
    }

    fn hard_reset(&mut self) {
        self.level = 1;
        self.score = 0;
        self.won = false;
        self.player.reset(75.0, self.dimensions[1] / 2.0);
        self.player_bullets.clear(); 
        self.enemies.clear();       
        self.game_over = false;
        let num_of_enemies = 4+(2*self.level);
        for _ in 0..num_of_enemies {
            self.gameobject_random_placement();  
        } 
    }    

    fn check_win(&mut self) {
        if self.player.pos.x > self.dimensions[0]/2.0-125.0 && 
            self.player.pos.x < self.dimensions[0]/2.0+125.0 &&
            self.player.pos.y > 5.0 && self.player.pos.y < 30.0 &&
            self.enemies.len() == 0 {
                self.level +=1;
                self.new_level = true;
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
    fn test_new_game() {
        let opengl = OpenGL::V3_2;
        //OpenGL function pointers must be loaded before creating the `Gl` backend
        //That is why this window is created
           let _window: Window = WindowSettings::new("create test",
                                                     [500, 500])
            .exit_on_esc(true)
            .build()
            .expect("Error creating window"); 
        let _gl = GlGraphics::new(opengl);

        let _g = Game::new(500.0,500.0);
    } 

    #[test]
    fn test_game_walls() {
        let opengl = OpenGL::V3_2;
        //OpenGL function pointers must be loaded before creating the `Gl` backend
        //That is why this window is created
           let _window: Window = WindowSettings::new("create test",
                                                     [500, 500])
            .exit_on_esc(true)
            .build()
            .expect("Error creating window"); 
        let _gl = GlGraphics::new(opengl);

        let mut g = Game::new(500.0,500.0);
        let t_walls = [1.0,1.0,2.0,2.0];
        g.add_walls(t_walls,t_walls,t_walls,t_walls,t_walls,t_walls,t_walls,t_walls,t_walls);
        assert!(g.walls.len() == 9); 
    }

}
