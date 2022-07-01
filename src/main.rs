use std::fmt::Debug;

#[derive(Debug, Clone)]
struct Grid<T>
{
    data: Vec<T>,
    dims: (usize, usize)
}

#[derive(Debug, Clone, Copy)]
enum Light
{
    Source(u8),
    Space(u8)
}

impl std::fmt::Display for Light
{
    fn fmt(&self,  f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let intensity = match self
        {
            Light::Source(intensity) => intensity,
            Light::Space(intensity) => intensity
        };
        if *intensity == 0
        {
            write!(f, "   ")            
        }
        else
        {
            write!(f, "{:^3}", intensity)
        }
    }
}


impl<T: Copy + Debug> Grid<T>
{
    fn new(dims: (usize, usize), default: T) -> Self
    {
        Self{data: vec![default; dims.0*dims.1], dims}
    }

    fn get<'a>(&'a self, (i,j): (usize, usize)) -> Option<&'a T>
    {
        self.data.get(i + j*self.dims.0)
    }
    fn get_mut<'a>(&'a mut self, (i,j): (usize, usize)) -> Option<&'a mut T>
    {
        self.data.get_mut(i + j*self.dims.0)
    }
    fn neighborhood(&self, (i, j): (usize, usize)) -> Vec<&T>
    {
        
        let coords = if (i+j) & 1 == 0
        {
            // 1/c\2
            //  \3/
            [Some((i,j)),
             if (i > 0) {Some((i-1, j))} else {None},
             if (i+1 < self.dims.0) {Some((i+1, j))} else {None},
             if (j+1 < self.dims.1) {Some((i, j+1))} else {None}]
                
        }
        else
        {
            //  /3\
            // 1\c/2
            [Some((i,j)),
             if (i > 0) {Some((i-1, j))} else {None},
             if (i+1 < self.dims.0) {Some((i+1, j))} else {None},
             if (j > 0) {Some((i, j-1))} else {None}]
        };

        coords.iter()
            .filter(|x| x.is_some())
            .map(|co| self.get(co.unwrap()))
            .filter(|x| x.is_some()).map(|x| x.unwrap()).collect()
        
    }

}
impl<T: Copy + Debug + std::fmt::Display> Grid<T>
{
    fn print(&self)
    {
        print!("      ·");
        for i in (1..self.dims.0).step_by(2)
        {
            print!("-----·");
        }
        println!();
        for j in (0..self.dims.1).step_by(2)
        {


            print!("     /");
            for i in (1..self.dims.0).step_by(2)
            {
                print!(" \\{:^3}/", self.get((i, j)).unwrap())
            }
            if self.dims.0 % 2 == 1
            {
                print!(" \\");
            }
            println!();
            print!("    ");
            for i in (0..self.dims.0).step_by(2)
            {
                print!("/{:^3}\\ ", self.get((i, j)).unwrap());
            }
            if self.dims.0 % 2 == 0
            {
                print!("/");
            }
            println!();
            print!("   ·");
            for i in (0..self.dims.0).step_by(2)
            {
                print!("-----·");
            }
            println!();

            if j+1 == self.dims.1
            {
                break;
            }
            
            print!("    ");
            for i in (0..self.dims.0).step_by(2)
            {
                print!("\\{:^3}/ ", self.get((i, j+1)).unwrap());
            }
            if self.dims.0 % 2 == 0
            {
                print!("\\");
            }
            println!();
            print!("     \\");
            for i in (1..self.dims.0).step_by(2)
            {
                print!(" /{:^3}\\", self.get((i, j+1)).unwrap())
            }
            if self.dims.0 % 2 == 1
            {
                print!(" /");
            }
            println!();

            print!("      ·");
            for i in (1..self.dims.0).step_by(2)
            {
                print!("-----·");
            }
            println!();

        }
        

    }


    
}


struct Automata<T>
{
    grids: [Grid<T>; 2],
    flag: usize
}

impl<T: Clone + std::fmt::Display + Copy + std::fmt::Debug> Automata<T>
{
    fn new(grid: Grid<T>) -> Self
    {
        Self
        {
            grids: [grid.clone(), grid.clone()],
            flag: 0
        }
    }

    fn print(&self)
    {
        self.grids[self.flag].print();
    }
    
    fn evolve<F>(&mut self, rule: F)
    where
        F: Fn(Vec<T>) -> T
    {
        for i in 0..self.grids[self.flag].dims.0
        {
            for j in 0..self.grids[self.flag].dims.1
            {
                let ngh = self.grids[self.flag].neighborhood((i, j)).into_iter().cloned().collect::<Vec<_>>();
                let new_cell = self.grids[(self.flag+1)%2].get_mut((i,j)).unwrap();
                *new_cell = rule(ngh);
            }
        }
        self.flag = (self.flag+1) % 2;
        
    }

    fn get<'a>(&'a self, (i,j): (usize, usize)) -> Option<&'a T>
    {
        self.grids[self.flag].get((i,j))
    }
    fn get_mut<'a>(&'a mut self, (i,j): (usize, usize)) -> Option<&'a mut T>
    {
        self.grids[self.flag].get_mut((i,j))
    }

}



fn main()
{
    let (w,h) = (30, 20);

    let mut automata = Automata::new(Grid::new((w,h), Light::Space(0)));

    automata.get_mut((10,10)).map(|light| *light = Light::Source(10));
    
    let mut flag = 0usize;

    let rule = |ngh: Vec<Light>| {
        match ngh[0]
        {
            Light::Space(_) =>
            {
                let mut max = 0u8;
                for ncel in ngh
                {
                    let level = match ncel
                    {
                        Light::Source(level) => level,
                        Light::Space(level) => level
                    };
                    if level > max
                    {
                        max = level;
                    }
                }
                Light::Space(max.max(1)-1)
            },
            Light::Source(lvl) => Light::Source(lvl)
        }

    };

    for _ in 0..10
    {
        automata.print();
        automata.evolve(rule);
    }
    automata.get_mut((10,10)).map(|light| *light = Light::Space(10));

    for _ in 0..20
    {
        automata.print();
        automata.evolve(rule);
    }

    
    
}

//      ·-----·
//     / \ 2 / \
//    / 1 \ / 3 \
//   ·-----·-----·
//    \ 6 / \ 4 /
//     \ / 5 \ /
//      ·-----·
