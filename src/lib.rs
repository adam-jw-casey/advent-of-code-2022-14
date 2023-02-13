use std::ops::IndexMut;
use std::ops::Index;
use std::iter::zip;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Cave{
    source: Point,
    structure: HashMap<usize, HashMap<usize, Material>>,
    lowest_platform: usize
}

impl Cave{
    fn new() -> Self{
        Cave{
            source: Point{x:500,y:0},
            structure: HashMap::new(),
            lowest_platform: 0
        }
    }

    /// # Examples
    /// ```
    /// use advent_of_code_2022_14::Cave;
    ///
    /// assert_eq!(
    ///     concat!(
    ///         "498,4 -> 498,6 -> 496,6\n",
    ///         "503,4 -> 502,4 -> 502,9 -> 494,9"
    ///     ).parse::<Cave>().unwrap().bottom(500),
    ///     9
    /// );
    ///
    /// assert_eq!(
    ///     concat!(
    ///         "498,4 -> 498,6 -> 496,6\n",
    ///         "503,4 -> 502,4 -> 502,9 -> 494,9"
    ///     ).parse::<Cave>().unwrap().bottom(503),
    ///     4
    /// );
    /// ```
    pub fn bottom (&self, col: usize) -> usize{
        match self.structure.get(&col){
            Some(c) => *c.keys().max().unwrap_or(&0),
            None => 0
        }
    }

    fn add_sand(&mut self) -> Option<Point>{
        if self[&self.source] == Material::Sand{
            return None;
        }

        let end = self.drop_sand(&self.source.clone());
        if let Some(point) = end{
            self[&point] = Material::Sand;
        }
        end
    }

    fn drop_sand(&mut self, point: &Point) -> Option<Point>{
        let &Point{x,y} = point;

        match vec![
            Point{x,y:y+1},
            Point{x:x-1,y:y+1},
            Point{x:x+1,y:y+1}]
            .iter()
            .find(|&p| self[p] == Material::Air){
                Some(p) => self.drop_sand(p),
                None => Some(*point)
        }
    }
}

impl ToString for Cave{
    fn to_string(&self) -> String {
        let tl = Point{
            x: *self.structure.keys().min().expect("Should not be empty"),
            y: 0
        };

        let br = Point{
            x: *self.structure.keys().max().expect("Should not be empty"),
            y: self.structure.keys().map(|c| self.bottom(*c)).max().expect("Should not be empty")
        };

        let mut out_str = String::new();

        for y in tl.y..=br.y{
            for x in tl.x..=br.x{
                out_str.push_str(&self[&Point{x,y}].to_string());
            }
            out_str.push('\n');
        }

        out_str
    }
}

impl Index<&Point> for Cave{
    type Output = Material;

    fn index(&self, index: &Point) -> &Self::Output {
        self.structure
            .get(&index.x)
            .and_then(|col| col.get(&index.y))
            .unwrap_or({
                if index.y >= self.lowest_platform + 2{
                    &Material::Rock
                }else{
                    &Material::Air
                }
            })
    }
}

impl IndexMut<&Point> for Cave{
    fn index_mut(&mut self, index: &Point) -> &mut Self::Output {
        self.structure
            .entry(index.x)
            .or_insert(HashMap::new())
            .entry(index.y)
            .or_insert(Material::Air)
    }
}

#[derive(Debug)]
pub struct ParseCaveError;

impl FromStr for Cave{
    type Err = ParseCaveError;  

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cave = Cave::new();

        for line in s.lines(){
            let points = line
                .split(" -> ")
                .map(|x| str::parse::<Point>(x).expect("Should be a valid point"))
                .collect::<Vec<_>>();

            for window in points.windows(2).collect::<Vec<_>>(){
                let [first, second] = &window
                    else{
                        return Err(ParseCaveError);
                    };

                let xs: Vec<_>;
                let ys: Vec<_>;

                if first.x == second.x{
                    if first.y < second.y{
                        ys = (first.y..=second.y).collect();
                    }else{
                        ys = (second.y..=first.y).collect();
                    }
                    xs = ys.iter().map(|_| first.x).collect();
                }else if first.y == second.y{
                    if first.x < second.x{
                        xs = (first.x..=second.x).collect();
                    }else{
                        xs = (second.x..=first.x).collect();
                    }
                    ys = xs.iter().map(|_| first.y).collect();
                }else{
                    return Err(ParseCaveError);
                }

                for (&x,&y) in zip(xs.iter(), ys.iter()){
                    cave[&Point{x,y}] = Material::Rock;
                }
            }
        }
        cave.lowest_platform = cave.structure.keys().map(|c| cave.bottom(*c)).max().expect("Should not be empty");
        Ok(cave)
    }
}

#[derive(Eq, PartialEq)]
pub enum Material{
    Sand,
    Air,
    Rock
}

impl ToString for Material{
    fn to_string(&self) -> String {
        match self{
            Material::Sand => 'o'.to_string(),
            Material::Air  => '.'.to_string(),
            Material::Rock => '#'.to_string()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point{
    pub x: usize,
    pub y: usize
}

#[derive(Debug)]
pub struct ParsePointError;

impl FromStr for Point{
    type Err = ParsePointError;

    /// Assumes input in the form "x,y"
    /// # Examples
    /// ```
    /// use advent_of_code_2022_14::Point;
    /// assert_eq!(
    ///     Point{x:5,y:10},
    ///     "5,10".parse::<Point>().unwrap()
    /// );
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ints = s
            .split(',')
            .map(|x| x.parse::<usize>().expect("Should be valid ints"))
            .collect::<Vec<_>>();

        let [x,y] = &ints[..]
            else {
                return Err(ParsePointError);
            };

        Ok(Point{x:*x,y:*y})
    }
}

/// Calculate the amount of sand that comes to rest
/// # Examples
/// ```
/// use advent_of_code_2022_14::resting_sand;
/// assert_eq!(
///     93,
///     resting_sand(concat!(
///         "498,4 -> 498,6 -> 496,6\n",
///         "503,4 -> 502,4 -> 502,9 -> 494,9"
///     ))
/// );
/// ```
pub fn resting_sand(input: &str) -> usize{
    let mut cave = input.parse::<Cave>().expect("Should be a valid cave");
    while cave.add_sand().is_some(){};
    print!("{}", cave.to_string());
    cave.structure
        .values()
        .flat_map(|col| col.values())
        .filter(|&x| x == &Material::Sand)
        .count()
}
