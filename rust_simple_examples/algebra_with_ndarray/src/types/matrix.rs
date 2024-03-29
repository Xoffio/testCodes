use ndarray::*;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;

pub fn print_matrix(){
    // Create a 2d matrix. (3x2)
    let _a = arr2(&[[1, 2],
                                                     [3, 4],
                                                     [5, 6]]);
    
    // Another way to create it.
    let a = arr2(&[[1, 2], [3, 4], [5, 6]]);

    // Another way to create a 1d, 2d or 3d matrix.
    let b = array![[1, 2], [3, 4], [5, 6]];

    println!("--------");
    println!("{}", a);
    println!("{:?}", a.dim());
    println!("{:?}", a.len());
    println!("--");
    println!("{}", b);
    println!("{:?}", b.dim());
    println!("{:?}", b.len());
    println!("--------\n");
}

pub fn print_matrix_ones(){
    // Create a 2x3 matrix of ones.
    // Specify both the element type and dimensionality:
    let ones1 = Array::<f64, Ix2>::ones((2, 3));
    let ones2 = Array2::<f64>::ones((2, 3));

    // Create a 3x3 matrix of ones.
    // Specify just the element type and infer the dimensionality:
    let ones3 = Array::<f64, _>::ones((3, 3));

    println!("--------");
    println!("{}", ones1);
    println!("--------\n");

    println!("--------");
    println!("{}", ones2);
    println!("--------\n");

    println!("--------");
    println!("{}", ones3);
    println!("--------\n");
}

pub fn print_matrix_zeros(){
    // Create a 3x3 matrix of zeros.
    // Specify just the element type and infer the dimensionality:
    let zeros3 = Array::<f64, _>::zeros((3, 3));

    println!("--------");
    println!("{}", zeros3);
    println!("--------\n");
}

pub fn print_matrix_rand(){
    // create a 2x5 matrix of random numbers from 0 to 10
    let r = Array::random((2, 5), Uniform::new(0., 10.));

    println!("--------");
    println!("{:1.3}", r);
    println!("--------\n");
}

pub fn print_matrix_identity(){
    let i_matrix = Array::<f64, _>::eye(5);
    //let i_matrix_a = Array2::<f64>::eye(5);

    println!("--------");
    println!("{}", i_matrix);
    println!("--------\n");
}

pub fn print_matrix_addition(){
    let ma = array![[1, 2], [3, 4], [50, 60]];

    // Create a 3x2 matrix full of 2
    let mb = Array::from_elem((3, 2), 2);

    println!("--------");
    println!("{}", ma+mb);
    println!("--------\n");
}

pub fn print_matrix_scalar_mul_div(){
    let scalar = 2;
    let ma = array![[10, 2], [30, 4], [50, 6]];

    println!("--------");
    println!("{}", scalar*&ma);
    println!("{}", ma/scalar);
    println!("--------\n");
}

pub fn print_matrix_dot_product(){
    let v = arr1(&[1, 2]);
    //let vv= vec![1, 2];
    let ma = array![[10, 2], [30, 4], [50, 6]];

    println!("--------");
    println!("{}", &ma.dot(&v));
    //println!("{}", &ma.dot(&vv));
    
    // the next is not valid and it will panic
    //println!("{}", &v.dot(&ma));
    println!("--------\n");
}

pub fn print_matrix_transpose(){
    let ma = array![[1, 2, 3], [4, 5, 6]];
 
    println!("---- transpose ----");
    // self.view().reversed_axes()
    println!("{}", &ma.t());                // Returns like a copy of the matrix transpose.
    println!("{}", &ma.reversed_axes());    // move the ownership of the matrix when returned.

    // this will panic because `reversed_axes` moved the ownership 
    // and it was destroyed on the precious line.
    //println!("{}", &ma);
    println!("--------\n");
}

pub fn print_matrix_info(){
    let ma = array![[1, 2, 3], [4, 5, 6]];
 
    println!("--------");
    println!("{:?}", &ma.dim()); // check `shape`
    println!("{:?}", &ma.len());
    println!("{:?}", &ma.ndim());
    println!("--------\n");
}

pub fn print_matrix_index(){
    let v = array![1, 2, 3, 4, 5, 6, 7, 8];
    let m2 = array![[1, 2, 3], [4, 5, 6]];
    let m3 = array![[[1, 2, 20], [3, 4, 40]], [[5, 6, 60], [7, 8, 80]]];
    
    println!("----1D----"); 
    println!("&v[3] {:?}", &v[3]);              // Get the value at pos 3
    println!("{:?}", &v.slice(s![2..5]));       // Get the value from pos 2 to pos 5

    println!("----2D----");
    println!("&m2[[0, 0] {:?}", &m2[[0, 0]]);   // Get the value at pos (0, 0)
    println!("&m2[[1, 2] {:?}", &m2[[1, 2]]);   // Get the value at pos (1, 2)
    println!("{:?}", &m2.slice(s![.., ..2]));   // Get the first two columns of each row

    println!("----3D----");
    println!("&m3[[1, 0, 2] {:?}", &m3[[1, 0, 2]]); // Get the value at pos (1, 0, 2)
    println!("{:?}", &m3.slice(s![1, .., ..]));     
}

pub fn print_matrix_manipulation(){
    println!("---- Matrix Manipulation ----");
    
    let m3 = array![[[1, 2, 20], [3, 4, 40]], [[5, 6, 60], [7, 8, 80]]];
    let mut m_ones = Array::<i32, _>::ones((2, 2, 3));
    println!("m3: {}\n", &m3);
    println!("m_ones: {}\n", &m_ones);

    println!("\nCreate a new vector from a section of a matrix.");
    let v = m3.slice(s![0, 0, ..]);
    println!("v: {}", &v);
    
    println!("\nAssign new vector from a section of another vector.");
    let mut vv = v.slice(s![..2]);
    println!("vv: {}", &vv);

    println!("\nAssign a section of a matrix to a vector.");
    vv = m3.slice(s![1, 0, ..]);
    println!("vv: {}", &vv);

    println!("\nCopy data from a matrix to another matrix");
    m_ones.assign(&m3);
    println!("m_ones: {}\n", &m_ones);

    println!("\nConcatenate two matrices");
    let mm = array![[1, 2, 30], [3, 4, 50]];
    let mmm = arr2(&[[1, 2, 30]]);
    println!(": {:?}", concatenate(Axis(0), &[mm.view(), mmm.view()]));

    println!("\n Stack two arrays along a new axis");
    let vec01 = arr1(&[1, 2, 3]);
    let vec02 = arr1(&[4, 5, 6]);
    println!(": {:?}", stack!(Axis(0), vec01, vec02));
    println!("\n: {:?}", stack!(Axis(1), vec01, vec02));

    println!("--------\n");
}

pub fn print_matrix_iteration(){
    println!("---- Matrix Iteration ----");
    let vec01 = arr1(&[1, 2, 3]);
    let vec02 = arr1(&[4, 5, 6]);
    println!(": {:?}", stack!(Axis(0), vec01, vec02));
    println!(": {:?}", stack!(Axis(1), vec01, vec02));

    let test = stack!(Axis(1), vec01, vec02);
    for i in test.axis_iter(Axis(0)).map(|p| (p[0], p[1])){
        println!("{:?}", i);
    }

}