# Overall design
The implementation of this project was made in Rust, this decision was made because
we wanted to get more experience with the language and its features. However, we
recognize that Rust is not the best election for this type of project, because it is not designed
for object-oriented programming, and the requirements of the project are more easily implemented
with a language like Java or C#.

Even though Rust is not the apropiate language for this type projects, we implemented all the requirements 
except the ones that needs polymorphism, because Rust does not have a good support for this feature. Moreover, the team
is sure that this project helped us to improve our skills with Rust, understanding better the patterns and how to implement
them in a different coding language.

## Polymorphism in Rust
Level 3 requires us to implement polymorphism with the item classes, allowing the code to extend the Item class depending on the type of item. 
However, since we decided to implement the project in Rust, this requirement makes the code less readable and more difficult to understand.

This is because, in order to implement polymorphism, dynamic dispatch should be used with traits and the 
signature of the _GildedRose_ struct must be changed to the following
```rust
pub struct GildedRose {
    pub items: Vec<Box<dyn ItemTrait>>,
}
```
Where `ItemTrait` is a Trait that defines the behaviour of an item, and has accesor methods to get the properties
```rust
pub trait ItemTrait{
    fn get_quality(&self) -> i32;
    fn update_quality(&mut self);
    fn get_sell_in(&self) -> i32;
    fn get_name(&self) -> String;
}
```
This would require all the existing code that uses the items in `GildedRose` to change to use the methods in the trait, and 
all functions definitions to change to return a `Box` with the trait object. 

For these reasons, we omitted this implementation and instead continued with the strategy pattern, which better fits Rust's programming style.