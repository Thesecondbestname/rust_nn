use crate::{matrix::Matrix, LAYERS};

// Plan: have a base case that matches on the tail with one element
// Have a marker case that matches on the marker, element, tail and the carried element.
macro_rules! _generate_biases{
    (@acc [$($acc:tt)*]) => {
        [ $($acc)* ]
    };
    (@acc [$($acc:tt)*] $element:expr, $($tail:expr),* ) => {
        _generate_biases!(marker @acc[Matrix<$element2, 1>,], $($tail)*)
    };
    (marker @acc [$($acc:tt)*] $element1:expr, $($tail:expr),*) => {
        _generate_biases!(@acc[Matrix<$element2, 1>,], $($tail)*)
    };
}
macro_rules! generate_biases {
    ($($tail:expr),*) => {
        _generate_biases!(@acc []  $($tail,)*)
    }
}
macro_rules! generate_NeuralNetwork {
    ($($element:expr),*)  => {
        struct NeuralNetwork<const data_size: usize>
        {
            weights: [generate_biases!($($element),*); 0],
            biases: [generate_biases!($($element),*); 100],
            data: [Matrix<500, 1>; data_size],
        }

        impl NeuralNetwork<0>
        {
        fn new() -> Self {
                Self {
                    data: [],
                    biases: [
                        // $(
                        //     Matrix::<{ $layers[i + 1] }, 1>::zeros()
                        // ),*
                    ],
                    weights: [
                        // $(
                        //     Matrix::<{ $layers[i + 1] }, { $layers[i] }>::zeros()
                        // ),*
                    ],
                }
            }
        }

    }
}
generate_NeuralNetwork![3, 5, 6, 2];
