//This is stately, a Rust mod library
//The purpose is to create a up/down and lateral
//state engine for Reactjs use.

pub mod stately {
    //use super::*;

    //declare a struct for Integer variables
    // we need dead_code allowed until we have a function
    // that reads value (seems that has occurred, but Rust disagrees)
    // Debug and clone are needed
    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    pub struct Integer {
        id: String,
        value: i32
    }

    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    pub struct Float {
        id: String,
        value: f64
    }

    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    pub struct StringVar {
        id: String,
        value: String
    }

    //Proper way
    //Create a struct of struct vectors
    //we need to derive Debug and Clone for 
    //fancy printing and copying
    #[derive(Debug)]
    pub struct StateArray {
        integers: Vec<Integer>,
        floats: Vec<Float>,
        string_vars: Vec<StringVar>
    }

    //IMPlement methods for the vector???
    //The methods are now "owned" by IntegerArray
    //By impl -ing methods to structs or vectors
    //we create the appearance of a class in C++
    impl StateArray {
        
        //  self is always a parameter
        // self is the IntegerArray struct (a struct of a vector)
        // our named parameters are a String and an integer
        // to be pushed into self.integers
        pub fn add_integer(&mut self, id: String, value: i32){
            //initialize an instance of Integer
            //with our parameters
            //Could leave them off here, but won't
            let e1 = Integer {
                id: id,
                value: value
            };
            //Use vector push to add struct to vector of structs IntegerArray
            self.integers.push(e1);
        }

        pub fn add_float(&mut self, id: String, value: f64){
            //initialize an instance of Float
            //with our parameters
            //Could leave them off here, but won't
            let e1 = Float {
                id: id,
                value: value
            };
            //Use vector push to add struct to vector of structs IntegerArray
            self.floats.push(e1);
        }

        pub fn add_string(&mut self, id: String, value: String){
            //initialize an instance of Float
            //with our parameters
            //Could leave them off here, but won't
            let e1 = StringVar {
                id: id,
                value: value
            };
            //Use vector push to add struct to vector of structs IntegerArray
            self.string_vars.push(e1);
        }

        // self is always a parameter of impl -ied methods
        // we use &mut to be safe.  Seems to work
        // Our ony named parameter is a String passed in
        // We will return in struct of type Integer (above)
       pub fn find_integer(&mut self, id: String) -> Integer{

            //To find the Integer struct we are looking for:
            //we will iter() over self.integers
            //        find the object |s| from self.integers
            //        such that
            //        if Integer s.id is equal to parameter id
            //        then return the enum Some(Integer) or
            //        if not found, return enum None (end of iter() )
            let ret_val = self.integers.iter().find(|s| s.id == id);

            //Use a match Option<> to determine what to return
            match ret_val {
                //If we found no match,
                //Return a struct with id of Not found
                //Can be read by caller
                None => return Integer {
                    id: "Not found".to_string(),
                    value: -1
                },

                //If we found a match, return a clone()
                //without clone it tries to return a &Integer
                Some(ret_val) => return ret_val.clone()
            }
        
        }

        pub fn find_float(&mut self, id: String) -> Float{

        
            let ret_val = self.floats.iter().find(|s| s.id == id);

            //Use a match Option<> to determine what to return
            match ret_val {
                //If we found no match,
                //Return a struct with id of Not found
                //Can be read by caller
                None => return Float {
                    id: "Not found".to_string(),
                    value: -1.0
                },

                //If we found a match, return a clone()
                //without clone it tries to return a &Integer
                Some(ret_val) => return ret_val.clone()
            }
        }

        pub fn find_stringvar(&mut self, id: String) -> StringVar{

        
            let ret_val = self.string_vars.iter().find(|s| s.id == id);

            //Use a match Option<> to determine what to return
            match ret_val {
                //If we found no match,
                //Return a struct with id of Not found
                //Can be read by caller
                None => return StringVar {
                    id: "Not found".to_string(),
                    value: "not found".to_string()
                },

                //If we found a match, return a clone()
                //without clone it tries to return a &Integer
                Some(ret_val) => return ret_val.clone()
            }
        }


    }
        

    //This goes as a standalone function that uses StateArray
    //It isn't a method, just a function
    pub fn config() -> StateArray {

        let mut c = StateArray { integers: vec![],
            floats: vec![],
            string_vars: vec![]
        };

        let int = Integer{
            id: "first_element".to_string(),
            value: -1
        };

    let float = Float{
        id: "first_element".to_string(),
        value: -1.0
    };

    let mystring = StringVar{
        id: "first_element".to_string(),
        value: "first".to_string()
    };

    c.integers.push(int);
    c.floats.push(float);
    c.string_vars.push(mystring);

    return c;

    }        

}
