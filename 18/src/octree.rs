use crate::Point;
use std::cmp::Ordering::{Equal, Greater, Less};

const NUMBER_OF_CHILDREN: usize = 8;

#[derive(Debug)]
pub struct OctreeError {
    msg: String,
}
impl OctreeError {
    fn new(arg: &str) -> OctreeError {
        OctreeError {
            msg: arg.to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Octree {
    Leaf(Option<Point>),
    Tree(Tree),
}

#[derive(Debug, Clone, Copy)]
enum Octant {
    TopLeftFront = 0,
    TopRightFront = 1,
    BottomRightFront = 2,
    BottomLeftFront = 3,
    TopLeftBack = 4,
    TopRightBack = 5,
    BottomRightBack = 6,
    BottomLeftBack = 7,
}

#[derive(Debug)]
pub struct Tree {
    min_point: Point,
    max_point: Point,

    children: [Box<Octree>; NUMBER_OF_CHILDREN],
}

impl Tree {
    pub fn new(min_point: Point, max_point: Point) -> Tree {
        let children = [
            Box::new(Octree::Leaf(None)),
            Box::new(Octree::Leaf(None)),
            Box::new(Octree::Leaf(None)),
            Box::new(Octree::Leaf(None)),
            Box::new(Octree::Leaf(None)),
            Box::new(Octree::Leaf(None)),
            Box::new(Octree::Leaf(None)),
            Box::new(Octree::Leaf(None)),
        ];

        log::trace!("{:?}, {:?}", min_point, max_point);

        Tree {
            min_point,
            max_point,
            children,
        }
    }

    fn validate_point(&self, point: Point) -> Result<(), OctreeError> {
        log::trace!("Min: {:?}, Max: {:?}", self.min_point, self.max_point);
        log::trace!("Point to validate: {:?}", point);

        if (point < self.min_point) || (point >= self.max_point) {
            return Err(OctreeError::new("Point out of bounds."));
        }

        Ok(())
    }

    pub fn insert(&mut self, point: Point) -> Result<(), OctreeError> {
        self.validate_point(point)?;

        if self.find(point)? {
            return Err(OctreeError::new("Point already exists!"));
        }

        let mid = self.get_mid_point();
        let octant = Tree::get_octant(mid, point);

        let index = octant as usize;
        let child: &mut Octree = &mut self.children[index];

        log::trace!("Child: {:?}", child);
        match child {
            Octree::Leaf(Some(_)) => unreachable!(),
            Octree::Tree(tree) => tree.insert(point),
            Octree::Leaf(None) => {
                // If node is a 1x1x1.
                if ((self.max_point.x - self.min_point.x) == 2)
                    && ((self.max_point.y - self.min_point.y) == 2)
                    && ((self.max_point.z - self.min_point.z) == 2)
                {
                    self.children[index] = Box::new(Octree::Leaf(Some(point)));
                } else {
                    let (new_min, new_max) = match octant {
                        Octant::TopLeftFront => (
                            Point {
                                x: self.min_point.x,
                                y: mid.y,
                                z: self.min_point.z,
                            },
                            Point {
                                x: mid.x,
                                y: self.max_point.y,
                                z: mid.z,
                            },
                        ),
                        Octant::TopRightFront => (
                            Point {
                                x: mid.x,
                                y: mid.y,
                                z: self.min_point.z,
                            },
                            Point {
                                x: self.max_point.x,
                                y: self.max_point.y,
                                z: mid.z,
                            },
                        ),
                        Octant::BottomRightFront => (
                            Point {
                                x: mid.x,
                                y: self.min_point.y,
                                z: self.min_point.z,
                            },
                            Point {
                                x: self.max_point.x,
                                y: mid.y,
                                z: mid.z,
                            },
                        ),

                        Octant::BottomLeftFront => (
                            Point {
                                x: self.min_point.x,
                                y: self.min_point.y,
                                z: self.min_point.z,
                            },
                            Point {
                                x: mid.x,
                                y: mid.y,
                                z: mid.z,
                            },
                        ),
                        Octant::TopLeftBack => (
                            Point {
                                x: self.min_point.x,
                                y: mid.y,
                                z: mid.z,
                            },
                            Point {
                                x: mid.x,
                                y: self.max_point.y,
                                z: self.max_point.z,
                            },
                        ),
                        Octant::TopRightBack => (
                            Point {
                                x: mid.x,
                                y: mid.y,
                                z: mid.z,
                            },
                            Point {
                                x: self.max_point.x,
                                y: self.max_point.y,
                                z: self.max_point.z,
                            },
                        ),
                        Octant::BottomRightBack => (
                            Point {
                                x: mid.x,
                                y: self.min_point.y,
                                z: mid.z,
                            },
                            Point {
                                x: self.max_point.x,
                                y: mid.y,
                                z: self.max_point.z,
                            },
                        ),
                        Octant::BottomLeftBack => (
                            Point {
                                x: self.min_point.x,
                                y: self.min_point.y,
                                z: mid.z,
                            },
                            Point {
                                x: mid.x,
                                y: mid.y,
                                z: self.max_point.z,
                            },
                        ),
                    };

                    let mut new_tree = Tree::new(new_min, new_max);
                    log::trace!("Point: {:?}", point);
                    log::trace!("Tree: {:?}", new_tree);
                    new_tree.insert(point)?;

                    self.children[index] = Box::new(Octree::Tree(new_tree));
                }

                Ok(())
            }
        }
    }

    pub fn find(&self, point: Point) -> Result<bool, OctreeError> {
        self.validate_point(point)?;

        let mid = self.get_mid_point();

        let octant = Tree::get_octant(mid, point);

        let child: &Octree = &self.children[octant as usize];

        match child {
            Octree::Leaf(None) => Ok(false),
            Octree::Leaf(Some(p)) => Ok(*p == point),
            Octree::Tree(tree) => tree.find(point),
        }
    }

    fn get_mid_point(&self) -> Point {
        let x = (self.max_point.x + self.min_point.x) / 2;
        let y = (self.max_point.y + self.min_point.y) / 2;
        let z = (self.max_point.z + self.min_point.z) / 2;

        Point { x, y, z }
    }

    fn get_octant(mid: Point, point: Point) -> Octant {
        match (
            point.x.cmp(&mid.x),
            point.y.cmp(&mid.y),
            point.z.cmp(&mid.z),
        ) {
            (Less, Less, Less) => Octant::BottomLeftFront,
            (Less, Less, Equal | Greater) => Octant::BottomLeftBack,
            (Equal | Greater, Less, Less) => Octant::BottomRightFront,
            (Equal | Greater, Less, Equal | Greater) => Octant::BottomRightBack,
            (Less, Equal | Greater, Less) => Octant::TopLeftFront,
            (Less, Equal | Greater, Equal | Greater) => Octant::TopLeftBack,
            (Equal | Greater, Equal | Greater, Less) => Octant::TopRightFront,
            (Equal | Greater, Equal | Greater, Equal | Greater) => Octant::TopRightBack,
        }
    }
}
