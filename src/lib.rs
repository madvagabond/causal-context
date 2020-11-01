pub mod dvv;


#[cfg(test)]
mod tests {

    use crate::dvv::*;

    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }



    #[test]
    fn test_merge() {

	let mut a = VersionVector::new();
	let mut b = VersionVector::new();

	b.incr("c");
	b.incr("d");

	a.merge(&b);


	assert_eq!(a, b)
    }





    #[test]
    fn test_descends_and_concurrent() {
	let mut a = VersionVector::new();
	let mut b = VersionVector::new();

	b.incr("c");
	b.incr("d");

	a.incr("a");
	a.incr("b");


	assert!(a.concurrent(&b));
	assert!(b.concurrent(&a)); 
	
	let mut c = a.clone();
	c.merge(&a);
	assert!(c.descends(&a));

	c.merge(&b);
	
	assert!(c.descends(&b))	
	
    }
}
