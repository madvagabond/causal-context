use std::collections::HashMap;

#[derive(Clone)]
pub struct Dot (String, u64);


impl Dot {



    /** Initializes the Dot with version of 1*/
    pub fn init(id: String) -> Self {
	Dot(id, 1)
    }


    /** Initializes dot with version of value*/
    pub fn new(id: String, value: u64) -> Self {
	Dot(id, value) 
    }

    
    pub fn incr(&mut self) {
	self.1 += 1 
    }


    pub fn id(&self) -> &String {
	&self.0
    }


    pub fn version(&self) -> u64 {
	self.1
    }



}



#[derive(Clone)]
pub struct VersionVector(HashMap<String, u64>);


impl VersionVector {

    pub fn get(&self, id: &str) -> u64 {
	match self.0.get(&id.to_string()) {
	    Some(x) => *x,
	    None => 0
	}
    }


    pub fn dot(&self, id: &str) -> Dot {
	let v = self.get(id);
	Dot::new(id.to_string(), v)
    }


    pub fn descends(&self, other: &VersionVector) -> bool {
	for (k, v) in other.0.iter() {
	    if self.get(k) < *v {
		return false
	    }
	}

	return true
    }




    pub fn merge(&mut self, other: &VersionVector) {


	for (k, v) in other.0.iter() {
	    if self.get(k) < *v {
		self.0.insert(k.clone(), *v);
	    }
	}
    }





    /** Finds events in A not present in B */
    pub fn dominating(&self, other: &Self) -> VersionVector {
	
	let state = self.0.iter()
	    .filter(|(k, v)| **v > other.get(k) )
	    .map(|(k, v)| (k.clone(), v.clone())  ) 
	    .collect();

	
	VersionVector(state)
    } 




    pub fn dots(&self) -> impl Iterator<Item=Dot> + '_ {

	self.0.iter().map(|(k, v)| Dot::new(k.clone(), *v))
    }




    
    
}
    
