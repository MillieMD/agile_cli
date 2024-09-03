use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Board {

    name : String,
    columns : Vec<Column>
}

trait Node {

    fn is_leaf(&self) -> bool;
}

impl Board {

    pub fn new(given_name : &str) -> Self {

        let mut b = Self {

            name : given_name.to_owned(),
            columns : vec![]
        };

        b.add_col("Not Started");
        b.add_col("Started");
        b.add_col("Finished");

        return b;
    }

    fn add_col(&mut self, name : &str){

        self.columns.push(Column::new(name));
    }

    pub fn name(&self) -> &String {
        return &self.name;
    }

}

impl Node for Board {

    fn is_leaf(&self) -> bool { return false; }

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Column {

    name : String,

    cards : Vec<Card>
}

impl Column {

    pub fn new(given_name : &str) -> Self {

        Self{

            name : given_name.to_owned(),
            cards : vec![]
        }
    }

    fn add_card(&mut self, card : Card){

        if !self.is_leaf(){

            self.cards.push(card);
        }
    }

}

impl Node for Column {

    fn is_leaf(&self) -> bool{

        return true;
    }

}


#[derive(Serialize, Deserialize, Debug)]
struct Card{

    name : String,
}

impl Card {

    pub fn new(given_name : &str) -> Self {

        Self {
            name : given_name.to_owned()
        }
    }

    pub fn set_name(&mut self, new_name : &str){

        self.name = new_name.to_owned();
    }

    pub fn name(&self) -> &String { return &self.name; }


}