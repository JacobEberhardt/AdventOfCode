#[derive(Debug, Clone)]
struct Gridwalker{
    position: Point,
    distance_covered: u32,
    orientation: Orientation,
}

#[derive(Debug, Clone, PartialEq)]
struct Point{
    x: usize,
    y: usize
}

#[derive(Debug, Clone)]
enum Orientation{
    Top, Bottom, Left, Right
}

impl Gridwalker{
    fn has_left(&self, grid: &Vec<Vec<u32>>) -> bool {
        match self.orientation {
            Orientation::Top => grid[self.position.x-1][self.position.y] != 0,
            Orientation::Bottom => grid[self.position.x+1][self.position.y] != 0,
            Orientation::Left => grid[self.position.x][self.position.y-1] != 0,
            Orientation::Right => grid[self.position.x][self.position.y+1] != 0
        }
    }

    fn turn_left(&mut self){
        match self.orientation {
            Orientation::Top => self.orientation = Orientation::Left,
            Orientation::Bottom => self.orientation = Orientation::Right,
            Orientation::Left => self.orientation = Orientation::Bottom,
            Orientation::Right => self.orientation = Orientation::Top
        }
    }

    fn make_step(&mut self){
        match self.orientation {
            Orientation::Top => self.position.y += 1,
            Orientation::Bottom => self.position.y -= 1,
            Orientation::Left => self.position.x -= 1,
            Orientation::Right => self.position.x += 1
        }
        self.distance_covered += 1;
    }

    fn update_field(&mut self, grid: &mut Vec<Vec<u32>>, value: u32){
        grid[self.position.x][self.position.y] = value;
    }

    fn get_field_value(&self, grid:&Vec<Vec<u32>>) -> u32{
        grid[self.position.x][self.position.y]
    }

    fn get_distance_covered(&self)->u32{
        self.distance_covered
    }

    fn get_adjacent_field_sum(&mut self, grid: &mut Vec<Vec<u32>>) -> u32{
        let initial_position = self.position.clone();
        let initial_orientation = self.orientation.clone();
        self.orientation=Orientation::Top;

        let mut sum = 0;
        // walk the circle and add values
        self.make_step();
        self.turn_left();
        for _ in 0..4 {
            sum += self.get_field_value(grid);
            self.make_step();
            sum += self.get_field_value(grid);
            self.turn_left();
            self.make_step();
        }
        self.turn_left();
        self.make_step();
        
        assert_eq!(initial_position, self.position); // end where we started
        self.orientation = initial_orientation; // and have the same orientation
        sum
    }
}

pub fn grid_manhattan_distance(input: &u32) -> isize {

// generate grid

    // find n s.t. x<=sqrt(n)%2==1
    let mut gridsize = (*input as f64).sqrt() as usize + 1;
    loop {
        if (gridsize^2) % 2 == 1 {
            break
        } else {
            gridsize += 1;
        }
    }

    // create grid
    let mut grid: Vec<Vec<u32>> = Vec::with_capacity(gridsize);
    for _ in 0..gridsize {
        let mut col = Vec::<u32>::with_capacity(gridsize);
        for _ in 0..gridsize {
            col.push(0);
        }
        grid.push(col);
    }

    // initialize grid

    let access_port_index = (gridsize-1)/2;

        // initialize fields 1 and 2
        grid[access_port_index][access_port_index] = 1;
        grid[access_port_index+1][access_port_index] = 2;

        // initialize gridwalker
        let mut grid_walker = Gridwalker{
            position: Point{x: access_port_index+1, y: access_port_index},
            distance_covered: 2,
            orientation: Orientation::Top
            };

        // walk the grid until at the target position
        loop{
            if grid_walker.distance_covered == *input {
                break; // we're at the goal!
            }
            // continue walking the grid
            if grid_walker.has_left(&grid) {
                grid_walker.make_step();
                let steps = grid_walker.get_distance_covered();
                grid_walker.update_field(&mut grid, steps);
            } else {
                grid_walker.turn_left();
            }
        }

    let manhattan_distance = (grid_walker.position.x as isize - access_port_index as isize).abs() + (grid_walker.position.y as isize - access_port_index as isize).abs();

    manhattan_distance
}


pub fn grid_adjacent_sum(input: &u32) -> u32 {

// generate grid

    // find n s.t. x<=sqrt(n)%2==1
    let mut gridsize = (*input as f64).sqrt() as usize + 1;
    loop {
        if (gridsize^2) % 2 == 1 {
            break
        } else {
            gridsize += 1;
        }
    }

    // create grid
    let mut grid: Vec<Vec<u32>> = Vec::with_capacity(gridsize);
    for _ in 0..gridsize {
        let mut col = Vec::<u32>::with_capacity(gridsize);
        for _ in 0..gridsize {
            col.push(0);
        }
        grid.push(col);
    }

    // initialize grid
    let access_port_index = (gridsize-1)/2;

        // initialize fields 1 and 2
        grid[access_port_index][access_port_index] = 1;
        grid[access_port_index+1][access_port_index] = 1;

        // initialize gridwalker
        let mut grid_walker = Gridwalker{
            position: Point{x: access_port_index+1, y: access_port_index},
            distance_covered: 2,
            orientation: Orientation::Top
            };

        // walk the grid until at the target position
        loop{
            if grid_walker.get_field_value(&grid) >= *input {
                break; // we're at the goal!
            }
            // continue walking the grid
            if grid_walker.has_left(&grid) {
                grid_walker.make_step();
                let adjacent_field_sum = grid_walker.get_adjacent_field_sum(&mut grid);
                grid_walker.update_field(&mut grid, adjacent_field_sum);
                //print!("Grid: {:?}", grid);
            } else {
                grid_walker.turn_left();
                // println!("turn");
            }
        }
    
    grid_walker.get_field_value(&grid)
}