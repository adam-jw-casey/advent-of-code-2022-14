use std::{collections::HashMap, str::FromStr};

struct Cave{
    source: Point,
    structure: HashMap<usize, HashMap<usize, Material>>
}

struct ParseCaveError;

impl FromStr for Cave{
    type Err = ParseCaveError;  

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

enum Material{
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

pub struct Point{
    x: u16,
    y: u16
}

pub struct ParsePointError;

impl FromStr for Point{
    type Err = ParsePointError;

    /// Assumes input in the form "x,y"
    /// # Examples
    /// ```
    /// use advent_of_code_2022_14::Point;
    /// assert_eq!(
    ///     Point{x:5,y:10},
    ///     "5,10".parse::<Point>()
    /// );
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ints = s
            .split(',')
            .map(|x| x.parse::<u16>().expect("Should be valid ints"))
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
///     24,
///     resting_sand(concat!(
///         "498,4 -> 498,6 -> 496,6\n",
///         "503,4 -> 502,4 -> 502,9 -> 494,9"
///     ))
/// );
/// ```
pub fn resting_sand(input: &str) -> usize{
    todo!();
}
