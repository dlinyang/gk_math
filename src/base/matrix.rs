use std::ops::{Add, Sub, Mul, Div, Index, IndexMut};
use std::fmt;

pub struct Mat<T, const R: usize, const C: usize> {
    pub data: [[T; C]; R],
}

impl<T, const R: usize, const C: usize> Mat<T, R, C> { 

    #[inline]
    pub fn new(data: [[T; C]; R]) -> Self { 
        Self { data } 
    } 

    #[inline]
    pub fn shape(&self) -> (usize, usize) {
        (R, C)
    }
    
    #[inline]
    pub fn rows(&self) -> usize {
        R
    }

    #[inline]
    pub fn cols(&self) -> usize {
        C
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.data.get_mut(row)?.get_mut(col)
    }
    
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get(row)?.get(col)
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) -> Result<(), &'static str> {
        if row >= R || col >= C {
            return Err("Index out of bounds");
        }
        self.data[row][col] = value;
        Ok(())
    }

    #[inline]
    pub fn is_square(&self) -> bool {
        R == C
    }
}


impl<T, const R: usize, const C: usize> Add for Mat<T, R, C>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;
        for i in 0..R {
            for j in 0..C {
                result.data[i][j] = result.data[i][j] + rhs.data[i][j];
            }
        }
        result
    }
}

impl<T, const R: usize, const C: usize> Sub for Mat<T, R, C>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self;
        for i in 0..R {
            for j in 0..C {
                result.data[i][j] = result.data[i][j] - rhs.data[i][j];
            }
        }
        result
    }
}

impl<T, const R: usize, const C: usize> Mul<T> for Mat<T, R, C>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;
    
    fn mul(self, scalar: T) -> Self::Output {
        let mut result = self;
        for i in 0..R {
            for j in 0..C {
                result.data[i][j] = result.data[i][j] * scalar;
            }
        }
        result
    }
}

impl<T, const R: usize, const C: usize> Div<T> for Mat<T, R, C>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;
    
    fn div(self, scalar: T) -> Self::Output {
        let mut result = self;
        for i in 0..R {
            for j in 0..C {
                result.data[i][j] = result.data[i][j] / scalar;
            }
        }
        result
    }
}

impl<T, const R: usize, const C: usize, const C2: usize> Mul<Mat<T, C, C2>> for Mat<T, R, C>
where
    T: Mul<Output = T> + Add<Output = T> + Default + Copy,
{
    type Output = Mat<T, R, C2>;
    
    fn mul(self, rhs: Mat<T, C, C2>) -> Self::Output {
        let mut result_data = [[T::default(); C2]; R];
        
        for i in 0..R {
            for j in 0..C2 {
                let mut sum = T::default();
                for k in 0..C {
                    sum = sum + (self.data[i][k] * rhs.data[k][j]);
                }
                result_data[i][j] = sum;
            }
        }
        
        Mat { data: result_data }
    }
}

impl<T, const R: usize, const C: usize> Mat<T, R, C>
where
    T: Copy,
{
    pub fn transpose(&self) -> Mat<T, C, R> {
        let mut result_data = [[self.data[0][0]; R]; C];
        
        for i in 0..R {
            for j in 0..C {
                result_data[j][i] = self.data[i][j];
            }
        }
        
        Mat { data: result_data }
    }
}

impl<T, const R: usize, const C: usize> Index<usize> for Mat<T, R, C> {
    type Output = [T;C];
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const R: usize, const C: usize> IndexMut<usize> for Mat<T, R, C> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T, const R: usize, const C: usize> fmt::Debug for Mat<T, R, C>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Matrix {}x{}:", R, C)?;
        for i in 0..R {
            write!(f, "[")?;
            for j in 0..C {
                write!(f, "{:?}", self.data[i][j])?;
                if j < C - 1 {
                    write!(f, ", ")?;
                }
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

impl<T, const N: usize> Mat<T, N, N>
where
    T: Default + Copy,
{
    pub fn identity() -> Self 
    where
        T: From<u8> + PartialEq + Default + Copy,
    {
        let mut result = Self::default();
        for i in 0..N {
            result.data[i][i] = T::from(1u8);
        }
        result
    }
    
    pub fn diag(diagonal: [T; N]) -> Self 
    where
        T: Default + Copy,
    {
        let mut result = Self::default();
        for i in 0..N {
            result.data[i][i] = diagonal[i];
        }
        result
    }
}

impl<T, const R: usize, const C: usize> Mat<T, R, C>
where
    T: Copy + Default,
{
    pub fn submatrix<const I: usize, const J: usize, const R1: usize, const C1: usize>(&self) -> Mat<T, R1, C1> {
        let mut result = Mat::default();
        for i in 0..R1 {
            for j in 0..C1 {
                if I + i < R && J + j < C {
                    result.data[i][j] = self.data[I + i][J + j];
                }
            }
        }
        result
    }
    
    pub fn flatten_row_major(&self) -> Vec<T> {
        let mut result = Vec::with_capacity(R * C);
        for i in 0..R {
            for j in 0..C {
                result.push(self.data[i][j]);
            }
        }
        result
    }
    
    pub fn flatten_col_major(&self) -> Vec<T> {
        let mut result = Vec::with_capacity(R * C);
        for j in 0..C {
            for i in 0..R {
                result.push(self.data[i][j]);
            }
        }
        result
    }
}

impl<T, const R: usize, const C: usize> Mat<T, R, C>
where
    T: Mul<Output = T> + Add<Output = T> + Default + Copy,
{
    pub fn sum(&self) -> T {
        let mut total = T::default();
        for i in 0..R {
            for j in 0..C {
                total = total + self.data[i][j];
            }
        }
        total
    }
    
    pub fn product(&self) -> T 
    where
        T: From<u8>,
    {
        let mut total = T::from(1u8);
        for i in 0..R {
            for j in 0..C {
                total = total * self.data[i][j];
            }
        }
        total
    }
}

impl<T, const R: usize, const C: usize> PartialEq for Mat<T, R, C>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        for i in 0..R {
            for j in 0..C {
                if self.data[i][j] != other.data[i][j] {
                    return false;
                }
            }
        }
        true
    }
}

impl<T, const R: usize, const C: usize> Mat<T, R, C> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().flat_map(|row| row.iter())
    }
    
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut().flat_map(|row| row.iter_mut())
    }
    
    pub fn rows_iter(&self) -> impl Iterator<Item = &[T; C]> {
        self.data.iter()
    }
    
    pub fn rows_iter_mut(&mut self) -> impl Iterator<Item = &mut [T; C]> {
        self.data.iter_mut()
    }
}

impl<T, const R: usize, const C: usize> Mat<T, R, C>
where
    T: From<u8> + Copy,
{
    pub fn zeros() -> Self {
        Self {
            data: [[T::from(0u8); C]; R],
        }
    }
    
    pub fn ones() -> Self {
        Self {
            data: [[T::from(1u8); C]; R],
        }
    }
}

impl<T, const R: usize, const C: usize> Clone for Mat<T, R, C>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        let mut data = unsafe { std::mem::MaybeUninit::<[[T; C]; R]>::uninit().assume_init() };
        for i in 0..R {
            for j in 0..C {
                data[i][j] = self.data[i][j].clone();
            }
        }
        Self { data }
    }
}

impl<T, const R: usize, const C: usize> Copy for Mat<T, R, C> where T: Copy {}

impl<T, const R: usize, const C: usize> Default for Mat<T, R, C> 
where T: Default + Copy 
{
    fn default() -> Self {
        Self {
            data: [[T::default(); C]; R],
        }
    }
}

impl<T, const R: usize, const C: usize> Mat<T, R, C>
where
    T: Default + Copy,
{
    pub fn from_vec(vec: &[T]) -> Result<Self, &'static str> {
        if vec.len() != R * C {
            return Err("Vector length does not match matrix dimensions");
        }
        
        let mut data = [[T::default(); C]; R];
        for i in 0..R {
            for j in 0..C {
                data[i][j] = vec[i * C + j];
            }
        }
        
        Ok(Self { data })
    }

    pub fn from_vec_col_major(vec: &[T]) -> Result<Self, &'static str> {
        if vec.len() != R * C {
            return Err("Vector length does not match matrix dimensions");
        }
        
        let mut data = [[T::default(); C]; R];
        for i in 0..R {
            for j in 0..C {
                data[i][j] = vec[j * R + i];
            }
        }
        
        Ok(Self { data })
    }
}