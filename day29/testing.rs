// Unit Test
//Function to test
fn add(a: i32, b: i32) -> i32 {
	a + b
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_add() {
		assert_eq!((2,3), 5);
	}
}

// Integration Test
use my_crate::add;

#[test]
fn integration_test_add() {
	assert_eq!(add(10, 20), 30);
}

// Doc test

/// Adds two numbers together.
///
/// # Examples
///
/// '''
/// let result = my_crate::add(2, 3);
/// assert_eq!(result, 5);
///'''
pub fn add(a: i32, b: i32) -> i32 {
	a + b
}